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

use std::io::{Read, SeekFrom, Seek, Write};
use std::os::raw::c_uint;
use crate::{Container, Handle};
use crate::error_codes::unwrap_or_err;
use crate::error_codes::{CErrCode, ERR_NONE, ERR_SECTION_IO};
use crate::header::SectionHeader;
use crate::export;

export!
{
    fn bpx_section_get_header(container: *const Container, handle: Handle, section_header: *mut SectionHeader)
    {
        let container = &*container;
        let section_header = &mut *section_header;
        let section = container.get(bpx::Handle::from_raw(handle));
        section_header.size = section.size;
        section_header.chksum = section.chksum;
        section_header.csize = section.csize;
        section_header.flags = section.flags;
        section_header.ty = section.btype;
        section_header.pointer = section.pointer;
    }

    fn bpx_section_read(container: *mut Container, handle: Handle, buffer: *mut u8, size: usize) -> c_uint
    {
        let container = &mut *container;
        let mut section = container.get_mut(bpx::Handle::from_raw(handle));
        let data = unwrap_or_err!(section.load().map_err(|v| v.cerr_code()));
        std::ptr::write_bytes(buffer, 0, size); //This allows us to initialize the buffer in preparation of std::io::Read call
        let slice = std::slice::from_raw_parts_mut(buffer, size);
        unwrap_or_err!(data.read(slice).map_err(|_| ERR_SECTION_IO));
        ERR_NONE
    }

    fn bpx_section_seek(container: *mut Container, handle: Handle, pos: u64) -> c_uint
    {
        let container = &mut *container;
        let mut section = container.get_mut(bpx::Handle::from_raw(handle));
        let data = unwrap_or_err!(section.load().map_err(|v| v.cerr_code()));
        unwrap_or_err!(data.seek(SeekFrom::Start(pos)).map_err(|_| ERR_SECTION_IO));
        ERR_NONE
    }

    //SAFETY: make sure buffer is initialized otherwise UB!
    fn bpx_section_write(container: *mut Container, handle: Handle, buffer: *const u8, size: usize) -> c_uint
    {
        let container = &mut *container;
        let mut section = container.get_mut(bpx::Handle::from_raw(handle));
        let data = unwrap_or_err!(section.load().map_err(|v| v.cerr_code()));
        let slice = std::slice::from_raw_parts(buffer, size);
        unwrap_or_err!(data.write(slice).map_err(|_| ERR_SECTION_IO));
        ERR_NONE
    }

    fn bpx_section_flush(container: *mut Container, handle: Handle) -> c_uint
    {
        let container = &mut *container;
        let mut section = container.get_mut(bpx::Handle::from_raw(handle));
        let data = unwrap_or_err!(section.load().map_err(|v| v.cerr_code()));
        unwrap_or_err!(data.flush().map_err(|_| ERR_SECTION_IO));
        ERR_NONE
    }
}
