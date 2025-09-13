use {
    core::num::ParseIntError,
    kernel::{
        error::{Error, Result, code::ERANGE},
        fmt,
        str::CString,
    },
};

/// Attempts to add two integers, `a` and `b`, by concatenating their string representations,
/// parsing the concatenated string back into an integer, and returning the result.
pub(crate) fn bad_add(a: i32, b: i32) -> Result<i32> {
    CString::try_from_fmt(fmt!("{a}{b}"))?
        .to_str()?
        .parse::<i32>()
        .map_err::<Error, fn(ParseIntError) -> Error>(
            |_: ParseIntError| ERANGE, // Math result not representable.
        )
}
