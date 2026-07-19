"""
Integration and unit tests for protoc_gen_ros2msg.py.

Run with:  pytest cmake/tests/test_plugin.py -v
Requires:  protoc, python3-protobuf (pip install protobuf or nix python3Packages.protobuf)
"""

import subprocess
import sys
import tempfile
from pathlib import Path

import pytest

# --------------------------------------------------------------------------- #
# Paths
# --------------------------------------------------------------------------- #

_THIS_DIR  = Path(__file__).parent
_REPO_ROOT = _THIS_DIR.parent.parent          # ateam-common-packets/
_PLUGIN    = _THIS_DIR.parent / "protoc_gen_ros2msg.py"
_PROTO_DIR = _REPO_ROOT / "proto"

_FIXTURE_DIR = _THIS_DIR / "protos"

_ALL_PROTOS = [
    "maneuvers.proto",
    "motor.proto",
    "power.proto",
    "kicker.proto",
    "body_control.proto",
    "control.proto",
    "telemetry.proto",
    "discovery.proto",
    "diagnostics.proto",
    "robot_parameters.proto",
    "radio.proto",
]

# --------------------------------------------------------------------------- #
# Helper
# --------------------------------------------------------------------------- #

def run_plugin(
    proto_files: list[str],
    optional_submsg: str = "has_field",
) -> tuple[int, str, dict[str, str]]:
    return _run_plugin_impl(proto_files, _PROTO_DIR, optional_submsg)


def run_plugin_fixture(
    fixture_file: str,
    optional_submsg: str = "has_field",
) -> tuple[int, str, dict[str, str]]:
    """Run the plugin against a test fixture proto in cmake/tests/protos/."""
    return _run_plugin_impl(
        [fixture_file],
        _FIXTURE_DIR,
        optional_submsg,
        extra_proto_path=_PROTO_DIR,
    )


def _run_plugin_impl(
    proto_files: list[str],
    proto_dir: Path,
    optional_submsg: str = "has_field",
    extra_proto_path: Path | None = None,
) -> tuple[int, str, dict[str, str]]:
    """
    Invoke protoc with the ros2msg plugin.

    Returns (exit_code, stderr_text, {filename: content}).
    """
    with tempfile.TemporaryDirectory() as tmp:
        tmp_path = Path(tmp)
        cmd = [
            "protoc",
            f"--plugin=protoc-gen-ros2msg={_PLUGIN}",
            f"--ros2msg_opt=optional_submsg={optional_submsg}",
            f"--ros2msg_out={tmp_path}",
            f"--proto_path={proto_dir}",
        ]
        if extra_proto_path is not None:
            cmd.append(f"--proto_path={extra_proto_path}")
        cmd.extend(str(proto_dir / f) for f in proto_files)
        result = subprocess.run(cmd, capture_output=True, text=True)
        files = {p.name: p.read_text() for p in tmp_path.glob("*.msg")}
        return result.returncode, result.stderr, files


# --------------------------------------------------------------------------- #
# Error-mode tests
# --------------------------------------------------------------------------- #

class TestErrorMode:
    """Verify optional_submsg=error rejects non-oneof message-type fields."""

    def test_error_mode_fails_on_submessage_fields(self):
        """telemetry.proto has non-oneof message fields — must fail under error mode."""
        code, err, _ = run_plugin(["telemetry.proto"], optional_submsg="error")
        assert code != 0, "Expected failure but plugin succeeded"

    def test_error_mode_reports_each_offending_field(self):
        """Error output must name every non-oneof message field in ExtendedTelemetry."""
        code, err, _ = run_plugin(["telemetry.proto"], optional_submsg="error")
        expected_fields = [
            "power_status",
            "front_left_motor",
            "back_left_motor",
            "back_right_motor",
            "front_right_motor",
            "body_control_telem",
            "kicker_status",
        ]
        for field in expected_fields:
            assert field in err, (
                f"Expected field '{field}' in error output but got:\n{err}"
            )

    def test_error_mode_passes_on_oneof_only_messages(self):
        """control.proto oneof fields are fine even in error mode — no submessages outside oneof."""
        code, err, _ = run_plugin(
            ["maneuvers.proto", "control.proto"],
            optional_submsg="error",
        )
        assert code == 0, f"Unexpected failure:\n{err}"

    def test_has_field_mode_succeeds_on_all_protos(self):
        """has_field mode must succeed on every proto file without errors."""
        code, err, generated = run_plugin(_ALL_PROTOS, optional_submsg="has_field")
        assert code == 0, f"protoc failed:\n{err}"
        assert len(generated) > 0


