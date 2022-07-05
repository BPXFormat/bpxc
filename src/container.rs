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
use bpx::core::builder::{Checksum, CompressionMethod, SectionHeaderBuilder};
use crate::types::{Container, Handle};
use crate::error_codes::unwrap_or_err;
use crate::error_codes::{CErrCode, ERR_NONE};
use crate::types::MainHeader;
use crate::ffi_helper::export;
use crate::ffi_helper::export_object;
use crate::ffi_helper::OutCell;

#[repr(C)]
pub struct SectionOptions
{
    pub size: u32,
    pub ty: u8,
    pub flags: u8,
    pub threshold: u32
}

pub const COMPRESSION_ZLIB: u8 = 0x1;
pub const COMPRESSION_XZ: u8 = 0x2;
pub const CHECKSUM_WEAK: u8 = 0x4;
pub const CHECKSUM_CRC32: u8 = 0x8;
pub const COMPRESSION_THRESHOLD: u8 = 0x10;

export_object! {
    Container {
        fn bpx_container_get_main_header(this, main_header: OutCell<MainHeader>) {
            main_header.set(MainHeader {
                section_num: this.get_main_header().section_num,
                version: this.get_main_header().version,
                ty: this.get_main_header().ty,
                chksum: this.get_main_header().chksum,
                signature: this.get_main_header().signature,
                type_ext: this.get_main_header().type_ext,
                file_size: this.get_main_header().file_size
            });
        }

        fn bpx_container_list_sections(this, out: *mut Handle, size: usize) {
            this.sections().iter().map(|v| v.into_raw()).take(size).enumerate().for_each(|(i, v)| {
                std::ptr::write(out.add(i), v);
            });
        }

        fn bpx_container_find_section_by_type(this, ty: u8, handle: OutCell<Handle>) -> bool {
            if let Some(v) = this.sections().find_by_type(ty) {
                handle.set(v.into_raw());
                true
            } else {
                false
            }
        }

        fn bpx_container_find_section_by_index(this, idx: u32, handle: OutCell<Handle>) -> bool {
            if let Some(v) = this.sections().find_by_index(idx) {
                handle.set(v.into_raw());
                true
            } else {
                false
            }
        }

        mut fn bpx_container_create_section(this, options: *const SectionOptions) -> Handle {
            let options = &*options;
            let mut builder = SectionHeaderBuilder::new();
            builder.ty(options.ty).size(options.size);
            if options.flags & CHECKSUM_CRC32 != 0 {
                builder.checksum(Checksum::Crc32);
            }
            if options.flags & CHECKSUM_WEAK != 0 {
                builder.checksum(Checksum::Weak);
            }
            if options.flags & COMPRESSION_ZLIB != 0 {
                builder.compression(CompressionMethod::Zlib);
            }
            if options.flags & COMPRESSION_XZ != 0 {
                builder.compression(CompressionMethod::Xz);
            }
            if options.flags & COMPRESSION_THRESHOLD != 0 {
                builder.threshold(options.threshold);
            }
            this.sections_mut().create(builder).into_raw()
        }

        mut fn bpx_container_save(this) -> c_uint {
            unwrap_or_err!(this.save().map_err(|e| e.cerr_code()));
            ERR_NONE
        }

        close bpx_container_close(this) {}
    }
}
