#!/usr/bin/env python3
"""
protoc plugin: generates ROS2 .msg files from .proto definitions.

Invoked by protoc as a subprocess; reads CodeGeneratorRequest from stdin,
writes CodeGeneratorResponse to stdout (standard protoc plugin protocol).

Type mappings:
  Proto scalar           → ROS2 primitive
  message Foo            → Foo.msg (separate file, referenced by name)
  enum Foo               → Foo.msg (constants-only message)
  repeated T             → T[] field
  oneof foo              → uint8 foo_case + constants + all arm fields
  enum field             → int32 (ROS2 has no enum type; constants in Foo.msg)
  [(ateam.bitmask)] field → minimal uint type inferred from enum max value,
                            with flag constants emitted inline above the field
  nested message Foo.Bar → Bar.msg with flat name Foo_Bar
  map<K,V> field         → skipped (no ROS2 map type)

Options (via --ros2msg_opt=key=value,key=value):
  optional_submsg=has_field  (default) Emit bool has_<field> before each
                             non-oneof message-type field.
  optional_submsg=error      Reject any non-oneof message-type field as a
                             build error; forces schema authors to be explicit.
"""

import sys
from pathlib import Path
from google.protobuf.compiler import plugin_pb2
from google.protobuf import descriptor_pb2

# Import generated options module so the bitmask extension is registered in
# the descriptor pool and accessible via field.options.Extensions[...].
sys.path.insert(0, str(Path(__file__).parent))
import ateam_options_pb2  # noqa: E402
from ateam_proto_shared import (  # noqa: E402
    parse_options,
    flatten_type_name,
    build_map_entry_type_names,
    iter_messages,
)

# Public alias: tests import plugin.strip_package; keep it in this module's namespace.
strip_package = flatten_type_name

FD = descriptor_pb2.FieldDescriptorProto

SCALAR_TYPE_MAP = {
    FD.TYPE_DOUBLE:   "float64",
    FD.TYPE_FLOAT:    "float32",
    FD.TYPE_INT64:    "int64",
    FD.TYPE_UINT64:   "uint64",
    FD.TYPE_INT32:    "int32",
    FD.TYPE_FIXED64:  "uint64",
    FD.TYPE_FIXED32:  "uint32",
    FD.TYPE_BOOL:     "bool",
    FD.TYPE_STRING:   "string",
    FD.TYPE_BYTES:    "uint8[]",
    FD.TYPE_UINT32:   "uint32",
    FD.TYPE_SINT32:   "int32",
    FD.TYPE_SINT64:   "int64",
    FD.TYPE_SFIXED32: "int32",
    FD.TYPE_SFIXED64: "int64",
}


def ros2_field_type(field: descriptor_pb2.FieldDescriptorProto) -> str:
    if field.type in SCALAR_TYPE_MAP:
        return SCALAR_TYPE_MAP[field.type]
    if field.type == FD.TYPE_ENUM:
        return "int32"
    if field.type == FD.TYPE_MESSAGE:
        return flatten_type_name(field.type_name)
    raise ValueError(f"unhandled proto field type {field.type} in field '{field.name}'")


def minimal_uint_type(max_value: int) -> str:
    """Return the smallest unsigned ROS2 integer type that fits max_value."""
    if max_value < 256:
        return "uint8"
    if max_value < 65536:
        return "uint16"
    return "uint32"


def iter_enums(fd):
    """Yield (flat_name, enum) for all enums in fd, including those nested in messages."""
    for enum in fd.enum_type:
        yield enum.name, enum

    def _walk(msg, parent_flat: str):
        flat = f"{parent_flat}_{msg.name}" if parent_flat else msg.name
        for enum in msg.enum_type:
            yield f"{flat}_{enum.name}", enum
        for nested in msg.nested_type:
            yield from _walk(nested, flat)

    for msg in fd.message_type:
        yield from _walk(msg, "")


# --------------------------------------------------------------------------- #
# .msg generation
# --------------------------------------------------------------------------- #

def generate_enum_msg(enum: descriptor_pb2.EnumDescriptorProto) -> str:
    lines = [f"# Generated from proto enum {enum.name}"]
    for v in enum.value:
        lines.append(f"int32 {v.name}={v.number}")
    return "\n".join(lines) + "\n"


