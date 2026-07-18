#!/usr/bin/env python3
"""
protoc plugin: generates C++ fromProto() conversion headers from .proto definitions.

For each .proto file in the request, generates <stem>_conversions.hpp containing
inline fromProto(const pkg::MsgType&) → ros2_pkg::msg::MsgType functions.

Invoked by protoc as a subprocess; reads CodeGeneratorRequest from stdin,
writes CodeGeneratorResponse to stdout (standard protoc plugin protocol).

Nested message types are flattened: a message Bar nested inside Foo generates
FooBar_conversions.hpp entries and is referenced as Foo_Bar in the C++ output,
matching the proto C++ generated naming convention (Foo_Bar is the actual class).

map<K,V> fields are silently skipped (no ROS2 equivalent).

Options (via --ros2cpp_opt=key=value,...):
  proto_include_prefix=<str>   required; directory in #include <prefix/stem.pb.h>
  ros2_package=<str>           required; ROS2 package, e.g. 'ateam_radio_msgs'
  namespace=<str>              C++ namespace for generated functions (default: ateam_conversions)
  optional_submsg=has_field    (default) emit presence check for non-oneof message fields
  optional_submsg=error        fail if any non-oneof message-type field is present
"""

import re
import sys
from pathlib import Path
from google.protobuf.compiler import plugin_pb2
from google.protobuf import descriptor_pb2

sys.path.insert(0, str(Path(__file__).parent))
import ateam_options_pb2  # noqa: E402
from ateam_proto_shared import (  # noqa: E402
    parse_options,
    flatten_type_name,
    build_map_entry_type_names,
    iter_messages,
)

# Public alias: tests import cpp_plugin.strip_package; keep it in this module's namespace.
strip_package = flatten_type_name

FD = descriptor_pb2.FieldDescriptorProto

_SCALAR_TYPE_MAP = {
    FD.TYPE_DOUBLE:   "double",
    FD.TYPE_FLOAT:    "float",
    FD.TYPE_INT64:    "int64_t",
    FD.TYPE_UINT64:   "uint64_t",
    FD.TYPE_INT32:    "int32_t",
    FD.TYPE_FIXED64:  "uint64_t",
    FD.TYPE_FIXED32:  "uint32_t",
    FD.TYPE_BOOL:     "bool",
    FD.TYPE_STRING:   "std::string",
    FD.TYPE_UINT32:   "uint32_t",
    FD.TYPE_SINT32:   "int32_t",
    FD.TYPE_SINT64:   "int64_t",
    FD.TYPE_SFIXED32: "int32_t",
    FD.TYPE_SFIXED64: "int64_t",
}

_SKIP_DEPS = frozenset({"ateam_options", "descriptor"})


def to_snake_case(name: str) -> str:
    """CamelCase or Flat_Name → snake_case  (e.g. CcmTelemetry → ccm_telemetry)"""
    return re.sub(r"(?<=[a-z0-9])(?=[A-Z])", "_", name).lower()


def minimal_uint_ctype(max_value: int) -> str:
    if max_value < 256:
        return "uint8_t"
    if max_value < 65536:
        return "uint16_t"
    return "uint32_t"


# --------------------------------------------------------------------------- #
# C++ generation helpers
# --------------------------------------------------------------------------- #

def _generate_scalar_assign(fname: str, field_type: int) -> str:
    if field_type == FD.TYPE_BYTES:
        return (
            f"  msg.{fname} = "
            f"std::vector<uint8_t>(p.{fname}().begin(), p.{fname}().end());"
        )
    if field_type == FD.TYPE_ENUM:
        return f"  msg.{fname} = static_cast<int32_t>(p.{fname}());"
    return f"  msg.{fname} = p.{fname}();"


