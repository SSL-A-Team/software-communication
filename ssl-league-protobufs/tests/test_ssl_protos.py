"""
Integration tests: run the protoc plugins against the SSL league protobufs.

These protos are third-party (RoboCup SSL), use proto2 syntax, and exercise
features that the ateam protos do not: nested messages/enums, map<K,V> fields,
required/optional labels, and large oneof blocks.

Run with:  pytest ssl-league-protobufs/tests/test_ssl_protos.py -v
"""

import subprocess
import sys
import tempfile
from pathlib import Path

# --------------------------------------------------------------------------- #
# Paths
# --------------------------------------------------------------------------- #

_THIS_DIR   = Path(__file__).parent
_REPO_ROOT  = _THIS_DIR.parent.parent
_PROTO_DIR  = _THIS_DIR.parent / "proto"
_PLUGIN_MSG = _REPO_ROOT / "ateam-common-packets" / "cmake" / "protoc_gen_ros2msg.py"
_PLUGIN_CPP = _REPO_ROOT / "ateam-common-packets" / "cmake" / "protoc_gen_ros2cpp.py"

_ALL_SSL_PROTOS = sorted(_PROTO_DIR.glob("*.proto"))

_DEFAULT_CPP_OPTS = (
    "proto_include_prefix=ssl_league_protobufs,"
    "ros2_package=ssl_league_msgs,"
    "namespace=ssl_conversions"
)

# --------------------------------------------------------------------------- #
# Helpers
# --------------------------------------------------------------------------- #

def _run(
    plugin: Path,
    proto_files: list[Path],
    out_glob: str,
    opts: str,
    extra_proto_path: Path | None = None,
) -> tuple[int, str, dict[str, str]]:
    with tempfile.TemporaryDirectory() as tmp:
        tmp_path = Path(tmp)
        plugin_name = plugin.stem.removeprefix("protoc_gen_")
        cmd = [
            "protoc",
            f"--plugin=protoc-gen-{plugin_name}={plugin}",
            f"--{plugin_name}_opt={opts}",
            f"--{plugin_name}_out={tmp_path}",
            f"--proto_path={_PROTO_DIR}",
        ]
        if extra_proto_path:
            cmd.append(f"--proto_path={extra_proto_path}")
        cmd.extend(str(p) for p in proto_files)
        result = subprocess.run(cmd, capture_output=True, text=True)
        files = {p.name: p.read_text() for p in tmp_path.glob(out_glob)}
        return result.returncode, result.stderr, files


def run_msg_plugin(
    proto_files: list[Path] | None = None,
    opts: str = "optional_submsg=has_field",
) -> tuple[int, str, dict[str, str]]:
    return _run(_PLUGIN_MSG, proto_files or _ALL_SSL_PROTOS, "*.msg", opts)


def run_cpp_plugin(
    proto_files: list[Path] | None = None,
    opts: str = _DEFAULT_CPP_OPTS,
) -> tuple[int, str, dict[str, str]]:
    return _run(_PLUGIN_CPP, proto_files or _ALL_SSL_PROTOS, "*.hpp", opts)


# --------------------------------------------------------------------------- #
# .msg plugin: smoke tests
# --------------------------------------------------------------------------- #

class TestSslMsgGeneration:
    """All SSL league protos generate .msg files without error."""

    def test_all_protos_succeed(self):
        code, err, _ = run_msg_plugin()
        assert code == 0, f"protoc failed:\n{err}"

    def test_generates_files_for_each_input(self):
        """Each input proto contributes at least one .msg file."""
        code, err, generated = run_msg_plugin()
        assert code == 0, f"protoc failed:\n{err}"
        assert len(generated) > 0

    def test_vision_detection_msgs(self):
        """ssl_vision_detection.proto → SSL_DetectionFrame, SSL_DetectionBall, etc."""
        code, err, generated = run_msg_plugin([_PROTO_DIR / "ssl_vision_detection.proto"])
        assert code == 0, f"protoc failed:\n{err}"
        assert "SSL_DetectionFrame.msg" in generated
        assert "SSL_DetectionBall.msg" in generated
        assert "SSL_DetectionRobot.msg" in generated

    def test_vision_geometry_msgs(self):
        code, err, generated = run_msg_plugin([
            _PROTO_DIR / "ssl_vision_geometry.proto",
        ])
        assert code == 0, f"protoc failed:\n{err}"
        assert "SSL_GeometryData.msg" in generated
        assert "SSL_GeometryFieldSize.msg" in generated

    def test_simulation_robot_control_msgs(self):
        code, err, generated = run_msg_plugin([
            _PROTO_DIR / "ssl_simulation_robot_control.proto",
        ])
        assert code == 0, f"protoc failed:\n{err}"
        assert "RobotControl.msg" in generated


# --------------------------------------------------------------------------- #
# .msg plugin: nested type flattening
# --------------------------------------------------------------------------- #

