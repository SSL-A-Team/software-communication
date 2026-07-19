# ateam-common-packets

Packet definitions for Robot↔AI radio communication: C headers, Protocol Buffer schemas, ROS2 `.msg` generation, and Rust bindings.

## Directory Structure

```
include/
  common.h                   Shared primitive types (fixed-width ints, assert_size macro)
  robot_metadata.h           Robot identity metadata
  radio/                     Packets exchanged over the radio link
    radio.h                  Top-level packet union (RadioData)
    basic_control.h          AI→Robot motion command
    basic_telemetry.h        Robot→AI status packet
    extended_telemetry.h     Robot→AI full debug telemetry
    body_control.h           Body controller extended telemetry
    discovery.h              Hello request/response
    error_telemetry.h        Robot→AI error report
    robot_parameters.h       Runtime-tunable parameter read/write
    robot_maneuvers/         Per-mode command and telemetry structs
  wire/                      Packets on the robot-internal SPI/UART buses (not sent over radio)
    stspin.h                 Motor controller velocity command
    stspin_current.h         Motor controller current/telemetry packets
    kicker.h                 Kicker board command and telemetry
    power.h                  Power board telemetry

proto/                       Protocol Buffer schemas (proto3)
  radio.proto                Top-level RadioPacket oneof (replaces C CommandCode + RadioData union)
  control.proto              BasicControl message
  telemetry.proto            BasicTelemetry and ExtendedTelemetry messages
  maneuvers.proto            Per-mode command messages and body controller telemetry stubs
  body_control.proto         BodyControlExtendedTelemetry message
  discovery.proto            HelloRequest / HelloResponse messages
  diagnostics.proto          ErrorTelemetry message
  robot_parameters.proto     ParameterCommand message (runtime tuning)
  motor.proto                CcmTelemetry messages
  power.proto                PowerTelemetry messages
  kicker.proto               KickerTelemetry messages
  ateam_options.proto        Custom proto options (bitmask annotation)

cmake/
  Ros2MsgGen.cmake           CMake function: generate ROS2 .msg files from protos at configure time
  Ros2CppConvertGen.cmake    CMake function: generate C++ fromProto() headers from protos
  protoc_gen_ros2msg.py      protoc plugin — .proto → ROS2 .msg
  protoc_gen_ros2cpp.py      protoc plugin — .proto → C++ fromProto() headers
  ateam_proto_shared.py      Shared helpers used by both plugins
  tests/                     pytest suites for both plugins

rust-lib/
  build.rs                   Runs bindgen (C→Rust) and micropb-gen (proto→Rust) at build time
  src/
    lib.rs                   Public API + basic control safety checks
    radio.rs                 Rust types mirroring C radio packet structs
    translation.rs           From<&c::T> for proto::T conversion impls
    bindings.rs              bindgen-generated C bindings (from include/radio/)
    metadata_bindings.rs     bindgen-generated bindings (from include/robot_metadata.h)
    proto_packets_gen.rs     micropb-generated Rust proto types (from proto/)
```

## Wire Framing

Radio packets use the format: `CRC32 || varint(len) || RadioPacket bytes`

The `RadioPacket` proto `oneof` wire tag replaces the legacy C `CommandCode` byte. Field numbers in `radio.proto` intentionally match the old `CC_*` enum values for cross-reference. CRC32 and length are transport-layer concerns external to the proto encoding.

## C Headers

Include the top-level radio header for all radio-facing types:

```c
#include "ateam-common-packets/include/radio/radio.h"
```

Wire-internal headers (motor controller, kicker, power board) are under `include/wire/` and are not needed by AI software.

## Proto / ROS2 Integration

Generate ROS2 `.msg` files from the proto schemas using the provided CMake function:

```cmake
include(Ros2MsgGen)
generate_ros2_msgs(
  PROTO_FILES   ${PROTO_FILES}
  PROTO_PATHS   ${PROTO_DIR}
  OUTPUT_DIR    ${CMAKE_CURRENT_BINARY_DIR}/msg
)
rosidl_generate_interfaces(${PROJECT_NAME} ${GENERATED_ROS2_MSGS})
```

Generate C++ `fromProto()` conversion headers:

```cmake
include(Ros2CppConvertGen)
generate_ros2_cpp_conversions(
  PROTO_FILES          ${PROTO_FILES}
  PROTO_PATHS          ${PROTO_DIR}
  PROTO_INCLUDE_PREFIX ateam_common_packets
  ROS2_PACKAGE         ateam_radio_msgs
  OUTPUT_DIR           ${CMAKE_CURRENT_BINARY_DIR}/include/conversions
)
```

## Rust Bindings

```sh
nix develop
cargo build   # generates bindings.rs and proto_packets_gen.rs
cargo test    # runs packing/size tests
```

Requires `arm-none-eabi-gcc` on the path (provided by the Nix shell) for bindgen's cross-compilation target headers. Set `$ARM_NONE_EABI_ROOT` to override the sysroot search.

## Testing

```sh
nix develop
make test                                          # all suites
python3 -m pytest cmake/tests/test_plugin.py -v   # ROS2 .msg plugin tests
python3 -m pytest cmake/tests/test_cpp_plugin.py -v  # C++ conversion plugin tests
cargo test                                         # Rust binding tests
```
