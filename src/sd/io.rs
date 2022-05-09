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

use crate::error_codes::{CErrCode, ERR_NONE};
use crate::error_codes::unwrap_or_err;
use std::os::raw::c_uint;
use crate::export;
use crate::types::Section;
use super::value::Value;

export!
{
    fn bpx_sd_value_decode_section(section: *mut Section, out: *mut Value) -> c_uint
    {
        let value = unwrap_or_err!(bpx::sd::Value::read(&mut **section).map_err(|v| v.cerr_code()));
        out.write(Value::wrap(value));
        ERR_NONE
    }

    fn bpx_sd_value_decode_memory(buffer: *const u8, size: usize, out: *mut Value) -> c_uint
    {
        let slice = std::slice::from_raw_parts(buffer, size);
        let value = unwrap_or_err!(bpx::sd::Value::read(slice).map_err(|v| v.cerr_code()));
        out.write(Value::wrap(value));
        ERR_NONE
    }

    fn bpx_sd_value_encode(section: *mut Section, value: *const Value) -> c_uint
    {
        let value = (*value).into_value();
        value.write(&mut **section).map(|_| ERR_NONE).unwrap_or_else(|v| v.cerr_code())
    }
}
