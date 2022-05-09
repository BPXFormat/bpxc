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

#ifndef BPX_OPEN2_H
#define BPX_OPEN2_H

#include "bpx/open.h"

typedef enum bpx_seek_from_e
{
    BPX_SEEK_START,
    BPX_SEEK_END,
    BPX_SEEK_CURRENT
} bpx_seek_from_t;

typedef struct bpx_container_io_s
{
    const void *userdata;
    //Return true for success, false otherwise
    /*@NotNull*/ bpx_error_t (*seek) (const void *userdata, bpx_seek_from_t from, bpx_u64_t pos, bpx_u64_t *new_pos);
    /*@NotNull*/ bpx_error_t (*read) (const void *userdata, bpx_u8_t *buffer, size_t size, size_t *bytes_read);
    bpx_error_t (*write) (const void *userdata, const bpx_u8_t *buffer, size_t size, size_t *bytes_written);
    bpx_error_t (*flush) (const void *userdata);
} bpx_container_io_t;

bpx_error_t bpx_container_open2(bpx_container_io_t io, bpx_container_t *out);
bpx_error_t bpx_container_create2(bpx_container_io_t io, const bpx_container_options_t *header, bpx_container_t *out);

#endif