# --------------------------------------------------------------------------- #
# Unit tests for plugin internals
# --------------------------------------------------------------------------- #

# Import the plugin module directly for white-box unit tests.
sys.path.insert(0, str(_PLUGIN.parent))
import protoc_gen_ros2msg as plugin  # noqa: E402  (import after path manipulation)


class TestParseOptions:
    def test_empty_string(self):
        assert plugin.parse_options("") == {}

    def test_single_pair(self):
        assert plugin.parse_options("optional_submsg=has_field") == {
            "optional_submsg": "has_field"
        }

    def test_multiple_pairs(self):
        result = plugin.parse_options("a=1,b=2,c=three")
        assert result == {"a": "1", "b": "2", "c": "three"}

    def test_value_with_equals(self):
        # Value itself contains '=' — only first '=' is the separator.
        result = plugin.parse_options("key=val=ue")
        assert result == {"key": "val=ue"}

    def test_entry_without_equals_is_skipped(self):
        result = plugin.parse_options("bare,key=value")
        assert result == {"key": "value"}


class TestStripPackage:
    def test_strips_package_prefix(self):
        assert plugin.strip_package(".ateam.FooBar") == "FooBar"

    def test_nested_package(self):
        assert plugin.strip_package(".a.b.c.Msg") == "Msg"

    def test_no_package(self):
        assert plugin.strip_package("Msg") == "Msg"


class TestUnknownOption:
    """Plugin must return a non-zero error response for unknown option values."""

    def test_unknown_optional_submsg_value(self):
        code, err, _ = run_plugin(["maneuvers.proto"], optional_submsg="bogus")
        assert code != 0, "Expected failure for unknown option but plugin succeeded"
        assert "bogus" in err


class TestMinimalUintType:
    def test_fits_uint8(self):
        assert plugin.minimal_uint_type(0)   == "uint8"
        assert plugin.minimal_uint_type(64)  == "uint8"
        assert plugin.minimal_uint_type(255) == "uint8"

    def test_fits_uint16(self):
        assert plugin.minimal_uint_type(256)   == "uint16"
        assert plugin.minimal_uint_type(32768) == "uint16"
        assert plugin.minimal_uint_type(65535) == "uint16"

    def test_needs_uint32(self):
        assert plugin.minimal_uint_type(65536)    == "uint32"
        assert plugin.minimal_uint_type(67108864) == "uint32"


