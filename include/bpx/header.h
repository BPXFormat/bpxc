// Copyright (c) 2021, BlockProject 3D
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

#ifndef BPX_HEADER_H
#define BPX_HEADER_H

#include "bpx/common.h"

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

#define BPX_FLAG_COMPRESS_XZ 0x2
#define BPX_FLAG_CHECK_WEAK 0x8
#define BPX_FLAG_COMPRESS_ZLIB 0x1
#define BPX_FLAG_CHECK_CRC32 0x4

#define BPX_SECTION_TYPE_STRING 0xFF
#define BPX_SECTION_TYPE_SD 0xFE

#define BPX_CURRENT_VERSION 0x2

#define BPX_KNOWN_VERSIONS [0x1, 0x2]

#endif