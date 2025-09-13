//! Rust kunit tests

use kernel::macros::kunit_tests;

mod common;

#[kunit_tests(rust_kunit)]
mod tests {
    use kernel::{
        alloc::{flags::GFP_KERNEL, kvec::KVec},
        error::Result,
    };

    #[test]
    fn test_kvec() {
        let mut numbers: KVec<i32> = KVec::new();

        assert_eq!(numbers.push(72, GFP_KERNEL), Ok(()));
        assert_eq!(numbers.push(108, GFP_KERNEL), Ok(()));
        assert_eq!(numbers.push(200, GFP_KERNEL), Ok(()));

        assert_eq!(numbers, [72, 108, 200]);
    }

    #[test]
    fn test_do_bad_add1() {
        let a: i32 = 960;
        let b: i32 = 110;

        let ret: Result<i32> = crate::common::bad_add(a, b);
        assert!(ret.is_ok());

        match ret {
            Ok(v) => assert_eq!(v, 960110),
            Err(_) => assert!(false),
        }
    }

    #[test]
    fn test_do_bad_add2() {
        let a: i32 = 960;
        let b: i32 = -110;

        let ret: Result<i32> = crate::common::bad_add(a, b);
        assert!(ret.is_err());

        match ret {
            Ok(_) => assert!(false),
            Err(_) => assert!(true),
        }
    }

    #[test]
    fn test_do_bad_add3() {
        let a: i32 = -960;
        let b: i32 = 110;

        let ret: Result<i32> = crate::common::bad_add(a, b);
        assert!(ret.is_ok());

        match ret {
            Ok(v) => assert_eq!(v, -960110),
            Err(_) => assert!(false),
        }
    }
}
