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

use std::os::raw::c_uint;
use bpx::core::error::{ReadError, WriteError};

// No error
pub const ERR_NONE: c_uint = 0x0;

// Local errors
#[cfg(windows)]
pub const ERR_INVALID_PATH: c_uint = 0x1;
pub const ERR_FILE_OPEN: c_uint = 0x2;
pub const ERR_FILE_CREATE: c_uint = 0x3;
pub const ERR_SECTION_IO: c_uint = 0x4;

// BPX errors
pub const ERR_CORE_CHKSUM: c_uint = 0x5;
pub const ERR_CORE_IO: c_uint = 0x6;
pub const ERR_CORE_BAD_VERSION: c_uint = 0x7;
pub const ERR_CORE_BAD_SIGNATURE: c_uint = 0x8;
pub const ERR_CORE_INFLATE: c_uint = 0x9;
pub const ERR_CORE_DEFLATE: c_uint = 0xA;
pub const ERR_CORE_SECTION_NOT_LOADED: c_uint = 0xB;
pub const ERR_CORE_CAPACITY: c_uint = 0xC;

pub trait CErrCode
{
    fn cerr_code(&self) -> u32;
}

impl CErrCode for bpx::core::error::ReadError
{
    fn cerr_code(&self) -> u32
    {
        match self {
            ReadError::Checksum(_, _) => ERR_CORE_CHKSUM,
            ReadError::Io(_) => ERR_CORE_IO,
            ReadError::BadVersion(_) => ERR_CORE_BAD_VERSION,
            ReadError::BadSignature(_) => ERR_CORE_BAD_SIGNATURE,
            ReadError::Inflate(_) => ERR_CORE_INFLATE
        }
    }
}

impl CErrCode for bpx::core::error::WriteError
{
    fn cerr_code(&self) -> u32
    {
        match self {
            WriteError::Io(_) => ERR_CORE_IO,
            WriteError::Capacity(_) => ERR_CORE_CAPACITY,
            WriteError::Deflate(_) => ERR_CORE_DEFLATE,
            WriteError::SectionNotLoaded => ERR_CORE_SECTION_NOT_LOADED
        }
    }
}

macro_rules! unwrap_or_err {
    ($e: expr) => {
        match $e {
            Err(code) => return code,
            Ok(v) => v
        }
    };
}

pub(crate) use unwrap_or_err;
