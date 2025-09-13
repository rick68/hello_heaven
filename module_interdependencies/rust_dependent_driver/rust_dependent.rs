// SPDX-License-Identifier: GPL-2.0

#![allow(missing_docs)]

use kernel::{Module, ThisModule, error::Result, macros::module};

extern "C" {
    fn hello(name: *const kernel::ffi::c_char);
}

struct RustDependentDriver;

impl Module for RustDependentDriver {
    fn init(_module: &'static ThisModule) -> Result<Self> {
        () = unsafe { hello(c"HEAVEN".as_ptr() as *const kernel::ffi::c_char) };
        Ok(RustDependentDriver)
    }
}

module! {
    type: RustDependentDriver,
    name: "rust_dependent_driver",
    authors: ["Rick Yang"],
    description: "Rust dependent driver sample",
    license: "GPL",
}
