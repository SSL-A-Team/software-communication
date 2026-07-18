"""
Integration tests for protoc_gen_ros2cpp.py.

Run with:  pytest cmake/tests/test_cpp_plugin.py -v
Requires:  protoc, python3-protobuf
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
_REPO_ROOT = _THIS_DIR.parent.parent
_PLUGIN    = _THIS_DIR.parent / "protoc_gen_ros2cpp.py"
_PROTO_DIR = _REPO_ROOT / "proto"
_FIXTURE_DIR = _THIS_DIR / "protos"

_DEFAULT_OPTS = (
    "proto_include_prefix=ateam_common_packets,"
    "ros2_package=ateam_radio_msgs,"
    "namespace=ateam_conversions"
)

# --------------------------------------------------------------------------- #
# Helpers
# --------------------------------------------------------------------------- #

def run_cpp_plugin(
    proto_files: list[str],
    opts: str = _DEFAULT_OPTS,
) -> tuple[int, str, dict[str, str]]:
    return _run_impl(proto_files, _PROTO_DIR, opts)


def run_cpp_plugin_fixture(
    fixture_file: str,
    opts: str = _DEFAULT_OPTS,
) -> tuple[int, str, dict[str, str]]:
    return _run_impl(
        [fixture_file],
        _FIXTURE_DIR,
        opts,
        extra_proto_path=_PROTO_DIR,
    )


def _run_impl(
    proto_files: list[str],
    proto_dir: Path,
    opts: str,
    extra_proto_path: Path | None = None,
) -> tuple[int, str, dict[str, str]]:
    with tempfile.TemporaryDirectory() as tmp:
        tmp_path = Path(tmp)
        cmd = [
            "protoc",
            f"--plugin=protoc-gen-ros2cpp={_PLUGIN}",
            f"--ros2cpp_opt={opts}",
            f"--ros2cpp_out={tmp_path}",
            f"--proto_path={proto_dir}",
        ]
        if extra_proto_path is not None:
            cmd.append(f"--proto_path={extra_proto_path}")
        cmd.extend(str(proto_dir / f) for f in proto_files)
        result = subprocess.run(cmd, capture_output=True, text=True)
        files = {p.name: p.read_text() for p in tmp_path.glob("*.hpp")}
        return result.returncode, result.stderr, files


# Import plugin for white-box unit tests.
sys.path.insert(0, str(_PLUGIN.parent))
import protoc_gen_ros2cpp as cpp_plugin  # noqa: E402


# --------------------------------------------------------------------------- #
# Header structure tests
# --------------------------------------------------------------------------- #

class TestHeaderStructure:
    """Verify generated file structure: pragma, includes, namespace."""

    def test_pragma_once(self):
        code, err, files = run_cpp_plugin(["motor.proto"])
        assert code == 0, f"protoc failed:\n{err}"
        assert files["motor_conversions.hpp"].startswith("#pragma once")

    def test_proto_include(self):
        code, err, files = run_cpp_plugin(["motor.proto"])
        assert code == 0, f"protoc failed:\n{err}"
        assert "#include <ateam_common_packets/motor.pb.h>" in files["motor_conversions.hpp"]

    def test_ros2_message_includes(self):
        code, err, files = run_cpp_plugin(["motor.proto"])
        assert code == 0, f"protoc failed:\n{err}"
        content = files["motor_conversions.hpp"]
        assert "#include <ateam_radio_msgs/msg/ccm_telemetry.hpp>" in content
        assert "#include <ateam_radio_msgs/msg/ccm_current_telemetry.hpp>" in content
        assert "#include <ateam_radio_msgs/msg/ccm_velocity_telemetry.hpp>" in content

    def test_namespace_open_and_close(self):
        code, err, files = run_cpp_plugin(["motor.proto"])
        assert code == 0, f"protoc failed:\n{err}"
        content = files["motor_conversions.hpp"]
        assert "namespace ateam_conversions {" in content
        assert "}  // namespace ateam_conversions" in content

    def test_custom_namespace(self):
        opts = _DEFAULT_OPTS.replace(
            "namespace=ateam_conversions", "namespace=my_ns"
        )
        code, err, files = run_cpp_plugin(["motor.proto"], opts=opts)
        assert code == 0, f"protoc failed:\n{err}"
        content = files["motor_conversions.hpp"]
        assert "namespace my_ns {" in content
        assert "}  // namespace my_ns" in content

    def test_import_dependency_included(self):
        """control.proto imports maneuvers.proto → maneuvers_conversions.hpp included."""
        code, err, files = run_cpp_plugin(["control.proto", "maneuvers.proto"])
        assert code == 0, f"protoc failed:\n{err}"
        assert '#include "maneuvers_conversions.hpp"' in files["control_conversions.hpp"]

    def test_stdlib_headers_included(self):
        code, err, files = run_cpp_plugin(["motor.proto"])
        assert code == 0, f"protoc failed:\n{err}"
        content = files["motor_conversions.hpp"]
        assert "#include <cstdint>" in content
        assert "#include <vector>" in content

    def test_function_signature(self):
        code, err, files = run_cpp_plugin(["motor.proto"])
        assert code == 0, f"protoc failed:\n{err}"
        content = files["motor_conversions.hpp"]
        assert (
            "inline ateam_radio_msgs::msg::CcmTelemetry fromProto(const ateam::CcmTelemetry& p)"
            in content
        )


# --------------------------------------------------------------------------- #
# Scalar field tests
# --------------------------------------------------------------------------- #

class TestScalarFields:
    """Scalar proto fields map to direct assignment."""

    def test_uint32_direct_assign(self):
        code, err, files = run_cpp_plugin(["motor.proto"])
        assert code == 0, f"protoc failed:\n{err}"
        assert "msg.bus_voltage_mv = p.bus_voltage_mv();" in files["motor_conversions.hpp"]

    def test_float_direct_assign(self):
        code, err, files = run_cpp_plugin(["motor.proto"])
        assert code == 0, f"protoc failed:\n{err}"
        assert "msg.vel_setpoint_rads = p.vel_setpoint_rads();" in files["motor_conversions.hpp"]

    def test_bool_direct_assign(self):
        code, err, files = run_cpp_plugin(["control.proto", "maneuvers.proto"])
        assert code == 0, f"protoc failed:\n{err}"
        assert "msg.estop_brake = p.estop_brake();" in files["control_conversions.hpp"]

    def test_sint32_direct_assign(self):
        """sint32 is signed; maps to int32_t via direct assign."""
        code, err, files = run_cpp_plugin(["telemetry.proto"] + [
            "maneuvers.proto", "motor.proto", "power.proto",
            "kicker.proto", "body_control.proto",
        ])
        assert code == 0, f"protoc failed:\n{err}"
        content = files["telemetry_conversions.hpp"]
        assert "msg.kf_pos_x = p.kf_pos_x();" in content

    def test_repeated_uint32_push_back(self):
        code, err, files = run_cpp_plugin(["motor.proto"])
        assert code == 0, f"protoc failed:\n{err}"
        assert (
            "for (const auto& v : p.current_samples_ma()) "
            "msg.current_samples_ma.push_back(v);"
            in files["motor_conversions.hpp"]
        )


# --------------------------------------------------------------------------- #
# Enum field tests
# --------------------------------------------------------------------------- #

class TestEnumFields:
    """Enum proto fields map to static_cast<int32_t>."""

    def test_enum_static_cast(self):
        code, err, files = run_cpp_plugin(["motor.proto"])
        assert code == 0, f"protoc failed:\n{err}"
        assert (
            "msg.motion_control_type = static_cast<int32_t>(p.motion_control_type());"
            in files["motor_conversions.hpp"]
        )

    def test_enum_in_control(self):
        code, err, files = run_cpp_plugin(["control.proto", "maneuvers.proto"])
        assert code == 0, f"protoc failed:\n{err}"
        assert (
            "msg.kick_request = static_cast<int32_t>(p.kick_request());"
            in files["control_conversions.hpp"]
        )
        assert (
            "msg.dribbler_mode = static_cast<int32_t>(p.dribbler_mode());"
            in files["control_conversions.hpp"]
        )


# --------------------------------------------------------------------------- #
# Bitmask field tests
# --------------------------------------------------------------------------- #

class TestBitmaskFields:
    """[(ateam.bitmask)] fields use static_cast to the narrowed type."""

    def test_uint16_bitmask_cast(self):
        """CcmErrorFlag max=32768 → uint16_t cast."""
        code, err, files = run_cpp_plugin(["motor.proto"])
        assert code == 0, f"protoc failed:\n{err}"
        assert (
            "msg.error_flags = static_cast<uint16_t>(p.error_flags());"
            in files["motor_conversions.hpp"]
        )

    def test_uint8_bitmask_cast(self):
        """PowerStatusFlag max<256 → uint8_t cast."""
        code, err, files = run_cpp_plugin(["power.proto"])
        assert code == 0, f"protoc failed:\n{err}"
        assert (
            "msg.status_flags = static_cast<uint8_t>(p.status_flags());"
            in files["power_conversions.hpp"]
        )

    def test_uint32_bitmask_cast(self):
        """BasicTelemetryFlag max=67108864 → uint32_t cast."""
        code, err, files = run_cpp_plugin([
            "telemetry.proto", "maneuvers.proto", "motor.proto",
            "power.proto", "kicker.proto", "body_control.proto",
        ])
        assert code == 0, f"protoc failed:\n{err}"
        assert (
            "msg.status_flags = static_cast<uint32_t>(p.status_flags());"
            in files["telemetry_conversions.hpp"]
        )


# --------------------------------------------------------------------------- #
# Submessage field tests (HAS_FIELD mode)
# --------------------------------------------------------------------------- #

class TestSubmessageHasField:
    """Non-oneof message fields get has_ check + conditional fromProto."""

    def test_has_field_emitted(self):
        code, err, files = run_cpp_plugin(["motor.proto"])
        assert code == 0, f"protoc failed:\n{err}"
        content = files["motor_conversions.hpp"]
        assert "msg.has_current_telem = p.has_current_telem();" in content
        assert "if (p.has_current_telem()) msg.current_telem = fromProto(p.current_telem());" in content

    def test_has_field_for_velocity_telem(self):
        code, err, files = run_cpp_plugin(["motor.proto"])
        assert code == 0, f"protoc failed:\n{err}"
        content = files["motor_conversions.hpp"]
        assert "msg.has_velocity_telem = p.has_velocity_telem();" in content
        assert "if (p.has_velocity_telem()) msg.velocity_telem = fromProto(p.velocity_telem());" in content


# --------------------------------------------------------------------------- #
# oneof field tests
# --------------------------------------------------------------------------- #

class TestOneofFields:
    """oneof expansions: case discriminant + conditional arm assignments."""

    def test_case_discriminant(self):
        code, err, files = run_cpp_plugin(["control.proto", "maneuvers.proto"])
        assert code == 0, f"protoc failed:\n{err}"
        assert (
            "msg.cmd_case = static_cast<uint8_t>(p.cmd_case());"
            in files["control_conversions.hpp"]
        )

    def test_message_arm_has_check(self):
        code, err, files = run_cpp_plugin(["control.proto", "maneuvers.proto"])
        assert code == 0, f"protoc failed:\n{err}"
        content = files["control_conversions.hpp"]
        assert "if (p.has_global_pos()) msg.global_pos = fromProto(p.global_pos());" in content
        assert "if (p.has_point_line()) msg.point_line = fromProto(p.point_line());" in content

    def test_scalar_oneof_arm_direct_assign(self):
        """Scalar arms in oneof use direct assignment (no has_ check needed)."""
        code, err, files = run_cpp_plugin_fixture("sparse_oneof.proto")
        assert code == 0, f"protoc failed:\n{err}"
        content = files["sparse_oneof_conversions.hpp"]
        assert "msg.choice_case = static_cast<uint8_t>(p.choice_case());" in content
        assert "msg.a = p.a();" in content
        assert "msg.b = p.b();" in content
        assert "msg.c = p.c();" in content


# --------------------------------------------------------------------------- #
# Error mode tests
# --------------------------------------------------------------------------- #

class TestErrorMode:
    """optional_submsg=error rejects non-oneof message fields."""

    def _error_opts(self) -> str:
        return _DEFAULT_OPTS + ",optional_submsg=error"

    def test_error_mode_fails_on_submessage(self):
        code, err, _ = run_cpp_plugin(["motor.proto"], opts=self._error_opts())
        assert code != 0, "Expected failure for non-oneof message fields in error mode"

    def test_error_mode_names_offending_field(self):
        code, err, _ = run_cpp_plugin(["motor.proto"], opts=self._error_opts())
        assert code != 0
        assert "current_telem" in err

    def test_error_mode_passes_on_scalar_only_message(self):
        """CcmCurrentTelemetry has only scalar fields — passes even in error mode."""
        code, err, files = run_cpp_plugin_fixture(
            "sparse_oneof.proto", opts=self._error_opts()
        )
        assert code == 0, f"Unexpected failure:\n{err}"

    def test_error_mode_passes_oneof_message_arms(self):
        """oneof message arms are exempt from the error mode restriction."""
        code, err, files = run_cpp_plugin(
            ["control.proto", "maneuvers.proto"], opts=self._error_opts()
        )
        assert code == 0, f"Unexpected failure:\n{err}"


# --------------------------------------------------------------------------- #
# Required option validation
# --------------------------------------------------------------------------- #

class TestRequiredOptions:
    """plugin must error clearly when required options are missing."""

    def test_missing_proto_include_prefix(self):
        opts = "ros2_package=ateam_radio_msgs,namespace=ateam_conversions"
        code, err, _ = run_cpp_plugin(["motor.proto"], opts=opts)
        assert code != 0
        assert "proto_include_prefix" in err

    def test_missing_ros2_package(self):
        opts = "proto_include_prefix=ateam_common_packets,namespace=ateam_conversions"
        code, err, _ = run_cpp_plugin(["motor.proto"], opts=opts)
        assert code != 0
        assert "ros2_package" in err

    def test_missing_both_required(self):
        opts = "namespace=ateam_conversions"
        code, err, _ = run_cpp_plugin(["motor.proto"], opts=opts)
        assert code != 0
        assert "proto_include_prefix" in err
        assert "ros2_package" in err

    def test_unknown_optional_submsg(self):
        opts = _DEFAULT_OPTS + ",optional_submsg=bogus"
        code, err, _ = run_cpp_plugin(["motor.proto"], opts=opts)
        assert code != 0
        assert "bogus" in err


# --------------------------------------------------------------------------- #
# Unit tests for plugin helpers
# --------------------------------------------------------------------------- #

class TestToSnakeCase:
    def test_single_word(self):
        assert cpp_plugin.to_snake_case("Motor") == "motor"

    def test_camel_two_words(self):
        assert cpp_plugin.to_snake_case("CcmTelemetry") == "ccm_telemetry"

    def test_all_upper_acronym_followed_by_word(self):
        assert cpp_plugin.to_snake_case("CcmCurrentTelemetry") == "ccm_current_telemetry"

    def test_already_snake(self):
        assert cpp_plugin.to_snake_case("already_snake") == "already_snake"


class TestMinimalUintCtype:
    def test_uint8(self):
        assert cpp_plugin.minimal_uint_ctype(0)   == "uint8_t"
        assert cpp_plugin.minimal_uint_ctype(255) == "uint8_t"

    def test_uint16(self):
        assert cpp_plugin.minimal_uint_ctype(256)   == "uint16_t"
        assert cpp_plugin.minimal_uint_ctype(65535) == "uint16_t"

    def test_uint32(self):
        assert cpp_plugin.minimal_uint_ctype(65536)    == "uint32_t"
        assert cpp_plugin.minimal_uint_ctype(67108864) == "uint32_t"


class TestParseOptions:
    def test_empty(self):
        assert cpp_plugin.parse_options("") == {}

    def test_single(self):
        assert cpp_plugin.parse_options("ros2_package=foo") == {"ros2_package": "foo"}

    def test_multiple(self):
        result = cpp_plugin.parse_options("a=1,b=2")
        assert result == {"a": "1", "b": "2"}


# --------------------------------------------------------------------------- #
# robot_parameters.proto C++ conversion tests
# --------------------------------------------------------------------------- #

class TestRobotParameters:
    """robot_parameters.proto: C++ conversion header for ParameterCommand."""

    def test_robot_parameters_header_generated(self):
        code, err, generated = run_cpp_plugin(["robot_parameters.proto"])
        assert code == 0, f"protoc failed:\n{err}"
        assert "robot_parameters_conversions.hpp" in generated

    def test_parameter_command_function_signature(self):
        code, err, generated = run_cpp_plugin(["robot_parameters.proto"])
        assert code == 0, f"protoc failed:\n{err}"
        content = generated["robot_parameters_conversions.hpp"]
        assert (
            "inline ateam_radio_msgs::msg::ParameterCommand "
            "fromProto(const ateam::ParameterCommand& p)"
            in content
        )

    def test_command_code_field_assigned(self):
        """command_code is an enum → static_cast<int32_t>."""
        code, err, generated = run_cpp_plugin(["robot_parameters.proto"])
        assert code == 0, f"protoc failed:\n{err}"
        content = generated["robot_parameters_conversions.hpp"]
        assert "msg.command_code = static_cast<int32_t>(p.command_code());" in content

    def test_parameter_name_field_assigned(self):
        code, err, generated = run_cpp_plugin(["robot_parameters.proto"])
        assert code == 0, f"protoc failed:\n{err}"
        content = generated["robot_parameters_conversions.hpp"]
        assert "msg.parameter_name = static_cast<int32_t>(p.parameter_name());" in content

    def test_repeated_float_data_assigned(self):
        """repeated float → push_back loop."""
        code, err, generated = run_cpp_plugin(["robot_parameters.proto"])
        assert code == 0, f"protoc failed:\n{err}"
        content = generated["robot_parameters_conversions.hpp"]
        assert "for (const auto& v : p.data()) msg.data.push_back(v);" in content


# --------------------------------------------------------------------------- #
# radio.proto C++ conversion tests
# --------------------------------------------------------------------------- #

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
_RADIO_DEPS = _ALL_PROTOS


class TestRadioPacket:
    """radio.proto: C++ conversion header for RadioPacket and Keepalive."""

    def test_radio_conversions_header_generated(self):
        code, err, generated = run_cpp_plugin(_RADIO_DEPS)
        assert code == 0, f"protoc failed:\n{err}"
        assert "radio_conversions.hpp" in generated

    def test_radio_packet_function_signature(self):
        code, err, generated = run_cpp_plugin(_RADIO_DEPS)
        assert code == 0, f"protoc failed:\n{err}"
        content = generated["radio_conversions.hpp"]
        assert (
            "inline ateam_radio_msgs::msg::RadioPacket "
            "fromProto(const ateam::RadioPacket& p)"
            in content
        )

    def test_keepalive_function_generated(self):
        code, err, generated = run_cpp_plugin(_RADIO_DEPS)
        assert code == 0, f"protoc failed:\n{err}"
        content = generated["radio_conversions.hpp"]
        assert (
            "inline ateam_radio_msgs::msg::Keepalive "
            "fromProto(const ateam::Keepalive& p)"
            in content
        )

    def test_oneof_arms_use_has_check(self):
        """Each oneof arm must use p.has_<arm>() before converting."""
        code, err, generated = run_cpp_plugin(_RADIO_DEPS)
        assert code == 0, f"protoc failed:\n{err}"
        content = generated["radio_conversions.hpp"]
        assert "if (p.has_control()) msg.control = fromProto(p.control());" in content
        assert "if (p.has_telemetry()) msg.telemetry = fromProto(p.telemetry());" in content
        assert "if (p.has_keepalive()) msg.keepalive = fromProto(p.keepalive());" in content
        assert "if (p.has_parameter_command()) msg.parameter_command = fromProto(p.parameter_command());" in content

    def test_payload_case_discriminant_assigned(self):
        """payload_case must be assigned from p.payload_case()."""
        code, err, generated = run_cpp_plugin(_RADIO_DEPS)
        assert code == 0, f"protoc failed:\n{err}"
        content = generated["radio_conversions.hpp"]
        assert "msg.payload_case = static_cast<uint8_t>(p.payload_case());" in content

    def test_dependency_includes_in_header(self):
        """radio_conversions.hpp must include each imported proto's conversion header."""
        code, err, generated = run_cpp_plugin(_RADIO_DEPS)
        assert code == 0, f"protoc failed:\n{err}"
        content = generated["radio_conversions.hpp"]
        assert '#include "control_conversions.hpp"' in content
        assert '#include "diagnostics_conversions.hpp"' in content
        assert '#include "discovery_conversions.hpp"' in content
        assert '#include "robot_parameters_conversions.hpp"' in content
        assert '#include "telemetry_conversions.hpp"' in content
