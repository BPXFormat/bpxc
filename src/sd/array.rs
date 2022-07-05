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

use std::mem::MaybeUninit;
use crate::sd::value::Value;
use crate::ffi_helper::export;

pub struct ArrayWrapper(Vec<Value>);

impl ArrayWrapper {
    pub fn new() -> ArrayWrapper {
        ArrayWrapper(Vec::new())
    }

    pub fn into_raw(self) -> *mut ArrayWrapper {
        let host = Box::new(self);
        Box::into_raw(host)
    }

    pub unsafe fn deallocate(ptr: *mut ArrayWrapper) {
        let host = Box::from_raw(ptr);
        drop(host);
    }

    pub fn wrap(array: bpx::sd::Array) -> Self {
        //TODO: optimize once into_inner is implemented in bpx::sd::Array.
        let mut lst = Vec::with_capacity(array.len());
        for v in &array {
            lst.push(Value::wrap(v.clone()));
        }
        Self(lst)
    }

    pub unsafe fn to_array(&self) -> bpx::sd::Array {
        let mut arr = bpx::sd::Array::with_capacity(self.0.len() as _);
        for v in &self.0 {
            arr.as_mut().push(v.into_value())
        }
        arr
    }
}

export!
{
    fn bpx_sd_array_push(array: *mut ArrayWrapper, value: *mut Value)
    {
        (*array).0.push(*value);
        (*value).reset();
    }

    fn bpx_sd_array_insert(array: *mut ArrayWrapper, value: *mut Value, index: usize)
    {
        (*array).0.insert(index, *value);
        (*value).reset();
    }

    fn bpx_sd_array_remove(array: *mut ArrayWrapper, index: usize)
    {
        (*array).0.get_mut(index).map(|v| v.free());
        (*array).0.remove(index);
    }

    fn bpx_sd_array_list(array: *const ArrayWrapper, out: *mut Value)
    {
        let slice: &mut [MaybeUninit<Value>] = std::slice::from_raw_parts_mut(out as _, (*array).0.len());
        for (i, v) in (*array).0.iter().enumerate() {
            slice[i].write(*v);
        }
    }

    fn bpx_sd_array_get(array: *const ArrayWrapper, index: usize) -> Value
    {
        (*array).0.get(index).cloned().unwrap_or(Value::null())
    }

    fn bpx_sd_array_len(array: *const ArrayWrapper) -> usize
    {
        (*array).0.len()
    }
}
