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

#ifndef BPX_SECTION_H
#define BPX_SECTION_H

#include "bpx/types.h"

void bpx_section_get_header(bpx_container_t container, bpx_handle_t handle, bpx_section_header_t *section_header);
bpx_error_t bpx_section_open(bpx_container_t container, bpx_handle_t handle, bpx_section_t *out);
bpx_error_t bpx_section_load(bpx_container_t container, bpx_handle_t handle, bpx_section_t *out);
bpx_size_t bpx_section_read(bpx_section_t section, bpx_u8_t *buffer, bpx_size_t size);
bpx_size_t bpx_section_write(bpx_section_t section, const bpx_u8_t *buffer, bpx_size_t size);
bpx_u64_t bpx_section_seek(bpx_section_t section, bpx_u64_t pos);
int bpx_section_flush(bpx_section_t section);
void bpx_section_close(bpx_section_t *section);

#endif