class TestSslNestedTypeFlattening:
    """Nested proto messages produce flat .msg names (ParentMsg_NestedMsg)."""

    def test_game_event_nested_types_flattened(self):
        """GameEvent.BallLeftField → GameEvent_BallLeftField.msg"""
        code, err, generated = run_msg_plugin([
            _PROTO_DIR / "ssl_gc_game_event.proto",
            _PROTO_DIR / "ssl_gc_common.proto",
            _PROTO_DIR / "ssl_gc_geometry.proto",
        ])
        assert code == 0, f"protoc failed:\n{err}"
        assert "GameEvent_BallLeftField.msg" in generated
        assert "GameEvent_Goal.msg" in generated

    def test_nested_type_reference_uses_flat_name(self):
        """GameEvent.msg references nested arm types by their flattened name."""
        code, err, generated = run_msg_plugin([
            _PROTO_DIR / "ssl_gc_game_event.proto",
            _PROTO_DIR / "ssl_gc_common.proto",
            _PROTO_DIR / "ssl_gc_geometry.proto",
        ])
        assert code == 0, f"protoc failed:\n{err}"
        content = generated["GameEvent.msg"]
        # The oneof arms reference nested message types; these must use flat names
        assert "GameEvent_BallLeftField ball_left_field_touch_line" in content
        assert "GameEvent_Goal goal" in content

    def test_referee_nested_team_info_flattened(self):
        """Referee.TeamInfo → Referee_TeamInfo.msg"""
        code, err, generated = run_msg_plugin([
            _PROTO_DIR / "ssl_gc_referee_message.proto",
            _PROTO_DIR / "ssl_gc_common.proto",
            _PROTO_DIR / "ssl_gc_geometry.proto",
            _PROTO_DIR / "ssl_gc_game_event.proto",
        ])
        assert code == 0, f"protoc failed:\n{err}"
        assert "Referee_TeamInfo.msg" in generated

    def test_nested_enum_flattened(self):
        """Referee.Stage (nested enum) → Referee_Stage.msg"""
        code, err, generated = run_msg_plugin([
            _PROTO_DIR / "ssl_gc_referee_message.proto",
            _PROTO_DIR / "ssl_gc_common.proto",
            _PROTO_DIR / "ssl_gc_geometry.proto",
            _PROTO_DIR / "ssl_gc_game_event.proto",
        ])
        assert code == 0, f"protoc failed:\n{err}"
        assert "Referee_Stage.msg" in generated


# --------------------------------------------------------------------------- #
# .msg plugin: map<> field handling
# --------------------------------------------------------------------------- #

class TestSslMapFields:
    """map<K,V> fields are silently skipped (no ROS2 map type)."""

    def test_config_generates_without_error(self):
        """ssl_gc_engine_config.proto has map<> fields — must not crash."""
        code, err, generated = run_msg_plugin([
            _PROTO_DIR / "ssl_gc_engine_config.proto",
            _PROTO_DIR / "ssl_gc_game_event.proto",
            _PROTO_DIR / "ssl_gc_common.proto",
            _PROTO_DIR / "ssl_gc_geometry.proto",
        ])
        assert code == 0, f"protoc failed:\n{err}"

    def test_map_field_not_in_output(self):
        """The map field name 'game_event_behavior' must not appear in the .msg."""
        code, err, generated = run_msg_plugin([
            _PROTO_DIR / "ssl_gc_engine_config.proto",
            _PROTO_DIR / "ssl_gc_game_event.proto",
            _PROTO_DIR / "ssl_gc_common.proto",
            _PROTO_DIR / "ssl_gc_geometry.proto",
        ])
        assert code == 0, f"protoc failed:\n{err}"
        # The map field should be absent; the message file itself should exist
        config_msg = generated.get("Config.msg", "")
        assert config_msg != "", "Config.msg should be generated"
        assert "game_event_behavior" not in config_msg

    def test_map_entry_type_not_in_output(self):
        """Synthetic map-entry message types must not produce .msg files."""
        code, err, generated = run_msg_plugin([
            _PROTO_DIR / "ssl_gc_engine_config.proto",
            _PROTO_DIR / "ssl_gc_game_event.proto",
            _PROTO_DIR / "ssl_gc_common.proto",
            _PROTO_DIR / "ssl_gc_geometry.proto",
        ])
        assert code == 0, f"protoc failed:\n{err}"
        # Synthetic names end in 'Entry' by protoc convention
        entry_files = [f for f in generated if "Entry" in f]
        assert entry_files == [], f"Unexpected map-entry .msg files: {entry_files}"


# --------------------------------------------------------------------------- #
# .msg plugin: proto2-specific features
# --------------------------------------------------------------------------- #

