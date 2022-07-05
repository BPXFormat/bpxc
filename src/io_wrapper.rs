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

use std::ffi::c_void;
use std::io::{Error, ErrorKind, Read, Seek, Write};
use std::os::raw::c_uint;
use crate::error_codes::{ERR_CORE_IO, ERR_NONE};
use crate::ffi_helper::callback;

#[repr(C)]
pub enum SeekFrom
{
    Start,
    End,
    Current
}

#[repr(C)]
pub struct ContainerIo
{
    pub userdata: *const c_void,
    //Return true for success, false otherwise
    pub seek: callback!((userdata: *const c_void, from: SeekFrom, pos: u64, new_pos: *mut u64) -> c_uint),
    pub read: callback!((userdata: *const c_void, buffer: *mut u8, size: usize, bytes_read: *mut usize) -> c_uint),
    pub write: Option<callback!((userdata: *const c_void, buffer: *const u8, size: usize, bytes_written: *mut usize) -> c_uint)>,
    pub flush: Option<callback!((userdata: *const c_void) -> c_uint)>
}

pub struct IoWrapper
{
    raw: ContainerIo,
    last_error: c_uint
}

impl IoWrapper
{
    pub fn new(raw: ContainerIo) -> IoWrapper
    {
        IoWrapper {
            raw,
            last_error: 0
        }
    }

    fn handle_low_level_err(&mut self, res: c_uint, count: usize) -> std::io::Result<usize>
    {
        if res == ERR_NONE {
            Ok(count)
        } else if res == ERR_CORE_IO {
            Err(Error::last_os_error())
        } else {
            self.last_error = res;
            Err(Error::new(ErrorKind::Other, "Low level C user defined custom error"))
        }
    }
}

impl Read for IoWrapper
{
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize>
    {
        let mut bytes_read = 0;
        let res = unsafe {
            (self.raw.read)(self.raw.userdata, buf.as_mut_ptr(), buf.len(), &mut bytes_read as _)
        };
        self.handle_low_level_err(res, bytes_read)
    }
}

impl Write for IoWrapper
{
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize>
    {
        let mut bytes_written = 0;
        let func = self.raw.write.ok_or_else(|| Error::new(ErrorKind::Unsupported, "Write operation is unsupported"))?;
        let res = unsafe {
            (func)(self.raw.userdata, buf.as_ptr(), buf.len(), &mut bytes_written as _)
        };
        self.handle_low_level_err(res, bytes_written)
    }

    fn flush(&mut self) -> std::io::Result<()>
    {
        let func = self.raw.flush.ok_or_else(|| Error::new(ErrorKind::Unsupported, "Flush operation is unsupported"))?;
        let res = unsafe {
            (func)(self.raw.userdata)
        };
        self.handle_low_level_err(res, 0).map(|_| ())
    }
}

impl Seek for IoWrapper
{
    fn seek(&mut self, pos: std::io::SeekFrom) -> std::io::Result<u64>
    {
        unsafe {
            let mut new_pos: u64 = 0; //Rust is buggy and can't figure out that the function only accepts u64 whereas it can with usize!!
            let res = match pos {
                std::io::SeekFrom::Start(offset) => (self.raw.seek)(self.raw.userdata, SeekFrom::Start, offset, &mut new_pos as _),
                std::io::SeekFrom::End(offset) => (self.raw.seek)(self.raw.userdata, SeekFrom::End, offset as u64, &mut new_pos as _),
                std::io::SeekFrom::Current(offset) => (self.raw.seek)(self.raw.userdata, SeekFrom::Current, offset as u64, &mut new_pos as _)
            };
            self.handle_low_level_err(res, 0).map(|_| new_pos)
        }
    }
}