class TestBitmaskAnnotation:
    """Verify [(ateam.bitmask)] fields get minimal type and inline constants."""

    def test_uint8_bitmask_battery_status(self):
        """BatteryStatusFlag max=64 → uint8 field and constants."""
        code, err, generated = run_plugin(["power.proto"])
        assert code == 0, f"protoc failed:\n{err}"
        content = generated["BatteryInfo.msg"]
        assert "uint8 status_flags" in content
        assert "uint8 BATTERY_STATUS_OK=1" in content
        assert "uint8 BATTERY_STATUS_CELL_IMBALANCE_WARN=64" in content

    def test_uint8_bitmask_power_status(self):
        """PowerStatusFlag max=32 → uint8."""
        code, err, generated = run_plugin(["power.proto"])
        assert code == 0, f"protoc failed:\n{err}"
        content = generated["PowerTelemetry.msg"]
        assert "uint8 status_flags" in content
        assert "uint8 POWER_STATUS_OK=1" in content

    def test_uint8_bitmask_kicker_status(self):
        """KickerStatusFlag max=64 → uint8."""
        code, err, generated = run_plugin(["motor.proto", "kicker.proto"])
        assert code == 0, f"protoc failed:\n{err}"
        content = generated["KickerTelemetry.msg"]
        assert "uint8 status_flags" in content
        assert "uint8 KICKER_STATUS_DRIBBLER_FW_LOADED=64" in content

    def test_uint16_bitmask_ccm_error(self):
        """CcmErrorFlag max=32768 → uint16."""
        code, err, generated = run_plugin(["motor.proto"])
        assert code == 0, f"protoc failed:\n{err}"
        content = generated["CcmTelemetry.msg"]
        assert "uint16 error_flags" in content
        assert "uint16 CCM_ERR_RESET_PIN=32768" in content

    def test_uint32_bitmask_basic_telemetry(self):
        """BasicTelemetryFlag max=67108864 → uint32."""
        code, err, generated = run_plugin(_ALL_PROTOS)
        assert code == 0, f"protoc failed:\n{err}"
        content = generated["BasicTelemetry.msg"]
        assert "uint32 status_flags" in content
        assert "uint32 BTEL_FLAG_CONTROLLER_RESET=67108864" in content

    def test_bitmask_constants_appear_before_field(self):
        """Flag constants must be declared before the field that uses them."""
        code, err, generated = run_plugin(["motor.proto"])
        assert code == 0, f"protoc failed:\n{err}"
        content = generated["CcmTelemetry.msg"]
        const_pos = content.index("uint16 CCM_ERR_MASTER_ERROR=1")
        field_pos = content.index("uint16 error_flags")
        assert const_pos < field_pos

    def test_bitmask_section_header_comment(self):
        """A '# bitmask: EnumName' comment must precede the constants."""
        code, err, generated = run_plugin(["motor.proto"])
        assert code == 0, f"protoc failed:\n{err}"
        assert "# bitmask: CcmErrorFlag" in generated["CcmTelemetry.msg"]


# --------------------------------------------------------------------------- #
# Oneof field-number tests
# --------------------------------------------------------------------------- #

class TestOneofFieldNumbers:
    """
    Oneof case constants must use proto field numbers, not sequential declaration
    indices.  Field numbers are the stable proto identifier; declaration order
    may change freely without altering wire semantics.
    """

    def test_sparse_oneof_constants_use_field_numbers(self):
        """Non-sequential field numbers (1,10,20) must appear verbatim, not as 1,2,3."""
        code, err, generated = run_plugin_fixture("sparse_oneof.proto")
        assert code == 0, f"protoc failed:\n{err}"
        content = generated["SparseOneof.msg"]
        assert "uint8 ONEOF_CHOICE_A=1" in content
        assert "uint8 ONEOF_CHOICE_B=10" in content
        assert "uint8 ONEOF_CHOICE_C=20" in content

    def test_sparse_oneof_none_is_always_zero(self):
        """NONE sentinel must be 0 regardless of field numbers used by arms."""
        code, err, generated = run_plugin_fixture("sparse_oneof.proto")
        assert code == 0, f"protoc failed:\n{err}"
        assert "uint8 ONEOF_CHOICE_NONE=0" in generated["SparseOneof.msg"]

    def test_sparse_oneof_no_sequential_constants(self):
        """Verify the old sequential values (2,3) are NOT present."""
        code, err, generated = run_plugin_fixture("sparse_oneof.proto")
        assert code == 0, f"protoc failed:\n{err}"
        content = generated["SparseOneof.msg"]
        assert "ONEOF_CHOICE_B=2" not in content
        assert "ONEOF_CHOICE_C=3" not in content

    def test_sequential_oneof_still_works(self):
        """Sequential field numbers (common case) are unaffected by the change."""
        code, err, generated = run_plugin(["control.proto", "maneuvers.proto"])
        assert code == 0, f"protoc failed:\n{err}"
        content = generated["BasicControl.msg"]
        # Field numbers 1-9 happen to equal the old sequential indices.
        assert "uint8 ONEOF_CMD_GLOBAL_POS=1" in content
        assert "uint8 ONEOF_CMD_GLOBAL_VEL=2" in content
        assert "uint8 ONEOF_CMD_POINT_LINE=9" in content

    def test_oneof_field_number_over_255_fails(self):
        """Field number 256 in a oneof must fail at configure time."""
        code, err, _ = run_plugin_fixture("overflow_oneof.proto")
        assert code != 0, "Expected failure for oneof field number > 255"
        assert "256" in err
        assert "255" in err  # the stated limit

    def test_overflow_error_names_message_oneof_and_field(self):
        """Error message must identify the message, oneof, and field by name."""
        code, err, _ = run_plugin_fixture("overflow_oneof.proto")
        assert code != 0
        assert "OverflowOneof" in err
        assert "choice" in err
        assert "b" in err