def _generate_repeated(field) -> str:
    fname = field.name
    if field.type == FD.TYPE_MESSAGE:
        return f"  for (const auto& v : p.{fname}()) msg.{fname}.push_back(fromProto(v));"
    if field.type == FD.TYPE_ENUM:
        return f"  for (const auto& v : p.{fname}()) msg.{fname}.push_back(static_cast<int32_t>(v));"
    if field.type == FD.TYPE_BYTES:
        return (
            f"  for (const auto& v : p.{fname}())"
            f" msg.{fname}.push_back(std::vector<uint8_t>(v.begin(), v.end()));"
        )
    return f"  for (const auto& v : p.{fname}()) msg.{fname}.push_back(v);"


def generate_field_lines(
    field,
    flat_name: str,
    optional_submsg: str,
    all_enums: dict,
    errors: list,
    map_entry_type_names: frozenset,
) -> list[str]:
    fname = field.name
    is_repeated = field.label == FD.LABEL_REPEATED

    # Skip map<K,V> fields
    if field.type == FD.TYPE_MESSAGE and field.type_name in map_entry_type_names:
        return []

    # [(ateam.bitmask)] annotation
    if field.options.HasExtension(ateam_options_pb2.bitmask):
        bitmask_opts = field.options.Extensions[ateam_options_pb2.bitmask]
        enum_name = bitmask_opts.flags_enum
        if field.type != FD.TYPE_UINT32:
            errors.append(
                f"{flat_name}.{fname}: [(ateam.bitmask)] requires uint32, got type {field.type}"
            )
            return []
        if enum_name not in all_enums:
            errors.append(
                f"{flat_name}.{fname}: [(ateam.bitmask)] references unknown enum '{enum_name}'"
            )
            return []
        max_val = max(v.number for v in all_enums[enum_name].value)
        ctype = minimal_uint_ctype(max_val)
        return [f"  msg.{fname} = static_cast<{ctype}>(p.{fname}());"]

    if is_repeated:
        return [_generate_repeated(field)]

    if field.type == FD.TYPE_MESSAGE:
        if optional_submsg == "error":
            errors.append(
                f"{flat_name}.{fname}: non-oneof message-type field has implicit proto3 "
                f"presence — set optional_submsg=has_field or move into a oneof."
            )
            return []
        return [
            f"  msg.has_{fname} = p.has_{fname}();",
            f"  if (p.has_{fname}()) msg.{fname} = fromProto(p.{fname}());",
        ]

    return [_generate_scalar_assign(fname, field.type)]


def generate_oneof_lines(msg, oneof_index: int) -> list[str]:
    oneof_name = msg.oneof_decl[oneof_index].name
    arms = [f for f in msg.field if f.HasField("oneof_index") and f.oneof_index == oneof_index]

    lines = [f"  msg.{oneof_name}_case = static_cast<uint8_t>(p.{oneof_name}_case());"]
    for arm in arms:
        if arm.type == FD.TYPE_MESSAGE:
            lines.append(f"  if (p.has_{arm.name}()) msg.{arm.name} = fromProto(p.{arm.name}());")
        elif arm.type == FD.TYPE_ENUM:
            lines.append(f"  msg.{arm.name} = static_cast<int32_t>(p.{arm.name}());")
        else:
            lines.append(f"  msg.{arm.name} = p.{arm.name}();")
    return lines


def generate_message_function(
    msg,
    flat_name: str,
    proto_package: str,
    ros2_package: str,
    optional_submsg: str,
    all_enums: dict,
    errors: list,
    map_entry_type_names: frozenset,
) -> list[str]:
    # Proto C++ names nested types with underscores: Foo::Bar → Foo_Bar
    cpp_proto = f"{proto_package}::{flat_name}" if proto_package else flat_name
    cpp_ros2 = f"{ros2_package}::msg::{flat_name}"

    body: list[str] = []
    emitted_oneofs: set = set()
    for field in msg.field:
        # Skip map fields
        if field.type == FD.TYPE_MESSAGE and field.type_name in map_entry_type_names:
            continue
        if field.HasField("oneof_index"):
            oi = field.oneof_index
            if oi in emitted_oneofs:
                continue
            emitted_oneofs.add(oi)
            body.extend(generate_oneof_lines(msg, oi))
        else:
            body.extend(generate_field_lines(
                field, flat_name, optional_submsg, all_enums, errors, map_entry_type_names
            ))

    return [
        f"inline {cpp_ros2} fromProto(const {cpp_proto}& p) {{",
        f"  {cpp_ros2} msg;",
        *body,
        "  return msg;",
        "}",
    ]


