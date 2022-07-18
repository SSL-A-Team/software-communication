/**
 * @file stspin_c_static_checks.h
 * @author Will Stuckey
 * @brief contains the static checks seemingly incompatible with Rust bindgen
 * @version 0.1
 * @date 2022-07-17
 * 
 * @copyright Copyright (c) 2022
 * 
 */

#pragma once

#include "stspin.h"

#if defined( __STDC_VERSION__ ) && __STDC_VERSION__ >= 201112L
static_assert(sizeof(MotorResponsePacket_t) == 36, "Expected MotorResponse_Motion_Packet to have a size of 36");
#endif

#if defined( __STDC_VERSION__ ) && __STDC_VERSION__ >= 201112L
static_assert(sizeof(MotorResponse_Motion_Packet_t) == 40, "Expected MotorResponse_Motion_Packet to have a size of 40");
#endif

#if defined( __STDC_VERSION__ ) && __STDC_VERSION__ >= 201112L
static_assert(sizeof(MotorResponsePacket_t) == 48, "Expected MotorResponse_Motion_Packet to have a size of 48");
#endif