# --------------------------------------------------------------------------- #
# Bitmask validation error tests
# --------------------------------------------------------------------------- #

class TestBitmaskValidation:
    """Verify plugin rejects invalid [(ateam.bitmask)] annotations."""

    def test_bitmask_on_non_uint32_field_fails(self):
        """[(ateam.bitmask)] on int32 field must fail with a descriptive error."""
        code, err, _ = run_plugin_fixture("bad_bitmask_type.proto")
        assert code != 0, "Expected failure for bitmask on non-uint32 field"
        assert "BadBitmaskType.error_flags" in err
        assert "uint32" in err

    def test_bitmask_unknown_enum_fails(self):
        """[(ateam.bitmask)] referencing an undefined enum must fail."""
        code, err, _ = run_plugin_fixture("bad_bitmask_enum.proto")
        assert code != 0, "Expected failure for bitmask with unknown enum reference"
        assert "BadBitmaskEnum.error_flags" in err
        assert "NonExistentEnum" in err


# --------------------------------------------------------------------------- #
# Nested type error tests
# --------------------------------------------------------------------------- #

class TestNestedTypes:
    """Nested message/enum definitions are supported via flattened names.

    A message Bar defined inside Foo generates Foo_Bar.msg, matching the naming
    convention proto C++ uses for nested types (Foo_Bar is the actual class name).
    """

    def test_nested_message_generates_flat_msg(self):
        """InnerMessage nested in OuterMessage → OuterMessage_InnerMessage.msg."""
        code, err, generated = run_plugin_fixture("nested_types.proto")
        assert code == 0, f"protoc failed:\n{err}"
        assert "OuterMessage_InnerMessage.msg" in generated

    def test_nested_enum_generates_flat_msg(self):
        """InnerEnum nested in OuterMessage → OuterMessage_InnerEnum.msg."""
        code, err, generated = run_plugin_fixture("nested_types.proto")
        assert code == 0, f"protoc failed:\n{err}"
        assert "OuterMessage_InnerEnum.msg" in generated

    def test_outer_references_inner_by_flat_name(self):
        """The 'inner' field in OuterMessage must use type OuterMessage_InnerMessage."""
        code, err, generated = run_plugin_fixture("nested_types.proto")
        assert code == 0, f"protoc failed:\n{err}"
        content = generated["OuterMessage.msg"]
        assert "OuterMessage_InnerMessage inner" in content

    def test_nested_inner_message_content(self):
        """OuterMessage_InnerMessage.msg must contain the nested message's field."""
        code, err, generated = run_plugin_fixture("nested_types.proto")
        assert code == 0, f"protoc failed:\n{err}"
        assert "uint32 value" in generated["OuterMessage_InnerMessage.msg"]

    def test_nested_enum_content(self):
        """OuterMessage_InnerEnum.msg must contain the enum constants."""
        code, err, generated = run_plugin_fixture("nested_types.proto")
        assert code == 0, f"protoc failed:\n{err}"
        content = generated["OuterMessage_InnerEnum.msg"]
        assert "int32 INNER_NONE=0" in content
        assert "int32 INNER_A=1" in content


