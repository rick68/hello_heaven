// SPDX-License-Identifier: GPL-2.0

#![allow(missing_docs)]

use kernel::{Module, ThisModule, error::Result, macros::module, pr_info};

#[no_mangle]
pub extern "C" fn hello(name: *const kernel::ffi::c_char) {
    pr_info!("Hello, {}!", unsafe {
        kernel::str::CStr::from_char_ptr(name)
    });
}

struct RustBaseUtils;

impl Module for RustBaseUtils {
    fn init(_module: &'static ThisModule) -> Result<Self> {
        () = hello(c"Heaven".as_ptr() as *const kernel::ffi::c_char);
        Ok(RustBaseUtils)
    }
}

module! {
    type: RustBaseUtils,
    name: "rust_base_utils",
    authors: ["Rick Yang"],
    description: "Rust base utilities sample",
    license: "GPL",
}
