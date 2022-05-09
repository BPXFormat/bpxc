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

use std::collections::HashMap;
use std::mem::MaybeUninit;
use std::os::raw::c_char;
use crate::sd::value::Value;
use crate::export;

pub struct ObjectWrapper(HashMap<u64, Value>);

impl ObjectWrapper {
    pub fn new() -> ObjectWrapper {
        ObjectWrapper(HashMap::new())
    }

    pub fn into_raw(self) -> *mut ObjectWrapper {
        let host = Box::new(self);
        Box::into_raw(host)
    }

    pub unsafe fn deallocate(ptr: *mut ObjectWrapper) {
        let host = Box::from_raw(ptr);
        drop(host);
    }

    pub unsafe fn insert_or_replace(&mut self, hash: u64, value: Value) {
        if let Some(mut old) = self.0.insert(hash, value) {
            old.free();
        }
    }

    pub fn wrap(object: bpx::sd::Object) -> Self {
        //TODO: optimize once into_inner is implemented in bpx::sd::Object.
        let mut map = HashMap::with_capacity(object.len());
        for (k, v) in &object {
            map.insert(k.into_inner(), Value::wrap(v.clone()));
        }
        Self(map)
    }
}

#[repr(C)]
pub struct ObjectEntry {
    hash: u64,
    value: Value
}

export!
{
    fn bpx_sd_object_get(object: *const ObjectWrapper, key: *const c_char) -> Value
    {
        let len = libc::strlen(key);
        let bytes = std::slice::from_raw_parts(std::mem::transmute(key), len);
        let key = std::str::from_utf8_unchecked(bytes);
        (*object).0.get(&bpx::utils::hash(key)).cloned().unwrap_or(Value::null())
    }

    fn bpx_sd_object_rawget(object: *const ObjectWrapper, hash: u64) -> Value
    {
        (*object).0.get(&hash).cloned().unwrap_or(Value::null())
    }

    fn bpx_sd_object_set(object: *mut ObjectWrapper, key: *const c_char, value: *mut Value)
    {
        let len = libc::strlen(key);
        let bytes = std::slice::from_raw_parts(std::mem::transmute(key), len);
        let key = std::str::from_utf8_unchecked(bytes);
        (*object).insert_or_replace(bpx::utils::hash(key), *value);
        (*value).reset();
    }

    fn bpx_sd_object_rawset(object: *mut ObjectWrapper, hash: u64, value: *mut Value)
    {
        (*object).insert_or_replace(hash, *value);
        (*value).reset();
    }

    fn bpx_sd_object_len(object: *const ObjectWrapper) -> usize
    {
        (*object).0.len()
    }

    fn bpx_sd_object_list(object: *const ObjectWrapper, out: *mut ObjectEntry)
    {
        let slice: &mut [MaybeUninit<ObjectEntry>] = std::slice::from_raw_parts_mut(out as _, (*object).0.len());
        for (i, (k, v)) in (*object).0.iter().enumerate() {
            slice[i].write(ObjectEntry {
                hash: *k,
                value: *v
            });
        }
    }
}
