// Copyright (c) 2022, BlockProject 3D
//
// All rights reserved.
//
// Redistribution and use in source and binary forms, with or without modification,
// are permitted provided that the following conditions are met:
//
//     * Redistributions of source code must retain the above copyright notice,
//       this list of conditions and the following disclaimer.
//     * Redistributions in binary form must reproduce the above copyright notice,
//       this list of conditions and the following disclaimer in the documentation
//       and/or other materials provided with the distribution.
//     * Neither the name of BlockProject 3D nor the names of its contributors
//       may be used to endorse or promote products derived from this software
//       without specific prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
// "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
// LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
// A PARTICULAR PURPOSE ARE DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT OWNER OR
// CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
// EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
// PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
// PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF
// LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING
// NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
// SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

#ifndef BPX_SD_H
#define BPX_SD_H

#include "bpx/types.h"
#include <stdbool.h>

typedef void* bpx_sd_array_t;
typedef void* bpx_sd_object_t;

enum bpx_sd_value_type_e
{
    BPX_SD_VALUE_TYPE_NULL,
    BPX_SD_VALUE_TYPE_BOOL,
    BPX_SD_VALUE_TYPE_UINT8,
    BPX_SD_VALUE_TYPE_UINT16,
    BPX_SD_VALUE_TYPE_UINT32,
    BPX_SD_VALUE_TYPE_UINT64,
    BPX_SD_VALUE_TYPE_INT8,
    BPX_SD_VALUE_TYPE_INT16,
    BPX_SD_VALUE_TYPE_INT32,
    BPX_SD_VALUE_TYPE_INT64,
    BPX_SD_VALUE_TYPE_FLOAT,
    BPX_SD_VALUE_TYPE_DOUBLE,
    BPX_SD_VALUE_TYPE_STRING,
    BPX_SD_VALUE_TYPE_ARRAY,
    BPX_SD_VALUE_TYPE_OBJECT
};

union bpx_sd_value_data_u {
    bool as_bool;
    bpx_u8_t as_u8;
    bpx_u16_t as_u16;
    bpx_u32_t as_u32;
    bpx_u64_t as_u64;
    bpx_i8_t as_i8;
    bpx_i16_t as_i16;
    bpx_i32_t as_i32;
    bpx_i64_t as_i64;
    float as_float;
    double as_double;
    char *as_string; //Always guaranteed to be null-terminated UTF-8.
    bpx_sd_array_t as_array;
    bpx_sd_object_t as_object;
};

typedef struct bpx_sd_value_s {
    enum bpx_sd_value_type_e type;
    union bpx_sd_value_data_u data;
} bpx_sd_value_t;

typedef struct bpx_sd_object_entry_s {
    bpx_u64_t hash;
    bpx_sd_value_t value;
} bpx_sd_object_entry_t;

bpx_error_t bpx_sd_value_decode_section(bpx_section_t section, bpx_sd_value_t *out);
bpx_error_t bpx_sd_value_decode_memory(const bpx_u8_t *buffer, bpx_size_t size, bpx_sd_value_t *out);
bpx_error_t bpx_sd_value_encode(bpx_section_t section, const bpx_sd_value_t *value);

bpx_sd_value_t bpx_sd_value_new();
bpx_sd_value_t bpx_sd_value_new_bool(bool value);
bpx_sd_value_t bpx_sd_value_new_u8(bpx_u8_t value);
bpx_sd_value_t bpx_sd_value_new_u16(bpx_u16_t value);
bpx_sd_value_t bpx_sd_value_new_u32(bpx_u32_t value);
bpx_sd_value_t bpx_sd_value_new_u64(bpx_u64_t value);
bpx_sd_value_t bpx_sd_value_new_i8(bpx_i8_t value);
bpx_sd_value_t bpx_sd_value_new_i16(bpx_i16_t value);
bpx_sd_value_t bpx_sd_value_new_i32(bpx_i32_t value);
bpx_sd_value_t bpx_sd_value_new_i64(bpx_i64_t value);
bpx_sd_value_t bpx_sd_value_new_float(float value);
bpx_sd_value_t bpx_sd_value_new_double(double value);
bpx_sd_value_t bpx_sd_value_new_string(const char *value);
bpx_sd_value_t bpx_sd_value_new_array();
bpx_sd_value_t bpx_sd_value_new_object();
void bpx_sd_value_free(bpx_sd_value_t **value);

void bpx_sd_array_push(bpx_sd_array_t array, bpx_sd_value_t *value); //Takes ownership of value.
void bpx_sd_array_insert(bpx_sd_array_t array, bpx_sd_value_t *value, bpx_size_t index); //Takes ownership of value.
void bpx_sd_array_remove(bpx_sd_array_t array, bpx_size_t index);
bpx_sd_value_t bpx_sd_array_get(bpx_sd_array_t array, bpx_size_t index);
bpx_size_t bpx_sd_array_len(bpx_sd_array_t array);
void bpx_sd_array_list(bpx_sd_array_t array, bpx_sd_value_t *out);

bpx_sd_value_t bpx_sd_object_get(bpx_sd_object_t object, const char *key);
void bpx_sd_object_set(bpx_sd_object_t object, const char *key, bpx_sd_value_t *value); //Takes ownership of value.
bpx_sd_value_t bpx_sd_object_rawget(bpx_sd_object_t object, bpx_u64_t hash);
void bpx_sd_object_rawset(bpx_sd_object_t object, bpx_u64_t hash, bpx_sd_value_t *value); //Takes ownership of value.
bpx_size_t bpx_sd_object_len(bpx_sd_object_t object);
void bpx_sd_object_list(bpx_sd_object_t object, bpx_sd_object_entry_t *out);

#endif
