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

use std::os::raw::c_uint;
use bpx::core::error::{DeflateError, Error, InflateError, OpenError};

// No error
pub const ERR_NONE: c_uint = 0x0;

// Local errors
#[cfg(windows)]
pub const ERR_INVALID_PATH: c_int = 0x1;
pub const ERR_FILE_OPEN: c_uint = 0x2;
pub const ERR_FILE_CREATE: c_uint = 0x3;

// BPX errors
pub const ERR_CORE_CHKSUM: c_uint = 0x5;
pub const ERR_CORE_IO: c_uint = 0x6;
pub const ERR_CORE_BAD_VERSION: c_uint = 0x7;
pub const ERR_CORE_BAD_SIGNATURE: c_uint = 0x8;
pub const ERR_CORE_CAPACITY: c_uint = 0xC;

// Inflate errors
pub const ERR_INFLATE_MEMORY: c_uint = 0xD;
pub const ERR_INFLATE_UNSUPPORTED: c_uint = 0xE;
pub const ERR_INFLATE_DATA: c_uint = 0xF;
pub const ERR_INFLATE_UNKNOWN: c_uint = 0x10;
pub const ERR_INFLATE_IO: c_uint = 0x11;

// Deflate errors
pub const ERR_DEFLATE_MEMORY: c_uint = 0x12;
pub const ERR_DEFLATE_UNSUPPORTED: c_uint = 0x13;
pub const ERR_DEFLATE_DATA: c_uint = 0x14;
pub const ERR_DEFLATE_UNKNOWN: c_uint = 0x15;
pub const ERR_DEFLATE_IO: c_uint = 0x16;

// Open errors
pub const ERR_OPEN_SECTION_IN_USE: c_uint = 0x17;
pub const ERR_OPEN_SECTION_NOT_LOADED: c_uint = 0x18;

// BPXSD errors
pub const ERR_SD_IO: c_uint = 0x19;
pub const ERR_SD_TRUNCATION: c_uint = 0x1A;
pub const ERR_SD_BAD_TYPE_CODE: c_uint = 0x1B;
pub const ERR_SD_UTF8: c_uint = 0x1C;
pub const ERR_SD_CAPACITY_EXCEEDED: c_uint = 0x1D;
pub const ERR_SD_NOT_AN_OBJECT: c_uint = 0x1E;

pub trait CErrCode
{
    fn cerr_code(&self) -> u32;
}

impl CErrCode for bpx::core::error::OpenError {
    fn cerr_code(&self) -> u32 {
        match self {
            OpenError::SectionInUse => ERR_OPEN_SECTION_IN_USE,
            OpenError::SectionNotLoaded => ERR_OPEN_SECTION_NOT_LOADED
        }
    }
}

impl CErrCode for bpx::core::error::InflateError {
    fn cerr_code(&self) -> u32 {
        match self {
            InflateError::Memory => ERR_INFLATE_MEMORY,
            InflateError::Unsupported(_) => ERR_INFLATE_UNSUPPORTED,
            InflateError::Data => ERR_INFLATE_DATA,
            InflateError::Unknown => ERR_INFLATE_UNKNOWN,
            InflateError::Io(_) => ERR_INFLATE_IO
        }
    }
}

impl CErrCode for bpx::core::error::DeflateError {
    fn cerr_code(&self) -> u32 {
        match self {
            DeflateError::Memory => ERR_DEFLATE_MEMORY,
            DeflateError::Unsupported(_) => ERR_DEFLATE_UNSUPPORTED,
            DeflateError::Data => ERR_DEFLATE_DATA,
            DeflateError::Unknown => ERR_DEFLATE_UNKNOWN,
            DeflateError::Io(_) => ERR_DEFLATE_IO
        }
    }
}

impl CErrCode for bpx::core::error::Error
{
    fn cerr_code(&self) -> u32
    {
        match self {
            Error::Checksum {..} => ERR_CORE_CHKSUM,
            Error::Io(_) => ERR_CORE_IO,
            Error::BadVersion(_) => ERR_CORE_BAD_VERSION,
            Error::BadSignature(_) => ERR_CORE_BAD_SIGNATURE,
            Error::Inflate(e) => e.cerr_code(),
            Error::Capacity(_) => ERR_CORE_CAPACITY,
            Error::Deflate(e) => e.cerr_code(),
            Error::Open(e) => e.cerr_code()
        }
    }
}

impl CErrCode for bpx::sd::error::Error {
    fn cerr_code(&self) -> u32 {
        match self {
            bpx::sd::error::Error::Io(_) => ERR_SD_IO,
            bpx::sd::error::Error::Truncation(_) => ERR_SD_TRUNCATION,
            bpx::sd::error::Error::BadTypeCode(_) => ERR_SD_BAD_TYPE_CODE,
            bpx::sd::error::Error::Utf8 => ERR_SD_UTF8,
            bpx::sd::error::Error::CapacityExceeded(_) => ERR_SD_CAPACITY_EXCEEDED,
            bpx::sd::error::Error::NotAnObject => ERR_SD_NOT_AN_OBJECT
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
