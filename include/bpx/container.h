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

#ifndef BPX_CONTAINER_H
#define BPX_CONTAINER_H

#include "bpx/types.h"

#include <stdbool.h>

typedef struct bpx_section_options_s
{
    bpx_u32_t size;
    bpx_u8_t ty;
    bpx_u8_t flags;
    bpx_u32_t threshold;
} bpx_section_options_t;

#define BPX_COMPRESSION_ZLIB 0x1
#define BPX_COMPRESSION_XZ 0x2
#define BPX_COMPRESSION_THRESHOLD 0x10
#define BPX_CHECKSUM_WEAK 0x4
#define BPX_CHECKSUM_CRC32 0x8

void bpx_container_get_main_header(bpx_container_t container, bpx_main_header_t *main_header);
void bpx_container_list_sections(bpx_container_t container, bpx_handle_t *out, size_t size);
bool bpx_container_find_section_by_type(bpx_container_t container, bpx_u8_t ty, bpx_handle_t *handle);
bool bpx_container_find_section_by_index(bpx_container_t container, bpx_u32_t idx, bpx_handle_t *handle);
void bpx_container_create_section(bpx_container_t container, const bpx_section_options_t *options);

bpx_error_t bpx_container_save(bpx_container_t container);

void bpx_container_close(bpx_container_t *container);

#endif
