use std::os::raw::c_char;
use std::ffi::{CString, CStr};
use encoding::{EncoderTrap, DecoderTrap};
use encoding::all::GB18030;
use encoding::Encoding; // encode() & decode() for GB18030

#[derive(Debug)]
pub struct UnicodeEncodeError(String);

impl<T> From<T> for UnicodeEncodeError where T: std::string::ToString {
    fn from(errmsg: T) -> UnicodeEncodeError {
        UnicodeEncodeError(errmsg.to_string())
    }
}

#[inline]
pub fn gb18030_encode(src: &str) -> Result<CString, UnicodeEncodeError> {
    let encoded = GB18030.encode(src, EncoderTrap::Ignore)?;
    Ok(CString::new(encoded)?)
}

#[macro_export]
macro_rules! gb18030 {
    ($fmt: expr) => { cqrs::gb18030_encode($fmt).unwrap().as_c_str().as_ptr() };
    ($fmt: expr, $($args: tt)*) => { cqrs::gb18030_encode(&format!($fmt, $($args)*)).unwrap().as_c_str().as_ptr() }
}

#[inline]
pub unsafe fn gb18030_decode(ptr: *const c_char) -> Result<String, UnicodeDecodeError> {
    Ok(GB18030.decode(CStr::from_ptr(ptr).to_bytes(), DecoderTrap::Ignore)?)
}

#[derive(Debug)]
pub struct UnicodeDecodeError(String);

impl<T> From<T> for UnicodeDecodeError where T: std::string::ToString {
    fn from(errmsg: T) -> UnicodeDecodeError {
        UnicodeDecodeError(errmsg.to_string())
    }
}
