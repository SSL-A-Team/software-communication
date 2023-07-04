#pragma once

#include <stdint.h>

#if defined(__cplusplus) || (defined( __STDC_VERSION__) && __STDC_VERSION__ >= 201112L)
    #if defined(__ARM_ARCH_7EM__) || defined(__ARM_ARCH_6M__)
        #define static_assert _Static_assert
    #else
        #include <assert.h>
    #endif
#else
    #define static_assert(args...)
#endif

#define assert_size(type, size) static_assert(sizeof(type) == size, "Expected " #type " to have a size of " #size)

// This is only used for generating an error that shows the actual size of given type
#define error_size(type) static_assert((char[sizeof(type)])"");