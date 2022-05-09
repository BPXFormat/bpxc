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

use std::ffi::{CStr, CString};
use std::mem::MaybeUninit;
use std::os::raw::c_char;
use crate::export;
use crate::sd::array::ArrayWrapper;
use crate::sd::object::ObjectWrapper;

#[repr(C)]
#[derive(Copy, Clone)]
pub enum ValueType {
    Null,
    Bool,
    Uint8,
    Uint16,
    Uint32,
    Uint64,
    Int8,
    Int16,
    Int32,
    Int64,
    Float,
    Double,
    String,
    Array,
    Object
}

#[repr(C)]
#[derive(Copy, Clone)]
pub union ValueData {
    as_bool: bool,
    as_u8: u8,
    as_u16: u16,
    as_u32: u32,
    as_u64: u64,
    as_i8: i8,
    as_i16: i16,
    as_i32: i32,
    as_i64: i64,
    as_float: f32,
    as_double: f64,
    as_string: *const c_char,
    as_array: *mut ArrayWrapper,
    as_object: *mut ObjectWrapper
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Value {
    pub ty: ValueType,
    pub data: MaybeUninit<ValueData>
}

impl Value {
    pub unsafe fn reset(&mut self) {
        match self.ty {
            ValueType::Object => self.data.assume_init_mut().as_object = std::ptr::null_mut(),
            ValueType::Array => self.data.assume_init_mut().as_array = std::ptr::null_mut(),
            ValueType::String => self.data.assume_init_mut().as_string = std::ptr::null_mut(),
            _ => ()
        };
        self.ty = ValueType::Null;
    }

    pub fn new(ty: ValueType, data: ValueData) -> Value {
        Self {
            ty,
            data: MaybeUninit::new(data)
        }
    }

    pub fn wrap(value: bpx::sd::Value) -> Self {
        match value {
            bpx::sd::Value::Null => Self::null(),
            bpx::sd::Value::Bool(v) => Self::new(ValueType::Bool, ValueData { as_bool: v }),
            bpx::sd::Value::Uint8(v) => Self::new(ValueType::Bool, ValueData { as_u8: v }),
            bpx::sd::Value::Uint16(v) => Self::new(ValueType::Bool, ValueData { as_u16: v }),
            bpx::sd::Value::Uint32(v) => Self::new(ValueType::Bool, ValueData { as_u32: v }),
            bpx::sd::Value::Uint64(v) => Self::new(ValueType::Bool, ValueData { as_u64: v }),
            bpx::sd::Value::Int8(v) => Self::new(ValueType::Bool, ValueData { as_i8: v }),
            bpx::sd::Value::Int16(v) => Self::new(ValueType::Bool, ValueData { as_i16: v }),
            bpx::sd::Value::Int32(v) => Self::new(ValueType::Bool, ValueData { as_i32: v }),
            bpx::sd::Value::Int64(v) => Self::new(ValueType::Bool, ValueData { as_i64: v }),
            bpx::sd::Value::Float(v) => Self::new(ValueType::Bool, ValueData { as_float: v }),
            bpx::sd::Value::Double(v) => Self::new(ValueType::Bool, ValueData { as_double: v }),
            bpx::sd::Value::String(v) => Self::new(ValueType::Bool, ValueData { as_string: CString::new(v).unwrap().into_raw() }),
            bpx::sd::Value::Array(v) => Self::new(ValueType::Bool, ValueData { as_array: ArrayWrapper::wrap(v).into_raw() }),
            bpx::sd::Value::Object(v) => Self::new(ValueType::Bool, ValueData { as_object: ObjectWrapper::wrap(v).into_raw() })
        }
    }

