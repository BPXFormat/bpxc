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
use crate::ffi_helper::export;
use crate::ffi_helper::export_object;
use crate::ffi_helper::OutCell;
use crate::ffi_helper::Object;
use bpx::core::SectionData;
use bpx::traits::Shift;
use bpx::traits::ShiftTo;

export_object! {
    Container {
        fn bpx_section_get_header(this, handle: Handle, section_header: OutCell<SectionHeader>) {
            let section = this.sections().header(bpx::core::Handle::from_raw(handle));
            section_header.set(SectionHeader {
                size: section.size,
                chksum: section.chksum,
                csize: section.csize,
                flags: section.flags,
                ty: section.ty,
                pointer: section.pointer
            });
        }

        fn bpx_section_load(this, handle: Handle, out: OutCell<Object<Section>>) -> c_uint {
            let section = unwrap_or_err!(this.sections()
                .load(bpx::core::Handle::from_raw(handle)).map_err(|e| e.cerr_code()));
            out.set(Object::new(section));
            ERR_NONE
        }

        fn bpx_section_open(this, handle: Handle, out: OutCell<Object<Section>>) -> c_uint
        {
            let section = unwrap_or_err!(this.sections()
                .open(bpx::core::Handle::from_raw(handle)).map_err(|e| e.cerr_code()));
            out.set(Object::new(section));
            ERR_NONE
        }
    }
}

export_object! {
    Section {
        fn bpx_section_size(this) -> usize { this.size() }

        mut fn bpx_section_read(this, buffer: *mut u8, size: usize) -> usize {
            std::ptr::write_bytes(buffer, 0, size); //This allows us to initialize the buffer in preparation of std::io::Read call
            let slice = std::slice::from_raw_parts_mut(buffer, size);
            this.read(slice).unwrap_or(usize::MAX)
        }

        //SAFETY: make sure buffer is initialized otherwise UB!
        mut fn bpx_section_write(this, buffer: *const u8, size: usize) -> usize {
            let slice = std::slice::from_raw_parts(buffer, size);
            this.write(slice).unwrap_or(usize::MAX)
        }

        //SAFETY: make sure buffer is initialized otherwise UB!
        mut fn bpx_section_write_append(this, buffer: *const u8, size: usize) -> usize {
            let slice = std::slice::from_raw_parts(buffer, size);
            this.write_append(slice).unwrap_or(usize::MAX)
        }

        mut fn bpx_section_seek(this, pos: u64) -> u64 {
            this.seek(SeekFrom::Start(pos)).unwrap_or(u64::MAX)
        }

        mut fn bpx_section_flush(this) -> c_int { this.flush().map(|_| 0).unwrap_or(-1) }

        mut fn bpx_section_truncate(this, size: usize, new_size: OutCell<usize>) -> c_int {
            let size = unwrap_or_err!(this.truncate(size).map_err(|_| -1));
            new_size.set(size);
            0
        }

        mut fn bpx_section_shift(this, amount: i64) -> c_int {
            let res = if amount < 0 {
                this.shift(ShiftTo::Left(-amount as u64))
            } else {
                this.shift(ShiftTo::Right(amount as u64))
            };
            res.map(|_| 0).unwrap_or(-1)
        }

        close bpx_section_close(this) {}
    }
}
