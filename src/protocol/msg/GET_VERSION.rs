// Copyright (c) 2024 lxyboy <lxyboy@hotmail.com>
//
// SPDX-License-Identifier: Apache-2.0 OR MIT

pub type Result<T = ()> = core::result::Result<T, ()>;

pub struct Buffer<T: AsRef<[u8]>> {
    buffer: T,
}

impl<T: AsRef<[u8]>> Buffer<T> {
    pub const fn new_unchecked(buffer: T) -> Buffer<T> {
        Buffer { buffer }
    }

    pub fn new_checked(buffer: T) -> Result<Buffer<T>> {
        let message = Self::new_unchecked(buffer);
        message.check_len()?;
        Ok(message)
    }

    pub fn check_len(&self) -> Result {
        Ok(())
    }
}

#[test]
fn test_message() {
    let buffer = [0, 1, 2, 3];
}