    pub unsafe fn into_value(self) -> bpx::sd::Value {
        match self.ty {
            ValueType::Null => bpx::sd::Value::Null,
            ValueType::Bool => self.data.assume_init().as_bool.into(),
            ValueType::Uint8 => self.data.assume_init().as_u8.into(),
            ValueType::Uint16 => self.data.assume_init().as_u16.into(),
            ValueType::Uint32 => self.data.assume_init().as_u32.into(),
            ValueType::Uint64 => self.data.assume_init().as_u64.into(),
            ValueType::Int8 => self.data.assume_init().as_i8.into(),
            ValueType::Int16 => self.data.assume_init().as_i16.into(),
            ValueType::Int32 => self.data.assume_init().as_i32.into(),
            ValueType::Int64 => self.data.assume_init().as_i64.into(),
            ValueType::Float => self.data.assume_init().as_float.into(),
            ValueType::Double => self.data.assume_init().as_double.into(),
            ValueType::String => {
                let len = libc::strlen(self.data.assume_init().as_string);
                let slice = std::slice::from_raw_parts(self.data.assume_init().as_string as _, len);
                let s = String::from(std::str::from_utf8_unchecked(slice));
                s.into()
            }
            ValueType::Array => (*self.data.assume_init().as_array).to_array().into(),
            ValueType::Object => (*self.data.assume_init().as_object).to_object().into()
        }
    }

    pub fn null() -> Self {
        Value {
            ty: ValueType::Null,
            data: MaybeUninit::uninit()
        }
    }

    pub unsafe fn free(&mut self) {
        match self.ty {
            ValueType::Array => {
                ArrayWrapper::deallocate(self.data.assume_init_mut().as_array);
                self.data.assume_init_mut().as_array = std::ptr::null_mut(); //Reset user pointer
            },
            ValueType::Object => {
                ObjectWrapper::deallocate(self.data.assume_init_mut().as_object);
                self.data.assume_init_mut().as_object = std::ptr::null_mut(); //Reset user pointer
            },
            ValueType::String => {
                let host = CString::from_raw(self.data.assume_init_mut().as_string as _);
                drop(host); //Force deallocate string
                self.data.assume_init_mut().as_string = std::ptr::null_mut(); //Reset user pointer
            },
            _ => ()
        };
        self.ty = ValueType::Null
    }
}

export!
{
    fn bpx_sd_value_new() -> Value
    {
        Value::null()
    }

    fn bpx_sd_value_new_bool(value: bool) -> Value
    {
        Value::new(ValueType::Bool, ValueData { as_bool: value })
    }

    fn bpx_sd_value_new_u8(value: u8) -> Value
    {
        Value::new(ValueType::Uint8, ValueData { as_u8: value })
    }

    fn bpx_sd_value_new_u16(value: u16) -> Value
    {
        Value::new(ValueType::Uint16, ValueData { as_u16: value })
    }

    fn bpx_sd_value_new_u32(value: u32) -> Value
    {
        Value::new(ValueType::Uint32, ValueData { as_u32: value })
    }

    fn bpx_sd_value_new_u64(value: u64) -> Value
    {
        Value::new(ValueType::Uint64, ValueData { as_u64: value })
    }

    fn bpx_sd_value_new_i8(value: i8) -> Value
    {
        Value::new(ValueType::Int8, ValueData { as_i8: value })
    }

    fn bpx_sd_value_new_i16(value: i16) -> Value
    {
        Value::new(ValueType::Int16, ValueData { as_i16: value })
    }

    fn bpx_sd_value_new_i32(value: i32) -> Value
    {
        Value::new(ValueType::Int32, ValueData { as_i32: value })
    }

    fn bpx_sd_value_new_i64(value: i64) -> Value
    {
        Value::new(ValueType::Int64, ValueData { as_i64: value })
    }

    fn bpx_sd_value_new_float(value: f32) -> Value
    {
        Value::new(ValueType::Float, ValueData { as_float: value })
    }

    fn bpx_sd_value_new_double(value: f64) -> Value
    {
        Value::new(ValueType::Double, ValueData { as_double: value })
    }

    fn bpx_sd_value_new_string(value: *const c_char) -> Value
    {
        Value::new(ValueType::String, ValueData { as_string: CString::from(CStr::from_ptr(value)).into_raw() })
    }

    fn bpx_sd_value_new_array() -> Value
    {
        Value::new(ValueType::Array, ValueData { as_array: ArrayWrapper::new().into_raw() })
    }

    fn bpx_sd_value_new_object() -> Value
    {
        Value::new(ValueType::Object, ValueData { as_object: ObjectWrapper::new().into_raw() })
    }

    fn bpx_sd_value_free(value: *mut Value)
    {
        (*value).free();
    }
}
