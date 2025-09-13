// SPDX-License-Identifier: GPL-2.0

//! Rust kunit sample.

use kernel::{Module, ThisModule, error::Result, macros::module, pr_info};

mod common;

struct RustKunit;

impl Module for RustKunit {
    fn init(_module: &'static ThisModule) -> Result<Self> {
        pr_info!("Rust kunit sample (init)\n");

        let a: i32 = 960;
        let b: i32 = 110;
        let ret: i32 = common::bad_add(a, b)?;
        pr_info!("Do the bad addition: {a} + {b} = {ret}");

        Ok(RustKunit)
    }
}

impl Drop for RustKunit {
    fn drop(&mut self) {
        pr_info!("Rust kunit sample (exit)\n");
    }
}

module! {
    type: RustKunit,
    name: "rust_kunit",
    authors: ["Rick Yang"],
    description: "Rust kunit sample",
    license: "GPL",
}
