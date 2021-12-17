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

use std::os::raw::{c_char, c_uint};
use std::fs::File;
use std::ffi::CStr;
use bpx::core::builder::{Checksum, CompressionMethod, MainHeaderBuilder, SectionHeaderBuilder};
use crate::Container;
use crate::error_codes::unwrap_or_err;
use crate::error_codes::{CErrCode, ERR_FILE_CREATE, ERR_FILE_OPEN, ERR_NONE};
use crate::header::MainHeader;
use crate::path_utils::cstr_to_path;

#[repr(C)]
pub struct ContainerOptions
{
    pub ty: u8,
    pub version: u32,
    pub type_ext: [u8; 16]
}

pub const COMPRESSION_ZLIB: u8 = 0x1;
pub const COMPRESSION_XZ: u8 = 0x2;
pub const CHECKSUM_WEAK: u8 = 0x4;
pub const CHECKSUM_CRC32: u8 = 0x8;
pub const COMPRESSION_THRESHOLD: u8 = 0x10;

#[repr(C)]
pub struct SectionOptions
{
    pub size: u32,
    pub ty: u8,
    pub flags: u8,
    pub threshold: u32
}

#[no_mangle]
pub unsafe fn bpx_container_open(file: *const c_char, out: *mut *const Container) -> c_uint
{
    let path = unwrap_or_err!(cstr_to_path(CStr::from_ptr(file)));
    let f = unwrap_or_err!(File::open(path).map_err(|_| ERR_FILE_OPEN));
    let container = unwrap_or_err!(bpx::core::Container::open(f).map_err(|e| e.cerr_code()));
    let host = Box::new(container);
    *out = Box::into_raw(host);
    ERR_NONE
}

#[no_mangle]
pub unsafe fn bpx_container_get_main_header(container: *const Container, main_header: *mut MainHeader)
{
    let container = &*container;
    let main_header = &mut *main_header;
    main_header.section_num = container.get_main_header().section_num;
    main_header.version = container.get_main_header().version;
    main_header.ty = container.get_main_header().btype;
    main_header.chksum = container.get_main_header().chksum;
    main_header.signature = container.get_main_header().signature;
    main_header.type_ext = container.get_main_header().type_ext;
    main_header.file_size = container.get_main_header().file_size;
}

#[no_mangle]
pub unsafe fn bpx_container_list_sections(container: *const Container, out: *mut bpx::Handle, size: usize)
{
    let container = &*container;
    container.iter().map(|v| v.handle()).take(size).enumerate().for_each(|(i, v)| {
        std::ptr::write(out.add(i * std::mem::size_of::<bpx::Handle>()), v);
    });
}

#[no_mangle]
pub unsafe fn bpx_container_find_section_by_type(container: *const Container, ty: u8, handle: *mut bpx::Handle) -> bool
{
    let container = &*container;
    if let Some(v) = container.find_section_by_type(ty) {
        *handle = v;
        true
    } else {
        false
    }
}

#[no_mangle]
pub unsafe fn bpx_container_find_section_by_index(container: *const Container, idx: u32, handle: *mut bpx::Handle) -> bool
{
    let container = &*container;
    if let Some(v) = container.find_section_by_index(idx) {
        *handle = v;
        true
    } else {
        false
    }
}

#[no_mangle]
pub unsafe fn bpx_container_create_section(container: *mut Container, options: *const SectionOptions) -> bpx::Handle
{
    let container = &mut *container;
    let options = &*options;
    let mut builder = SectionHeaderBuilder::new();
    builder.with_type(options.ty).with_size(options.size);
    if options.flags & CHECKSUM_CRC32 != 0 {
        builder.with_checksum(Checksum::Crc32);
    }
    if options.flags & CHECKSUM_WEAK != 0 {
        builder.with_checksum(Checksum::Weak);
    }
    if options.flags & COMPRESSION_ZLIB != 0 {
        builder.with_compression(CompressionMethod::Zlib);
    }
    if options.flags & COMPRESSION_XZ != 0 {
        builder.with_compression(CompressionMethod::Xz);
    }
    if options.flags & COMPRESSION_THRESHOLD != 0 {
        builder.with_threshold(options.threshold);
    }
    container.create_section(builder)
}

#[no_mangle]
pub unsafe fn bpx_container_create(file: *const c_char, header: *const ContainerOptions, out: *mut *const Container) -> c_uint
{
    let path = unwrap_or_err!(cstr_to_path(CStr::from_ptr(file)));
    let f = unwrap_or_err!(File::create(path).map_err(|_| ERR_FILE_CREATE));
    let h = &*header;
    let container = bpx::core::Container::create(f, MainHeaderBuilder::new()
        .with_type(h.ty)
        .with_type_ext(h.type_ext)
        .with_version(h.version));
    let host = Box::new(container);
    *out = Box::into_raw(host);
    ERR_NONE
}

#[no_mangle]
pub unsafe fn bpx_container_save(container: *mut Container) -> c_uint
{
    let container = &mut *container;
    unwrap_or_err!(container.save().map_err(|e| e.cerr_code()));
    ERR_NONE
}

#[no_mangle]
pub unsafe fn bpx_container_close(container: *mut *mut Container)
{
    let b = Box::from_raw(*container);
    std::mem::drop(b); // deallocate the bpx container
    std::ptr::write(container, std::ptr::null_mut()); // reset user pointer to NULL
}
