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

#ifndef BPX_TYPES_H
#define BPX_TYPES_H

typedef void* bpx_container_t;
typedef void* bpx_section_t;

#include <stdint.h>
#include <stdlib.h>

typedef uint8_t bpx_u8_t;
typedef uint16_t bpx_u16_t;
typedef uint32_t bpx_u32_t;
typedef uint64_t bpx_u64_t;

typedef int8_t bpx_i8_t;
typedef int16_t bpx_i16_t;
typedef int32_t bpx_i32_t;
typedef int64_t bpx_i64_t;

typedef bpx_u32_t bpx_handle_t;

typedef size_t bpx_size_t;

typedef unsigned int bpx_error_t;

typedef struct bpx_main_header_s
{
    bpx_u8_t signature[3];
    bpx_u8_t ty;
    bpx_u32_t chksum;
    bpx_u64_t file_size;
    bpx_u32_t section_num;
    bpx_u32_t version;
    bpx_u8_t type_ext[16];
} bpx_main_header_t;

typedef struct bpx_section_header_s
{
    bpx_u64_t pointer;
    bpx_u32_t csize;
    bpx_u32_t size;
    bpx_u32_t chksum;
    bpx_u8_t ty;
    bpx_u8_t flags;
} bpx_section_header_t;

#endif
