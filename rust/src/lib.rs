use std::ffi::CString;
use std::ptr;

use napi::bindgen_prelude::*;
use napi_derive::napi;

mod compiler_options;
mod error;

mod sys {
    use std::ffi::{c_char, c_void};

    #[link(name = "ts", kind = "static", modifiers = "+bundle,+whole-archive")]
    unsafe extern "C" {
        pub fn RunProject(
            project: *const c_char,
            diagnostics: *mut *mut c_void,
            length: *mut usize,
        );

        pub fn Transform(input: *const c_char, filename: *const c_char) -> *const c_char;

    }
}

#[napi]
pub fn run_project(project: String) -> Result<()> {
    let mut diagnostics = ptr::null_mut();
    let mut length = 0;
    unsafe {
        sys::RunProject(
            CString::new(project)?.as_ptr(),
            &mut diagnostics,
            &mut length,
        )
    };

    Ok(())
}

#[napi]
pub fn transform(input: String, filename: String) -> Result<RawCString> {
    let result = unsafe {
        sys::Transform(
            CString::new(input)?.into_raw(),
            CString::new(filename)?.into_raw(),
        )
    };
    Ok(RawCString::new(result, NAPI_AUTO_LENGTH))
}
