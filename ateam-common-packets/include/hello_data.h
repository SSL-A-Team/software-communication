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

#include <stdint.h>

// if C11 or newer is used, include assert_static macro for testing
#if defined(__cplusplus) || (defined( __STDC_VERSION__) && __STDC_VERSION__ >= 201112L)
#include <assert.h>
#endif

typedef enum TeamColor : unsigned char {
    TC_YELLOW = 0,
    TC_BLUE = 1
} TeamColor_t;
#if defined(__cplusplus) || (defined( __STDC_VERSION__) && __STDC_VERSION__ >= 201112L)
static_assert(sizeof(TeamColor_t) == 1, "Expected TeamColor_t to have a size of 1");
#endif

typedef struct HelloData {
    uint8_t robot_id;
    TeamColor_t color;
} HelloData_t;
#if defined(__cplusplus) || (defined( __STDC_VERSION__) && __STDC_VERSION__ >= 201112L)
static_assert(sizeof(HelloData_t) == 2, "Expected HelloData_t to have a size of 2");
#endif