def generate_header(
    fd,
    proto_include_prefix: str,
    ros2_package: str,
    namespace: str,
    optional_submsg: str,
    all_enums: dict,
    errors: list,
    map_entry_type_names: frozenset,
) -> str:
    stem = Path(fd.name).stem
    lines: list[str] = ["#pragma once", ""]

    # Proto generated header
    lines.append(f"#include <{proto_include_prefix}/{stem}.pb.h>")

    # ROS2 message headers (one per non-map-entry message in this file)
    for flat_name, _ in iter_messages(fd):
        ros2_inc = to_snake_case(flat_name)
        lines.append(f"#include <{ros2_package}/msg/{ros2_inc}.hpp>")

    # Conversion headers for imported proto files (skip well-known / options)
    dep_includes = [
        f'#include "{Path(dep).stem}_conversions.hpp"'
        for dep in fd.dependency
        if Path(dep).stem not in _SKIP_DEPS and not dep.startswith("google/")
    ]
    if dep_includes:
        lines.append("")
        lines.extend(dep_includes)

    # Only emit namespace block if the file has any (non-map-entry) messages
    all_msgs = list(iter_messages(fd))
    if all_msgs:
        lines += ["", "#include <cstdint>", "#include <vector>"]
        lines += ["", f"namespace {namespace} {{", ""]

        for flat_name, msg in all_msgs:
            lines.extend(
                generate_message_function(
                    msg, flat_name, fd.package, ros2_package,
                    optional_submsg, all_enums, errors, map_entry_type_names,
                )
            )
            lines.append("")

        lines += [f"}}  // namespace {namespace}", ""]

    return "\n".join(lines)


def main() -> None:
    data = sys.stdin.buffer.read()
    request = plugin_pb2.CodeGeneratorRequest()
    request.ParseFromString(data)

    response = plugin_pb2.CodeGeneratorResponse()
    response.supported_features = plugin_pb2.CodeGeneratorResponse.FEATURE_PROTO3_OPTIONAL

    opts = parse_options(request.parameter)

    missing = [k for k in ("proto_include_prefix", "ros2_package") if k not in opts]
    if missing:
        response.error = f"Required options missing: {', '.join(missing)}"
        sys.stdout.buffer.write(response.SerializeToString())
        return

    proto_include_prefix = opts["proto_include_prefix"]
    ros2_package = opts["ros2_package"]
    namespace = opts.get("namespace", "ateam_conversions")
    optional_submsg = opts.get("optional_submsg", "has_field")

    if optional_submsg not in ("has_field", "error"):
        response.error = (
            f"Unknown optional_submsg={optional_submsg!r}. "
            f"Valid values: 'has_field', 'error'."
        )
        sys.stdout.buffer.write(response.SerializeToString())
        return

    all_enums: dict = {}
    for fd in request.proto_file:
        for enum in fd.enum_type:
            all_enums[enum.name] = enum

    map_entry_type_names = build_map_entry_type_names(request)
    all_files = {f.name: f for f in request.proto_file}
    errors: list = []

    for file_name in request.file_to_generate:
        fd = all_files[file_name]
        stem = Path(file_name).stem
        out = response.file.add()
        out.name = f"{stem}_conversions.hpp"
        out.content = generate_header(
            fd, proto_include_prefix, ros2_package, namespace,
            optional_submsg, all_enums, errors, map_entry_type_names,
        )

    if errors:
        response.error = "\n".join(errors)

    sys.stdout.buffer.write(response.SerializeToString())


if __name__ == "__main__":
    main()
