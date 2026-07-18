# Ros2CppConvertGen.cmake
#
# Provides generate_ros2_cpp_conversions() — runs the protoc ros2cpp plugin at
# CMake configure time and returns the list of generated .hpp headers.
#
# Usage:
#   generate_ros2_cpp_conversions(
#     PROTO_FILES          path/to/a.proto path/to/b.proto ...
#     PROTO_PATHS          path/to/proto/include/dir ...
#     PROTO_INCLUDE_PREFIX ateam_common_packets   # prefix for #include <prefix/stem.pb.h>
#     ROS2_PACKAGE         ateam_radio_msgs       # ROS2 package name
#     OUTPUT_DIR           ${CMAKE_CURRENT_BINARY_DIR}/include/conversions  # optional
#     NAMESPACE            ateam_conversions       # optional, default: ateam_conversions
#     OPTIONAL_SUBMSG      HAS_FIELD               # or ERROR; default HAS_FIELD
#   )
#   # After the call, ${GENERATED_ROS2_CPP_HEADERS} contains the .hpp file list.
#   target_include_directories(my_target PUBLIC ${OUTPUT_DIR})
#
# Each output file is named <proto_stem>_conversions.hpp and contains inline
# fromProto(const pkg::MsgType&) → ros2pkg::msg::MsgType functions.
#
# If a proto file imports another proto file, the generated header will include
# the corresponding <dep_stem>_conversions.hpp; all conversion headers should be
# generated into the same OUTPUT_DIR.
#
# Generation runs at configure time.  CMake re-runs automatically when any
# PROTO_FILES changes (CMAKE_CONFIGURE_DEPENDS).

cmake_minimum_required(VERSION 3.16)

function(generate_ros2_cpp_conversions)
  cmake_parse_arguments(
    _ARG
    ""
    "OUTPUT_DIR;PROTO_INCLUDE_PREFIX;ROS2_PACKAGE;NAMESPACE;OPTIONAL_SUBMSG"
    "PROTO_FILES;PROTO_PATHS"
    ${ARGN}
  )

  # --- Validate arguments ---
  if(NOT _ARG_PROTO_FILES)
    message(FATAL_ERROR "generate_ros2_cpp_conversions: PROTO_FILES is required")
  endif()

  if(NOT _ARG_PROTO_INCLUDE_PREFIX)
    message(FATAL_ERROR "generate_ros2_cpp_conversions: PROTO_INCLUDE_PREFIX is required")
  endif()

  if(NOT _ARG_ROS2_PACKAGE)
    message(FATAL_ERROR "generate_ros2_cpp_conversions: ROS2_PACKAGE is required")
  endif()

  # --- Defaults ---
  if(NOT _ARG_OUTPUT_DIR)
    set(_ARG_OUTPUT_DIR "${CMAKE_CURRENT_BINARY_DIR}/ros2_cpp_conversions")
  endif()

  if(NOT _ARG_NAMESPACE)
    set(_ARG_NAMESPACE "ateam_conversions")
  endif()

  if(NOT _ARG_OPTIONAL_SUBMSG)
    set(_ARG_OPTIONAL_SUBMSG "HAS_FIELD")
  endif()
  string(TOLOWER "${_ARG_OPTIONAL_SUBMSG}" _opt_submsg)

  if(NOT _opt_submsg STREQUAL "has_field" AND NOT _opt_submsg STREQUAL "error")
    message(FATAL_ERROR
      "generate_ros2_cpp_conversions: OPTIONAL_SUBMSG must be HAS_FIELD or ERROR, "
      "got '${_ARG_OPTIONAL_SUBMSG}'"
    )
  endif()

  # --- Find tools ---
  find_program(_PROTOC protoc REQUIRED
    DOC "protoc compiler (install via nix: protobuf)"
  )
  find_package(Python3 REQUIRED COMPONENTS Interpreter)

  # --- Plugin path ---
  get_filename_component(_CMAKE_DIR "${CMAKE_CURRENT_LIST_FILE}" DIRECTORY)
  set(_PLUGIN_SRC "${_CMAKE_DIR}/protoc_gen_ros2cpp.py")

  if(NOT EXISTS "${_PLUGIN_SRC}")
    message(FATAL_ERROR
      "generate_ros2_cpp_conversions: plugin not found at ${_PLUGIN_SRC}"
    )
  endif()

  # Generate an executable wrapper in the build tree so we can pass an
  # explicit Python interpreter without relying on the script's shebang.
  set(_PLUGIN_WRAPPER "${CMAKE_BINARY_DIR}/protoc_gen_ros2cpp")
  file(WRITE "${_PLUGIN_WRAPPER}"
    "#!/bin/sh\nset -e\nexec \"${Python3_EXECUTABLE}\" \"${_PLUGIN_SRC}\" \"$@\"\n"
  )
  file(CHMOD "${_PLUGIN_WRAPPER}"
    PERMISSIONS
      OWNER_READ OWNER_WRITE OWNER_EXECUTE
      GROUP_READ GROUP_EXECUTE
      WORLD_READ WORLD_EXECUTE
  )

  # --- Output directory ---
  file(MAKE_DIRECTORY "${_ARG_OUTPUT_DIR}")

  # --- Build --proto_path arguments ---
  set(_proto_path_args)
  foreach(_path ${_ARG_PROTO_PATHS})
    list(APPEND _proto_path_args "--proto_path=${_path}")
  endforeach()

  # Build plugin option string
  set(_plugin_opts
    "proto_include_prefix=${_ARG_PROTO_INCLUDE_PREFIX}"
    "ros2_package=${_ARG_ROS2_PACKAGE}"
    "namespace=${_ARG_NAMESPACE}"
    "optional_submsg=${_opt_submsg}"
  )
  list(JOIN _plugin_opts "," _plugin_opts_str)

  # --- Run protoc at configure time ---
  execute_process(
    COMMAND
      "${_PROTOC}"
      "--plugin=protoc-gen-ros2cpp=${_PLUGIN_WRAPPER}"
      "--ros2cpp_opt=${_plugin_opts_str}"
      "--ros2cpp_out=${_ARG_OUTPUT_DIR}"
      ${_proto_path_args}
      ${_ARG_PROTO_FILES}
    RESULT_VARIABLE _result
    ERROR_VARIABLE  _stderr
    OUTPUT_QUIET
  )

  if(NOT _result EQUAL 0)
    message(FATAL_ERROR
      "generate_ros2_cpp_conversions: protoc failed (exit ${_result}):\n${_stderr}"
    )
  endif()

  # Re-run CMake configure when any proto file changes.
  set_property(
    DIRECTORY APPEND PROPERTY
    CMAKE_CONFIGURE_DEPENDS ${_ARG_PROTO_FILES}
  )

  # Collect results and expose to caller.
  file(GLOB _generated "${_ARG_OUTPUT_DIR}/*.hpp")
  if(NOT _generated)
    message(FATAL_ERROR
      "generate_ros2_cpp_conversions: no .hpp files found in ${_ARG_OUTPUT_DIR} after generation"
    )
  endif()

  set(GENERATED_ROS2_CPP_HEADERS "${_generated}" PARENT_SCOPE)
endfunction()
