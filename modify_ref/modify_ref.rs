// SPDX-License-Identifier: GPL-2.0

#![allow(missing_docs)]

use kernel::{Module, ThisModule, dbg, error::Result, macros::module, pr_info};

#[inline]
fn do_something_bad_1(x: &i32) {
    #[allow(invalid_reference_casting)]
    unsafe {
        *(x as *const i32 as *mut i32) += 1
    };
}

fn do_something_bad_2(x: &i32) {
    pr_info!("x = {x}\n");
    #[allow(invalid_reference_casting)]
    unsafe {
        *(x as *const i32 as *mut i32) += 1
    };
}

struct ModifyRef;

impl Module for ModifyRef {
    fn init(_module: &'static ThisModule) -> Result<Self> {
        let x: i32 = 42;

        dbg!(x);
        () = do_something_bad_1(&x);
        pr_info!("x = {x} (after do_something_bad_1)\n");
        () = do_something_bad_2(&x);
        pr_info!("x = {x} (after do_something_bad_2)\n");

        Ok(ModifyRef)
    }
}

module! {
    type: ModifyRef,
    name: "modify_ref",
    authors: ["Rick Yang"],
    description: "Modify reference",
    license: "GPL",
}
