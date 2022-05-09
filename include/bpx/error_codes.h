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

#ifndef BPX_ERROR_CODES_H
#define BPX_ERROR_CODES_H

#define BPX_ERR_NONE 0x0
#define BPX_ERR_INVALID_PATH 0x1
#define BPX_ERR_FILE_OPEN 0x2
#define BPX_ERR_FILE_CREATE 0x3

// BPX errors
#define BPX_ERR_CORE_CHKSUM 0x5
#define BPX_ERR_CORE_IO 0x6
#define BPX_ERR_CORE_BAD_VERSION 0x7
#define BPX_ERR_CORE_BAD_SIGNATURE 0x8
#define BPX_ERR_CORE_CAPACITY 0xC

// Inflate errors
#define BPX_ERR_INFLATE_MEMOR 0xD
#define BPX_ERR_INFLATE_UNSUPPORTE 0xE
#define BPX_ERR_INFLATE_DAT 0xF
#define BPX_ERR_INFLATE_UNKNOWN 0x10
#define BPX_ERR_INFLATE_IO 0x11

// Deflate errors
#define BPX_ERR_DEFLATE_MEMORY 0x12
#define BPX_ERR_DEFLATE_UNSUPPORTED 0x13
#define BPX_ERR_DEFLATE_DATA 0x14
#define BPX_ERR_DEFLATE_UNKNOWN 0x15
#define BPX_ERR_DEFLATE_IO 0x16

// Open errors
#define BPX_ERR_OPEN_SECTION_IN_USE 0x17
#define BPX_ERR_OPEN_SECTION_NOT_LOADED 0x18

// BPXSD errors
#define BPX_ERR_SD_IO 0x19
#define BPX_ERR_SD_TRUNCATION 0x1A
#define BPX_ERR_SD_BAD_TYPE_CODE 0x1B
#define BPX_ERR_SD_UTF8 0x1C
#define BPX_ERR_SD_CAPACITY_EXCEEDED 0x1D
#define BPX_ERR_SD_NOT_AN_OBJECT 0x1E

#endif
