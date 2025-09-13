#![allow(missing_docs)]

fn do_something_bad(x: &i32) {
    #[allow(invalid_reference_casting)]
    unsafe {
        *(x as *const i32 as *mut i32) += 1
    };
}

fn main() {
    let x: i32 = 42;
    () = do_something_bad(&x);
    dbg!(x);
}
