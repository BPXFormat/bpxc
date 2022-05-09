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

use std::io::{Read, SeekFrom, Seek, Write};
use std::os::raw::{c_int, c_uint};
use crate::types::{Container, Handle, Section};
use crate::error_codes::unwrap_or_err;
use crate::error_codes::{CErrCode, ERR_NONE};
use crate::types::SectionHeader;
use crate::export;

export! {
    fn bpx_section_get_header(container: *const Container, handle: Handle, section_header: *mut SectionHeader)
    {
        let container = &*container;
        let section_header = &mut *section_header;
        let section = container.sections().header(bpx::core::Handle::from_raw(handle));
        section_header.size = section.size;
        section_header.chksum = section.chksum;
        section_header.csize = section.csize;
        section_header.flags = section.flags;
        section_header.ty = section.ty;
        section_header.pointer = section.pointer;
    }

    fn bpx_section_load(container: *const Container, handle: Handle, out: *mut *const Section) -> c_uint
    {
        let container = &*container;
        let section = unwrap_or_err!(container.sections().load(bpx::core::Handle::from_raw(handle)).map_err(|e| e.cerr_code()));
        let host = Box::new(section);
        *out = Box::into_raw(host);
        ERR_NONE
    }

    fn bpx_section_open(container: *const Container, handle: Handle, out: *mut *const Section) -> c_uint
    {
        let container = &*container;
        let section = unwrap_or_err!(container.sections().open(bpx::core::Handle::from_raw(handle)).map_err(|e| e.cerr_code()));
        let host = Box::new(section);
        *out = Box::into_raw(host);
        ERR_NONE
    }

    fn bpx_section_read(section: *mut Section, buffer: *mut u8, size: usize) -> usize {
        let section = &mut *section;
        std::ptr::write_bytes(buffer, 0, size); //This allows us to initialize the buffer in preparation of std::io::Read call
        let slice = std::slice::from_raw_parts_mut(buffer, size);
        section.read(slice).unwrap_or(usize::MAX)
    }

    //SAFETY: make sure buffer is initialized otherwise UB!
    fn bpx_section_write(section: *mut Section, buffer: *const u8, size: usize) -> usize {
        let section = &mut *section;
        let slice = std::slice::from_raw_parts(buffer, size);
        section.write(slice).unwrap_or(usize::MAX)
    }

    fn bpx_section_seek(section: *mut Section, pos: u64) -> u64
    {
        let section = &mut *section;
        section.seek(SeekFrom::Start(pos)).unwrap_or(u64::MAX)
    }

    fn bpx_section_flush(section: *mut Section) -> c_int
    {
        let section = &mut *section;
        section.flush().map(|_| 0).unwrap_or(-1)
    }

    fn bpx_section_close(section: *mut *mut Section)
    {
        let b = Box::from_raw(*section);
        drop(b); // deallocate the heap wrapper and unlock the RefMut
        std::ptr::write(section, std::ptr::null_mut()); // reset user pointer to NULL
    }
}
