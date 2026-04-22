#pragma once

#include "common.h"

typedef enum TeamColor : uint8_t {
    TC_YELLOW = 0,
    TC_BLUE = 1
} TeamColor;
assert_size(TeamColor, 1);

typedef struct HelloRequest {
    uint8_t robot_id;
    TeamColor color;

    uint8_t coms_repo_dirty : 1;
    uint8_t controls_repo_dirty : 1;
    uint8_t firmware_repo_dirty : 1;
    uint8_t bitfield_reserved1 : 5;
    uint8_t reserved[1];

    uint8_t coms_hash[4];
    uint8_t controls_hash[4];
    uint8_t firmware_hash[4];
} HelloRequest;
assert_size(HelloRequest, 16);

typedef struct HelloResponse {
    uint8_t ipv4[4];
    uint16_t port;
} HelloResponse;
assert_size(HelloResponse, 6);
