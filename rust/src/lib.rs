use std::ffi::CString;
use std::ptr;

use napi::bindgen_prelude::*;
use napi_derive::napi;

mod sys {
    use std::ffi::{c_char, c_void};

    #[link(name = "ts", kind = "static", modifiers = "+bundle,+whole-archive")]
    unsafe extern "C" {
        pub fn RunProject(
            project: *const c_char,
            diagnostics: *mut *mut c_void,
            length: *mut usize,
        );
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