class TestSslProto2:
    """Proto2 required/optional labels do not break generation."""

    def test_required_fields_generate_cleanly(self):
        """ssl_vision_detection.proto uses 'required' — must generate without error."""
        code, err, generated = run_msg_plugin([_PROTO_DIR / "ssl_vision_detection.proto"])
        assert code == 0, f"protoc failed:\n{err}"
        content = generated["SSL_DetectionBall.msg"]
        assert "float32 confidence" in content
        assert "float32 x" in content

    def test_optional_scalar_generates_as_field(self):
        """Proto2 'optional' scalars emit as plain fields (ROS2 has no optional scalar)."""
        code, err, generated = run_msg_plugin([_PROTO_DIR / "ssl_vision_detection.proto"])
        assert code == 0, f"protoc failed:\n{err}"
        content = generated["SSL_DetectionBall.msg"]
        # area is optional in proto2; should appear as plain uint32 field
        assert "uint32 area" in content

    def test_optional_message_gets_has_flag(self):
        """Proto2 'optional' message-type field gets bool has_ sentinel."""
        code, err, generated = run_msg_plugin([
            _PROTO_DIR / "ssl_vision_geometry.proto",
        ])
        assert code == 0, f"protoc failed:\n{err}"
        content = generated["SSL_GeometryData.msg"]
        # models is optional SSL_GeometryModels field
        assert "bool has_models" in content

    def test_large_oneof_generates(self):
        """GameEvent oneof with 40+ arms must generate all case constants."""
        code, err, generated = run_msg_plugin([
            _PROTO_DIR / "ssl_gc_game_event.proto",
            _PROTO_DIR / "ssl_gc_common.proto",
            _PROTO_DIR / "ssl_gc_geometry.proto",
        ])
        assert code == 0, f"protoc failed:\n{err}"
        content = generated["GameEvent.msg"]
        assert "uint8 event_case" in content
        assert "uint8 ONEOF_EVENT_NONE=0" in content
        # ball_left_field_touch_line = 6 in the proto
        assert "uint8 ONEOF_EVENT_BALL_LEFT_FIELD_TOUCH_LINE=6" in content


# --------------------------------------------------------------------------- #
# C++ plugin: smoke tests
# --------------------------------------------------------------------------- #

class TestSslCppGeneration:
    """All SSL league protos generate C++ conversion headers without error."""

    def test_all_protos_succeed(self):
        code, err, _ = run_cpp_plugin()
        assert code == 0, f"protoc failed:\n{err}"

    def test_generates_header_per_proto(self):
        code, err, generated = run_cpp_plugin()
        assert code == 0, f"protoc failed:\n{err}"
        assert "ssl_vision_detection_conversions.hpp" in generated
        assert "ssl_gc_game_event_conversions.hpp" in generated

    def test_detection_frame_function_signature(self):
        code, err, generated = run_cpp_plugin([_PROTO_DIR / "ssl_vision_detection.proto"])
        assert code == 0, f"protoc failed:\n{err}"
        content = generated["ssl_vision_detection_conversions.hpp"]
        assert (
            "inline ssl_league_msgs::msg::SSL_DetectionFrame "
            "fromProto(const SSL_DetectionFrame& p)"
            in content
        )

    def test_nested_type_function_signature(self):
        """GameEvent_BallLeftField generates a correctly-typed fromProto function."""
        code, err, generated = run_cpp_plugin([
            _PROTO_DIR / "ssl_gc_game_event.proto",
            _PROTO_DIR / "ssl_gc_common.proto",
            _PROTO_DIR / "ssl_gc_geometry.proto",
        ])
        assert code == 0, f"protoc failed:\n{err}"
        content = generated["ssl_gc_game_event_conversions.hpp"]
        # Proto C++ class for nested type: GameEvent_BallLeftField (no package)
        assert (
            "inline ssl_league_msgs::msg::GameEvent_BallLeftField "
            "fromProto(const GameEvent_BallLeftField& p)"
            in content
        )

    def test_map_field_not_in_cpp_output(self):
        """Map fields must be absent from C++ conversion bodies."""
        code, err, generated = run_cpp_plugin([
            _PROTO_DIR / "ssl_gc_engine_config.proto",
            _PROTO_DIR / "ssl_gc_game_event.proto",
            _PROTO_DIR / "ssl_gc_common.proto",
            _PROTO_DIR / "ssl_gc_geometry.proto",
        ])
        assert code == 0, f"protoc failed:\n{err}"
        content = generated["ssl_gc_engine_config_conversions.hpp"]
        assert "game_event_behavior" not in content

    def test_required_field_direct_assign(self):
        """Proto2 'required' scalar fields emit direct assignment."""
        code, err, generated = run_cpp_plugin([_PROTO_DIR / "ssl_vision_detection.proto"])
        assert code == 0, f"protoc failed:\n{err}"
        content = generated["ssl_vision_detection_conversions.hpp"]
        assert "msg.confidence = p.confidence();" in content
        assert "msg.x = p.x();" in content

    def test_dependency_include_in_header(self):
        """ssl_gc_game_event_conversions.hpp includes ssl_gc_common_conversions.hpp."""
        code, err, generated = run_cpp_plugin([
            _PROTO_DIR / "ssl_gc_game_event.proto",
            _PROTO_DIR / "ssl_gc_common.proto",
            _PROTO_DIR / "ssl_gc_geometry.proto",
        ])
        assert code == 0, f"protoc failed:\n{err}"
        content = generated["ssl_gc_game_event_conversions.hpp"]
        assert '#include "ssl_gc_common_conversions.hpp"' in content
        assert '#include "ssl_gc_geometry_conversions.hpp"' in content
