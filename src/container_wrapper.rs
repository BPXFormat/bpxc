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

use std::fs::File;
use std::io::{Read, Seek, SeekFrom, Write};
use crate::IoWrapper;

pub enum ContainerWrapper
{
    File(File),
    IoWrapper(IoWrapper)
}

impl From<File> for ContainerWrapper
{
    fn from(v: File) -> Self
    {
        Self::File(v)
    }
}

impl From<IoWrapper> for ContainerWrapper
{
    fn from(v: IoWrapper) -> Self
    {
        Self::IoWrapper(v)
    }
}

impl Read for ContainerWrapper
{
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize>
    {
        match self {
            ContainerWrapper::File(v) => v.read(buf),
            ContainerWrapper::IoWrapper(v) => v.read(buf)
        }
    }
}

impl Write for ContainerWrapper
{
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize>
    {
        match self {
            ContainerWrapper::File(v) => v.write(buf),
            ContainerWrapper::IoWrapper(v) => v.write(buf)
        }
    }

    fn flush(&mut self) -> std::io::Result<()>
    {
        match self {
            ContainerWrapper::File(v) => v.flush(),
            ContainerWrapper::IoWrapper(v) => v.flush()
        }
    }
}

impl Seek for ContainerWrapper
{
    fn seek(&mut self, pos: SeekFrom) -> std::io::Result<u64>
    {
        match self {
            ContainerWrapper::File(v) => v.seek(pos),
            ContainerWrapper::IoWrapper(v) => v.seek(pos)
        }
    }
}