# --------------------------------------------------------------------------- #
# diagnostics.proto output tests
# --------------------------------------------------------------------------- #

class TestDiagnosticsProto:
    """Verify diagnostics.proto generates the expected .msg output."""

    def test_error_telemetry_msg_generated(self):
        """diagnostics.proto must produce ErrorTelemetry.msg."""
        code, err, generated = run_plugin(["diagnostics.proto"])
        assert code == 0, f"protoc failed:\n{err}"
        assert "ErrorTelemetry.msg" in generated

    def test_error_telemetry_has_timestamp_field(self):
        code, err, generated = run_plugin(["diagnostics.proto"])
        assert code == 0, f"protoc failed:\n{err}"
        assert "uint32 timestamp" in generated["ErrorTelemetry.msg"]

    def test_error_telemetry_has_error_message_field(self):
        code, err, generated = run_plugin(["diagnostics.proto"])
        assert code == 0, f"protoc failed:\n{err}"
        assert "string error_message" in generated["ErrorTelemetry.msg"]


# --------------------------------------------------------------------------- #
# robot_parameters.proto output tests
# --------------------------------------------------------------------------- #

class TestRobotParameters:
    """robot_parameters.proto: ParameterCommand + enums generate expected .msg files."""

    def test_parameter_command_msg_generated(self):
        code, err, generated = run_plugin(["robot_parameters.proto"])
        assert code == 0, f"protoc failed:\n{err}"
        assert "ParameterCommand.msg" in generated

    def test_parameter_command_code_enum_generated(self):
        code, err, generated = run_plugin(["robot_parameters.proto"])
        assert code == 0, f"protoc failed:\n{err}"
        assert "ParameterCommandCode.msg" in generated

    def test_parameter_name_enum_generated(self):
        code, err, generated = run_plugin(["robot_parameters.proto"])
        assert code == 0, f"protoc failed:\n{err}"
        assert "ParameterName.msg" in generated

    def test_parameter_command_has_command_code_field(self):
        """Enum fields emit as int32 in ROS2 .msg."""
        code, err, generated = run_plugin(["robot_parameters.proto"])
        assert code == 0, f"protoc failed:\n{err}"
        assert "int32 command_code" in generated["ParameterCommand.msg"]

    def test_parameter_command_has_parameter_name_field(self):
        """Enum fields emit as int32 in ROS2 .msg."""
        code, err, generated = run_plugin(["robot_parameters.proto"])
        assert code == 0, f"protoc failed:\n{err}"
        assert "int32 parameter_name" in generated["ParameterCommand.msg"]

    def test_parameter_command_has_repeated_float_data(self):
        """data is repeated float — ROS2 maps repeated float to float32[]."""
        code, err, generated = run_plugin(["robot_parameters.proto"])
        assert code == 0, f"protoc failed:\n{err}"
        assert "float32[] data" in generated["ParameterCommand.msg"]

    def test_parameter_command_code_constants(self):
        code, err, generated = run_plugin(["robot_parameters.proto"])
        assert code == 0, f"protoc failed:\n{err}"
        content = generated["ParameterCommandCode.msg"]
        assert "int32 PCC_READ=0" in content
        assert "int32 PCC_WRITE=1" in content
        assert "int32 PCC_ACK=2" in content
        assert "int32 PCC_NACK_INVALID_NAME=3" in content
        assert "int32 PCC_NACK_INVALID_TYPE_FOR_NAME=4" in content

    def test_parameter_name_constants(self):
        code, err, generated = run_plugin(["robot_parameters.proto"])
        assert code == 0, f"protoc failed:\n{err}"
        content = generated["ParameterName.msg"]
        assert "int32 PN_KF_PROCESS_STD=0" in content
        assert "int32 PN_PHYS_FRICTION_MODEL=6" in content
        assert "int32 PN_TWIST_FB_PIDII_ANGULAR=13" in content


