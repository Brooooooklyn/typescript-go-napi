use std::borrow::Cow;
use std::ffi::CString;
use std::ptr;

use napi::bindgen_prelude::*;
use napi_derive::napi;

use crate::compiler_options::CompilerOptions;

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

        pub fn TranspileModule(input: *const c_char, filename: *const c_char) -> *const c_char;

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

#[napi(object, object_to_js = false)]
pub struct TranspileModuleOptions {
    pub filename: Option<String>,
    pub compiler_options: Option<CompilerOptions>,
}

#[napi]
pub fn transpile_module(
    input: Either<String, &[u8]>,
    options: TranspileModuleOptions,
) -> Result<RawCString> {
    let result = unsafe {
        sys::TranspileModule(
            CString::new(input.as_ref())?.into_raw(),
            CString::new(
                options
                    .filename
                    .map(Cow::Owned)
                    .unwrap_or(Cow::Borrowed("/example.ts"))
                    .as_ref(),
            )?
            .into_raw(),
        )
    };
    Ok(RawCString::new(result, NAPI_AUTO_LENGTH))
}
