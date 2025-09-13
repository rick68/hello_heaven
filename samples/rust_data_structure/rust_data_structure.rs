// SPDX-License-Identifier: GPL-2.0

//! Rust data structure sample.

use kernel::{
    Module, ThisModule,
    alloc::{flags::GFP_KERNEL, kvec::KVec},
    c_str,
    error::Result,
    macros::module,
    pr_cont, pr_info,
    rbtree::{RBTree, RBTreeNode},
    str::CString,
};

struct RustDataStructure {
    numbers: KVec<i32>,
    map: RBTree<i32, CString>,
}

impl Module for RustDataStructure {
    fn init(_module: &'static ThisModule) -> Result<Self> {
        pr_info!("Rust data structure sample (init)\n");

        let mut numbers: KVec<i32> = KVec::new();
        () = numbers.push(960, GFP_KERNEL)?;
        () = numbers.push(110, GFP_KERNEL)?;
        pr_info!(
            "sum = {}",
            numbers
                .iter()
                .fold::<i32, fn(i32, &i32) -> i32>(0, |acc: i32, x: &i32| -> i32 {
                    acc.wrapping_add(*x)
                })
        );

        let mut map: RBTree<i32, CString> = RBTree::new();
        let _: Option<RBTreeNode<i32, CString>> =
            map.try_create_and_insert(960, c_str!("Hello").to_cstring()?, GFP_KERNEL)?;
        let _: Option<RBTreeNode<i32, CString>> =
            map.try_create_and_insert(110, c_str!("Haven").to_cstring()?, GFP_KERNEL)?;
        pr_info!(
            "sum = {:?}",
            map.iter().fold::<Option<i32>, _>(
                Some(0),
                |acc: Option<i32>, (key, _value): (&i32, &CString)| -> Option<i32> {
                    acc?.checked_add(*key)
                },
            )
        );

        Ok(RustDataStructure { numbers, map })
    }
}

impl Drop for RustDataStructure {
    fn drop(&mut self) {
        pr_info!("My numbers are {:?}\n", self.numbers);
        pr_info!("My entries are:\n");
        for (key, value) in self.map.iter() {
            pr_cont!("\t({key:?} => {value:?})\n");
        }
        pr_info!("Rust data structure sample (exit)\n");
    }
}

module! {
    type: RustDataStructure,
    name: "rust_data_structure",
    authors: ["Rick Yang"],
    description: "Rust data structure sample",
    license: "GPL",
}