# --------------------------------------------------------------------------- #
# radio.proto output tests
# --------------------------------------------------------------------------- #

_RADIO_DEPS = _ALL_PROTOS


class TestRadioPacket:
    """radio.proto: oneof replaces CommandCode; field numbers match legacy CC_* values."""

    def test_radio_packet_msg_generated(self):
        code, err, generated = run_plugin(_RADIO_DEPS)
        assert code == 0, f"protoc failed:\n{err}"
        assert "RadioPacket.msg" in generated

    def test_keepalive_msg_generated(self):
        """Empty Keepalive message generates a (header-only) .msg file."""
        code, err, generated = run_plugin(_RADIO_DEPS)
        assert code == 0, f"protoc failed:\n{err}"
        assert "Keepalive.msg" in generated

    def test_oneof_case_constants_match_legacy_cc_values(self):
        """Field numbers intentionally align with legacy CC_* values for migration docs."""
        code, err, generated = run_plugin(_RADIO_DEPS)
        assert code == 0, f"protoc failed:\n{err}"
        content = generated["RadioPacket.msg"]
        assert "uint8 ONEOF_PAYLOAD_KEEPALIVE=4" in content       # CC_KEEPALIVE=4
        assert "uint8 ONEOF_PAYLOAD_HELLO_REQUEST=21" in content   # CC_HELLO_REQ=21
        assert "uint8 ONEOF_PAYLOAD_HELLO_RESPONSE=22" in content  # CC_HELLO_RESP=22
        assert "uint8 ONEOF_PAYLOAD_TELEMETRY=41" in content       # CC_TELEMETRY=41
        assert "uint8 ONEOF_PAYLOAD_EXTENDED_TELEMETRY=42" in content  # CC_CONTROL_DEBUG_TELEMETRY=42
        assert "uint8 ONEOF_PAYLOAD_PARAMETER_COMMAND=43" in content   # CC_ROBOT_PARAMETER_COMMAND=43
        assert "uint8 ONEOF_PAYLOAD_ERROR_TELEMETRY=44" in content # CC_ERROR_TELEMETRY=44
        assert "uint8 ONEOF_PAYLOAD_CONTROL=61" in content         # CC_CONTROL=61

    def test_oneof_none_is_zero(self):
        code, err, generated = run_plugin(_RADIO_DEPS)
        assert code == 0, f"protoc failed:\n{err}"
        assert "uint8 ONEOF_PAYLOAD_NONE=0" in generated["RadioPacket.msg"]

    def test_payload_case_field_present(self):
        code, err, generated = run_plugin(_RADIO_DEPS)
        assert code == 0, f"protoc failed:\n{err}"
        assert "uint8 payload_case" in generated["RadioPacket.msg"]

    def test_arm_types_reference_correct_msg_names(self):
        """Each oneof arm field uses the flat ROS2 message type name."""
        code, err, generated = run_plugin(_RADIO_DEPS)
        assert code == 0, f"protoc failed:\n{err}"
        content = generated["RadioPacket.msg"]
        assert "HelloRequest hello_request" in content
        assert "BasicControl control" in content
        assert "BasicTelemetry telemetry" in content
        assert "ExtendedTelemetry extended_telemetry" in content
        assert "ParameterCommand parameter_command" in content
        assert "ErrorTelemetry error_telemetry" in content
        assert "Keepalive keepalive" in content
