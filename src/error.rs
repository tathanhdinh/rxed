
use std::{fmt, ffi};
// use failure::{Backtrace, Context, Fail};

// pub mod super::*;
use super::gen;

#[derive(Fail, Debug)]
pub struct Error {
    xed_error: gen::xed_error_enum_t,
}

impl Error {
    pub fn new(xed_error: gen::xed_error_enum_t) -> Error {
        Error {xed_error}
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let err_str = unsafe {
            ffi::CStr::from_ptr(gen::xed_error_enum_t2str(self.xed_error))
        };
        write!(f, "{}", err_str.to_str().unwrap())
    }
}