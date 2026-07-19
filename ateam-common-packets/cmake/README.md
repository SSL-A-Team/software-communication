# ROS2 .msg Generation from Proto

`Ros2MsgGen.cmake` wraps `protoc_gen_ros2msg.py`, a protoc plugin that converts
`.proto` definitions into ROS2 `.msg` files at CMake configure time.

## Prerequisites

| Tool | Nix attribute | Notes |
|------|--------------|-------|
| `protoc` | `pkgs.protobuf` | 3.x or later |
| Python 3.11+ | — | with `google.protobuf` pip package |
| `ateam_options_pb2.py` | checked in at `cmake/` | see [Custom options](#bitmask-custom-field-option) |

The nix dev shell (`flake.nix`) provides all of these.

## Quick start

```cmake
# In your CMakeLists.txt:
include(path/to/ateam-common-packets/cmake/Ros2MsgGen.cmake)

generate_ros2_msgs(
  PROTO_FILES
    ${PROTO_DIR}/motor.proto
    ${PROTO_DIR}/telemetry.proto
  PROTO_PATHS
    ${PROTO_DIR}
  OUTPUT_DIR
    ${CMAKE_CURRENT_BINARY_DIR}/msg
)

rosidl_generate_interfaces(${PROJECT_NAME}
  ${GENERATED_ROS2_MSGS}
)
```

`GENERATED_ROS2_MSGS` is set in the calling scope after `generate_ros2_msgs()` returns.

## Function reference: `generate_ros2_msgs`

```
generate_ros2_msgs(
  PROTO_FILES    <file> ...       # required; absolute paths preferred
  PROTO_PATHS    <dir> ...        # --proto_path roots passed to protoc
  OUTPUT_DIR     <dir>            # default: ${CMAKE_CURRENT_BINARY_DIR}/ros2_msgs
  OPTIONAL_SUBMSG  HAS_FIELD|ERROR  # default: HAS_FIELD
)
```

### `PROTO_FILES`
Absolute paths to the `.proto` files to generate `.msg` files for. Imported
files (e.g. `ateam_options.proto`, `google/protobuf/descriptor.proto`) do not
need to be listed here; include their containing directory in `PROTO_PATHS`.

### `PROTO_PATHS`
One or more directories passed as `--proto_path` to protoc. At minimum this
must contain the directory holding any files listed in `PROTO_FILES` and any
directories needed to resolve imports. The `ateam-common-packets/proto/`
directory must always be included when using the `[(ateam.bitmask)]` option
(it holds `ateam_options.proto`).

### `OUTPUT_DIR`
Destination for generated `.msg` files. Defaults to
`${CMAKE_CURRENT_BINARY_DIR}/ros2_msgs`. Created automatically.

### `OPTIONAL_SUBMSG`

#### Why this option exists

Proto3 has no wire concept of "field not present" for message-type fields. A
submessage field is either absent from the wire (equivalent to the default —
an empty message) or present with its encoded fields. At the application level
this means a field like `PowerTelemetry power_status = 3` could be absent
because power data is unavailable, or present but legitimately all-zero — and
those two cases are **indistinguishable** once decoded.

ROS2 `.msg` structs have no optional mechanism: every field is always present,
zero-initialized. When the plugin flattens a proto message with a submessage
field into a `.msg`, it must decide how to carry proto3's implicit presence
information across to ROS2. `OPTIONAL_SUBMSG` controls that decision.

| Value | Behaviour |
|-------|-----------|
| `HAS_FIELD` (default) | Emits `bool has_<field>` immediately before each such field |
| `ERROR` | Fails the configure step with an error listing every offending field |

#### `HAS_FIELD` — permissive, with explicit sentinel

A `bool has_<field>` is prepended to every non-`oneof` message-type field.
Consumers set or check this bool to signal presence.

Given the proto field:
```proto
PowerTelemetry power_status = 3;
```

Generated `.msg` output:
```
bool has_power_status
PowerTelemetry power_status
```

Use `HAS_FIELD` when:
- Generating from existing protos you do not own or cannot restructure.
- Any submessage field is genuinely optional and you need presence tracking.
- You want the build to succeed and handle presence manually in your ROS2 code.

#### `ERROR` — strict, forces explicit schema design

The build fails at CMake configure time with an error for each offending field.
The only way to resolve the error is to either move the field into a `oneof`
or switch back to `HAS_FIELD`.

Use `ERROR` when:
- Writing new protos where you control the schema and want to enforce that
  all optionality is explicit.
- You want the generated `.msg` to have no hidden sentinel bools — every field
  has a clear meaning without a companion `has_` flag.

#### Using `oneof` to satisfy `ERROR` mode

`oneof` fields always carry a discriminant (`<name>_case`) and work in both
modes. Wrapping an optional submessage in a `oneof` is the idiomatic proto3
way to express "this field may or may not be present":

```proto
// Before: bare submessage field — rejected in ERROR mode
PowerTelemetry power_status = 3;

// After: oneof wrapping — accepted in both modes
oneof power {
    PowerTelemetry power_status = 3;
}
```

Generated `.msg` output for the `oneof` form:
```
# oneof power
uint8 ONEOF_POWER_NONE=0
uint8 ONEOF_POWER_POWER_STATUS=1
uint8 power_case
PowerTelemetry power_status
```

Consumers read `power_case` to check presence (`ONEOF_POWER_NONE` means
absent) before accessing `power_status`. This is more explicit than a loose
`bool has_power_status` and is fully self-describing in the `.msg` file.

## Type mappings

### Scalar fields

| Proto type | ROS2 type |
|-----------|-----------|
| `double` | `float64` |
| `float` | `float32` |
| `int32`, `sint32`, `sfixed32` | `int32` |
| `int64`, `sint64`, `sfixed64` | `int64` |
| `uint32`, `fixed32` | `uint32` |
| `uint64`, `fixed64` | `uint64` |
| `bool` | `bool` |
| `string` | `string` |
| `bytes` | `uint8[]` |

### Enum fields

Proto enums map to `int32` fields in the `.msg`. ROS2 has no native enum type.
The enum's constants are available as a separate `<EnumName>.msg` file
(constants-only message) generated alongside the main message.

### Repeated fields

`repeated T field` becomes `T[] field` (ROS2 dynamic array). Fixed-size
arrays are not supported; all repeated fields produce dynamic arrays.

### Message fields (submessages)

Non-`oneof` message-type fields get a `bool has_<field>` sentinel prepended
(with `HAS_FIELD` mode) to carry proto3's implicit presence information across
to ROS2. The referenced type name is used verbatim — the type must be generated
in the same `rosidl_generate_interfaces()` call.

### `oneof` fields

A `oneof foo { A a = 1; B b = 2; }` expands to:

```
# oneof foo
uint8 ONEOF_FOO_NONE=0
uint8 ONEOF_FOO_A=1
uint8 ONEOF_FOO_B=2
uint8 foo_case
A a
B b
```

`foo_case` carries the discriminant. All arm fields are always present in the
`.msg` struct regardless of which arm is set — ROS2 has no union type.
Consumers must check `foo_case` before using an arm.

## `[(ateam.bitmask)]` custom field option

`uint32` fields annotated with `[(ateam.bitmask).flags_enum = "EnumName"]`
receive special treatment:

1. The referenced enum's maximum value is inspected to determine the smallest
   unsigned type that fits: `uint8` (max < 256), `uint16` (max < 65536),
   or `uint32` otherwise.
2. All enum constants are emitted inline above the field as typed constants.
3. The field itself is emitted with the inferred type instead of `uint32`.

Example output for `CcmTelemetry.error_flags` (16 flag bits, max = 32768):

```
# bitmask: CcmErrorFlag
uint16 CCM_ERR_NONE=0
uint16 CCM_ERR_MASTER_ERROR=1
...
uint16 CCM_ERR_RESET_PIN=32768
uint16 error_flags
```

**Constraints:**
- The annotated field must be `uint32` in the proto source. Any other type
  causes a build error.
- The enum name in `flags_enum` must resolve to an enum visible to protoc in
  the current set of compiled files. An unknown name causes a build error.
- Import `ateam_options.proto` in any `.proto` file that uses this annotation.

The annotation has **no wire overhead** — it is compile-time metadata only and
does not alter the proto encoding.

## Re-generation and incremental builds

Generation runs at CMake configure time. CMake automatically re-configures
when any file listed in `PROTO_FILES` changes (`CMAKE_CONFIGURE_DEPENDS`).
Changes to **imported** proto files (files in `PROTO_PATHS` but not in
`PROTO_FILES`) do not trigger re-configuration. If you modify an imported
file, run `cmake .` manually or touch one of the listed `PROTO_FILES`.

## Limitations

- **No nested messages or enums.** Proto allows defining a message or enum
  inside another message (`message Outer { message Inner { ... } }`). The
  plugin only iterates `fd.message_type` and `fd.enum_type`, which are
  file-scope (top-level) definitions only. This causes three distinct failures:

  1. **Nested message definition:** no `.msg` is generated for the inner type.
     `rosidl_generate_interfaces` then fails at ROS2 build time because the
     field that references it names a `.msg` that does not exist.
  2. **Field referencing a nested type:** `strip_package` takes the last
     `.`-delimited component of the fully-qualified type name
     (`.ateam.Outer.Inner` → `Inner`). The type name in the `.msg` is
     correct, but because no `Inner.msg` was generated (see above), the ROS2
     build still fails.
  3. **Nested enum in a `[(ateam.bitmask)]` annotation:** the `all_enums`
     lookup only contains file-scope enums. A bitmask referencing a nested
     enum (`Outer.InnerFlag`) will not be found and the plugin emits a build
     error: `unknown enum 'InnerFlag'`.

  **Workaround:** define all messages and enums at file scope. There is no
  semantic difference in proto3 — nesting is purely a namespace convention,
  and the `package` declaration already controls the proto namespace.
- **No `map<K,V>` fields.** Proto map fields are not handled and will not
  appear in the output.
- **No proto3 `optional` scalar fields.** The `optional` keyword on scalar
  fields is ignored; presence is not tracked for scalars.
- **`oneof` arms are always emitted.** Because ROS2 has no union type, all
  arms coexist in the struct. Memory layout is not compact.
- **Enum type in `.msg` is `int32`.** ROS2 constants are used for named
  access but the field itself is untyped from ROS2's perspective.
- **Single package only.** The plugin strips the package prefix from type
  names (`strip_package`). If you import messages from multiple proto packages
  with colliding short names, the generated `.msg` will have name conflicts.
- **Protoc version.** Tested with protobuf 3.x/4.x. The plugin uses the
  binary `CodeGeneratorRequest`/`CodeGeneratorResponse` protocol; it is
  insensitive to the protobuf Python library version as long as
  `google.protobuf.compiler.plugin_pb2` is available.

---

# C++ fromProto() Conversion Header Generation

`Ros2CppConvertGen.cmake` wraps `protoc_gen_ros2cpp.py`, a protoc plugin that
generates C++ `fromProto()` conversion functions from `.proto` definitions.

For each `.proto` file, it produces a `<stem>_conversions.hpp` header with
`inline` free functions that convert proto C++ types to ROS2 message types:

```cpp
ros2_package::msg::MsgType fromProto(const ateam::MsgType& p);
```

## Quick start

```cmake
include(path/to/ateam-common-packets/cmake/Ros2CppConvertGen.cmake)

generate_ros2_cpp_conversions(
  PROTO_FILES
    ${PROTO_DIR}/motor.proto
    ${PROTO_DIR}/telemetry.proto
  PROTO_PATHS
    ${PROTO_DIR}
  PROTO_INCLUDE_PREFIX  ateam_common_packets
  ROS2_PACKAGE          ateam_radio_msgs
  OUTPUT_DIR            ${CMAKE_CURRENT_BINARY_DIR}/include/conversions
  NAMESPACE             ateam_conversions
)

target_include_directories(my_target PUBLIC
  ${CMAKE_CURRENT_BINARY_DIR}/include/conversions
)
```

Then include and call:

```cpp
#include "motor_conversions.hpp"

ateam::CcmTelemetry proto_msg = ...; // decoded from wire
auto ros_msg = ateam_conversions::fromProto(proto_msg);
```

`GENERATED_ROS2_CPP_HEADERS` is set in the calling scope after the call returns.

## Function reference: `generate_ros2_cpp_conversions`

```
generate_ros2_cpp_conversions(
  PROTO_FILES          <file> ...        # required; absolute paths preferred
  PROTO_PATHS          <dir> ...         # --proto_path roots passed to protoc
  PROTO_INCLUDE_PREFIX <prefix>          # required; prefix in #include <prefix/stem.pb.h>
  ROS2_PACKAGE         <package>         # required; ROS2 package name
  OUTPUT_DIR           <dir>             # default: ${CMAKE_CURRENT_BINARY_DIR}/ros2_cpp_conversions
  NAMESPACE            <cpp_namespace>   # default: ateam_conversions
  OPTIONAL_SUBMSG      HAS_FIELD|ERROR   # default: HAS_FIELD
)
```

### `PROTO_INCLUDE_PREFIX`

The directory component in the proto-generated C++ include path:

```cpp
#include <ateam_common_packets/motor.pb.h>
//       ^^^^^^^^^^^^^^^^^^^^^ PROTO_INCLUDE_PREFIX
```

This must match however `protoc --cpp_out` places its generated `.pb.h` files
in your build system's include tree.

### `ROS2_PACKAGE`

The ROS2 package that provides the generated `.msg` C++ headers. Used for
include paths (`<ros2_package/msg/msg_name.hpp>`) and type names
(`ros2_package::msg::MsgName`).

### `NAMESPACE`

C++ namespace wrapping all generated `fromProto()` functions. Defaults to
`ateam_conversions`. All functions across all generated headers share the same
namespace, so proto imports between files resolve without qualification.

### `OPTIONAL_SUBMSG`

Same semantics as in `generate_ros2_msgs()` — see [OPTIONAL_SUBMSG](#optional_submsg)
above. Controls handling of non-`oneof` message-type fields:

| Value | Behaviour |
|-------|-----------|
| `HAS_FIELD` (default) | Emits `msg.has_<field> = p.has_<field>()` and a guarded `fromProto` call |
| `ERROR` | Fails configure if any such field is present |

## Type mapping (C++)

| Proto type | C++ conversion |
|-----------|----------------|
| Scalar (int32, float, bool, …) | `msg.f = p.f();` |
| `enum` | `msg.f = static_cast<int32_t>(p.f());` |
| `bytes` | `msg.f = std::vector<uint8_t>(p.f().begin(), p.f().end());` |
| `repeated T` (scalar) | `for (v : p.f()) msg.f.push_back(v);` |
| `repeated T` (message) | `for (v : p.f()) msg.f.push_back(fromProto(v));` |
| `message T` (non-oneof) | `msg.has_f = p.has_f(); if (p.has_f()) msg.f = fromProto(p.f());` |
| `oneof` discriminant | `msg.name_case = static_cast<uint8_t>(p.name_case());` |
| `oneof` message arm | `if (p.has_arm()) msg.arm = fromProto(p.arm());` |
| `oneof` scalar arm | `msg.arm = p.arm();` |
| `[(ateam.bitmask)]` | `msg.f = static_cast<uint{8,16,32}_t>(p.f());` (narrowed) |

## Generated header structure

For `motor.proto` with `PROTO_INCLUDE_PREFIX=ateam_common_packets`,
`ROS2_PACKAGE=ateam_radio_msgs`, `NAMESPACE=ateam_conversions`:

```cpp
#pragma once

#include <ateam_common_packets/motor.pb.h>
#include <ateam_radio_msgs/msg/ccm_telemetry.hpp>
// ... other message includes

#include <cstdint>
#include <vector>

namespace ateam_conversions {

inline ateam_radio_msgs::msg::CcmTelemetry fromProto(const ateam::CcmTelemetry& p) {
  ateam_radio_msgs::msg::CcmTelemetry msg;
  msg.error_flags = static_cast<uint16_t>(p.error_flags());  // bitmask narrowing
  msg.motion_control_type = static_cast<int32_t>(p.motion_control_type());
  msg.gain_stage_index = p.gain_stage_index();
  msg.has_current_telem = p.has_current_telem();
  if (p.has_current_telem()) msg.current_telem = fromProto(p.current_telem());
  // ...
  return msg;
}

}  // namespace ateam_conversions
```

## Import dependencies

If `control.proto` imports `maneuvers.proto`, `control_conversions.hpp` will
automatically include `maneuvers_conversions.hpp` using a relative include.
Generate all conversion headers for a proto graph into the same `OUTPUT_DIR`
so relative includes resolve.

---

## Running tests

From the `ateam-common-packets/` directory:

```sh
python3 -m pytest cmake/tests/test_plugin.py -v        # .msg plugin
python3 -m pytest cmake/tests/test_cpp_plugin.py -v    # C++ plugin
```

Or via the repo Makefile:

```sh
make proto-plugin-test
make proto-cpp-plugin-test
```

Tests cover: scalar/bitmask type inference, `oneof` expansion, error mode
rejection, `optional_submsg` option parsing, and per-proto output correctness
including `diagnostics.proto` and all bitmask-annotated fields.
