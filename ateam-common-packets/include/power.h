#pragma once

#include "common.h"

typedef struct PowerStatusPacket {
    uint16_t power_ok : 1;
    uint16_t power_rail_3v3_ok : 1;
    uint16_t power_rail_5v0_ok : 1;
    uint16_t power_rail_12v0_ok : 1;
    uint16_t battery_ok : 1;
    uint16_t battery_balance_connected : 1;
    uint16_t battery_low_warn : 1;
    uint16_t battery_cell_low_warn : 1;
    uint16_t battery_cell_imbalance_warn : 1;
    uint16_t shutdown_requested : 1;
    // add future flags here, decrement reserved
    uint16_t reserved : 6;

    uint16_t battery_mv;
    uint16_t cell1_mv;
    uint16_t cell2_mv;
    uint16_t cell3_mv;
    uint16_t cell4_mv;
    uint16_t cell5_mv;
    uint16_t cell6_mv;

    uint8_t battery_pct;
    uint8_t cell1_pct;
    uint8_t cell2_pct;
    uint8_t cell3_pct;
    uint8_t cell4_pct;
    uint8_t cell5_pct;
    uint8_t cell6_pct;
    uint8_t reserved_2;
} PowerStatusPacket;
assert_size(PowerStatusPacket, 24);

typedef struct PowerCommandPacket {
    uint32_t request_shutdown : 1;
    uint32_t cancel_shutdown : 1;
    uint32_t force_shutdown : 1;
    uint32_t reserved : 29;
} PowerCommnandPacket;