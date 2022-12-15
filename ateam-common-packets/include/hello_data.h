/**
 * @file hello_data.h
 * @author Matthew Barulic
 * @brief definition for Hello Data data type
 * @version 0.1
 * 
 * @copyright Copyright (c) 2022
 *
 */

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
} HelloRequest;
assert_size(HelloRequest, 2);

typedef struct HelloResponse {
    uint8_t ipv4[4];
    uint16_t port;
} HelloResponse;
assert_size(HelloResponse, 6);
