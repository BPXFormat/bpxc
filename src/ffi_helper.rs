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

#[repr(transparent)]
pub struct OutCell<T>(*mut T);

impl<T> OutCell<T> {
    pub unsafe fn set(&self, value: T) {
        if self.0 == std::ptr::null_mut() {
            //Avoid crashing due to dereference of nullptr.
            return;
        }
        *self.0 = value;
    }
}

#[repr(transparent)]
pub struct Object<T>(*const T);

impl<T> Object<T> {
    pub fn new(value: T) -> Object<T> {
        let host = Box::new(value);
        Object(Box::into_raw(host))
    }
}

macro_rules! callback {
    (($($name: ident: $t: ty),*) $(-> $t1: ty)?) => {
        unsafe extern "C" fn ($($name: $t),*) $(-> $t1)?
    };
}

pub(crate) use callback;

macro_rules! export {
    (
        $(
            fn $name: ident ($($pname: ident: $ptype: ty),*) $(-> $ret: ty)? $body: block
        )*
    ) => {
        $(
            #[no_mangle]
            pub unsafe extern "C" fn $name ($($pname: $ptype),*) $(-> $ret)? $body
        )*
    };
}

pub(crate) use export;

macro_rules! export_object {
    (
        $obj: ty {
            $(
                fn $name: ident ($self: ident $(, $($pname: ident: $ptype: ty),*)?) $(-> $ret: ty)? $body: block
            )*
            $(
                mut fn $mut_name: ident ($mut_self: ident $(, $($mut_pname: ident: $mut_ptype: ty),*)?) $(-> $mut_ret: ty)? $mut_body: block
            )*
            $(close $closer_name: ident ($closer_self: ident) $closer_body: block)?
        }
    ) => {
        export! {
            $(
                fn $mut_name($mut_self: *mut $obj $(, $($mut_pname: $mut_ptype),*)?) $(-> $mut_ret)? {
                    let $mut_self = &mut *$mut_self;
                    $mut_body
                }
            )*
            $(
                fn $name($self: *const $obj $(, $($pname: $ptype),*)?) $(-> $ret)? {
                    let $self = &*$self;
                    $body
                }
            )*
            $(fn $closer_name(ptr: *mut *mut $obj) {
                let $closer_self = Box::from_raw(*ptr);
                $closer_body
                drop($closer_self); // deallocate the heap wrapper
                std::ptr::write(ptr, std::ptr::null_mut()); // reset user pointer to NULL
            })?
        }
    };
}

pub(crate) use export_object;
