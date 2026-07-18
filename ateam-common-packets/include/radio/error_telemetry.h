#pragma once

#include "../common.h"

typedef struct ErrorTelemetry {
    uint32_t timestamp;
    unsigned char error_message[60];
} ErrorTelemetry;
assert_size(ErrorTelemetry, 64);

