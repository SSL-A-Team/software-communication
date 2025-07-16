/**
 * @file error_telemetry.h
 * @author Nicholas Witten / Joseph Spall
 * @brief Simple error messages over radio
 * @version 0.1
 * 
 * @copyright Copyright (c) 2022
 *
 */

#pragma once

#include "common.h"

typedef struct ErrorTelemetry {
    uint32_t timestamp;
    unsigned char error_message[60];
} ErrorTelemetry;
assert_size(ErrorTelemetry, 64);

