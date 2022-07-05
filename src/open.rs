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

use std::os::raw::{c_char, c_uint};
use bpx::core::builder::MainHeaderBuilder;
use std::fs::File;
use std::ffi::CStr;
use crate::error_codes::CErrCode;
use crate::error_codes::ERR_FILE_CREATE;
use crate::error_codes::ERR_FILE_OPEN;
use crate::path_utils::cstr_to_path;
use crate::types::Container;
use crate::error_codes::ERR_NONE;
use crate::error_codes::unwrap_or_err;
use crate::ffi_helper::export;
use crate::container_wrapper::ContainerWrapper;
use crate::io_wrapper::ContainerIo;
use crate::io_wrapper::IoWrapper;

#[repr(C)]
pub struct ContainerOptions
{
    pub ty: u8,
    pub version: u32,
    pub type_ext: [u8; 16]
}

export!
{
    fn bpx_container_open(file: *const c_char, out: *mut *const Container) -> c_uint
    {
        let path = unwrap_or_err!(cstr_to_path(CStr::from_ptr(file)));
        let f = unwrap_or_err!(File::open(path).map_err(|_| ERR_FILE_OPEN));
        let container = unwrap_or_err!(bpx::core::Container::open(ContainerWrapper::from(f)).map_err(|e| e.cerr_code()));
        let host = Box::new(container);
        *out = Box::into_raw(host);
        ERR_NONE
    }

    fn bpx_container_create(file: *const c_char, header: *const ContainerOptions, out: *mut *const Container) -> c_uint
    {
        let path = unwrap_or_err!(cstr_to_path(CStr::from_ptr(file)));
        let f = unwrap_or_err!(File::create(path).map_err(|_| ERR_FILE_CREATE));
        let h = &*header;
        let container = bpx::core::Container::create(ContainerWrapper::from(f), MainHeaderBuilder::new()
            .ty(h.ty)
            .type_ext(h.type_ext)
            .version(h.version));
        let host = Box::new(container);
        *out = Box::into_raw(host);
        ERR_NONE
    }

    fn bpx_container_open2(io: ContainerIo, out: *mut *const Container) -> c_uint
    {
        let wrapper = ContainerWrapper::from(IoWrapper::new(io));
        let container = unwrap_or_err!(bpx::core::Container::open(wrapper).map_err(|e| e.cerr_code()));
        let host = Box::new(container);
        *out = Box::into_raw(host);
        ERR_NONE
    }

    fn bpx_container_create2(io: ContainerIo, header: *const ContainerOptions, out: *mut *const Container) -> c_uint
    {
        let h = &*header;
        let wrapper = ContainerWrapper::from(IoWrapper::new(io));
        let container = bpx::core::Container::create(wrapper, MainHeaderBuilder::new()
            .ty(h.ty)
            .type_ext(h.type_ext)
            .version(h.version));
        let host = Box::new(container);
        *out = Box::into_raw(host);
        ERR_NONE
    }
}