def generate_message_msg(
    msg: descriptor_pb2.DescriptorProto,
    flat_name: str,
    optional_submsg: str,
    all_enums: dict,
    errors: list,
    map_entry_type_names: frozenset,
) -> str:
    lines = [f"# Generated from proto message {flat_name}"]

    emitted_oneofs: set = set()

    for field in msg.field:
        # Skip map<K,V> fields: they reference a synthetic nested map-entry message
        # and have no equivalent ROS2 type.
        if field.type == FD.TYPE_MESSAGE and field.type_name in map_entry_type_names:
            continue

        is_repeated = field.label == FD.LABEL_REPEATED
        in_oneof = field.HasField("oneof_index")

        if in_oneof:
            oi = field.oneof_index
            if oi in emitted_oneofs:
                continue
            emitted_oneofs.add(oi)

            oneof_name = msg.oneof_decl[oi].name
            oneof_fields = [
                f for f in msg.field
                if f.HasField("oneof_index") and f.oneof_index == oi
            ]

            lines.append("")
            lines.append(f"# oneof {oneof_name}")
            lines.append(f"# case constants use proto field numbers (stable across reordering)")
            lines.append(f"uint8 ONEOF_{oneof_name.upper()}_NONE=0")
            for of in oneof_fields:
                if of.number > 255:
                    errors.append(
                        f"{flat_name}: oneof '{oneof_name}' field '{of.name}' has field "
                        f"number {of.number} which exceeds the uint8 range (max 255) used "
                        f"for the case discriminant. Use field numbers ≤ 255 in oneof "
                        f"declarations, or file a request to widen the discriminant type."
                    )
                    continue
                const = f"ONEOF_{oneof_name.upper()}_{of.name.upper()}"
                lines.append(f"uint8 {const}={of.number}")
            lines.append(f"uint8 {oneof_name}_case")
            for of in oneof_fields:
                lines.append(f"{ros2_field_type(of)} {of.name}")
            continue

        # Check for [(ateam.bitmask)] annotation.
        if field.options.HasExtension(ateam_options_pb2.bitmask):
            bitmask_opts = field.options.Extensions[ateam_options_pb2.bitmask]
            enum_name = bitmask_opts.flags_enum

            if field.type != FD.TYPE_UINT32:
                errors.append(
                    f"{flat_name}.{field.name}: [(ateam.bitmask)] requires uint32, "
                    f"got type {field.type}"
                )
                continue

            if enum_name not in all_enums:
                errors.append(
                    f"{flat_name}.{field.name}: [(ateam.bitmask)] references unknown "
                    f"enum '{enum_name}'"
                )
                continue

            enum = all_enums[enum_name]
            max_val = max(v.number for v in enum.value)
            uint_type = minimal_uint_type(max_val)

            lines.append(f"")
            lines.append(f"# bitmask: {enum_name}")
            for v in enum.value:
                lines.append(f"{uint_type} {v.name}={v.number}")
            lines.append(f"{uint_type} {field.name}")
            continue

        if field.type == FD.TYPE_MESSAGE:
            if optional_submsg == "error":
                errors.append(
                    f"{flat_name}.{field.name}: non-oneof message-type field has "
                    f"implicit proto3 presence — set optional_submsg=has_field to "
                    f"auto-generate a bool presence flag, or move into a oneof."
                )
                continue
            lines.append(f"bool has_{field.name}")

        ros2_type = ros2_field_type(field)
        if is_repeated:
            lines.append(f"{ros2_type}[] {field.name}")
        else:
            lines.append(f"{ros2_type} {field.name}")

    return "\n".join(lines) + "\n"


def main() -> None:
    data = sys.stdin.buffer.read()
    request = plugin_pb2.CodeGeneratorRequest()
    request.ParseFromString(data)

    response = plugin_pb2.CodeGeneratorResponse()
    response.supported_features = (
        plugin_pb2.CodeGeneratorResponse.FEATURE_PROTO3_OPTIONAL
    )

    opts = parse_options(request.parameter)
    optional_submsg = opts.get("optional_submsg", "has_field")
    if optional_submsg not in ("has_field", "error"):
        response.error = (
            f"Unknown optional_submsg={optional_submsg!r}. "
            f"Valid values: 'has_field', 'error'."
        )
        sys.stdout.buffer.write(response.SerializeToString())
        return

    # Build enum lookup across all files (file-scope enums only; bitmask
    # annotations reference enums by short name and are only used in ateam
    # protos where all enums are at file scope).
    all_enums: dict = {}
    for fd in request.proto_file:
        for enum in fd.enum_type:
            all_enums[enum.name] = enum

    map_entry_type_names = build_map_entry_type_names(request)
    all_files = {f.name: f for f in request.proto_file}
    errors: list = []

    for file_name in request.file_to_generate:
        fd = all_files[file_name]

        for flat_name, enum in iter_enums(fd):
            out = response.file.add()
            out.name = f"{flat_name}.msg"
            out.content = generate_enum_msg(enum)

        for flat_name, msg in iter_messages(fd):
            out = response.file.add()
            out.name = f"{flat_name}.msg"
            out.content = generate_message_msg(
                msg, flat_name, optional_submsg, all_enums, errors, map_entry_type_names
            )

    if errors:
        response.error = "\n".join(errors)

    sys.stdout.buffer.write(response.SerializeToString())


if __name__ == "__main__":
    main()
