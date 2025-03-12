use std::ffi::CString;

use napi::bindgen_prelude::*;
use napi_derive::napi;

mod sys {
    use std::ffi::c_char;

    #[link(name = "ts", kind = "static", modifiers = "+bundle,+whole-archive")]
    unsafe extern "C" {
        pub fn ResolveTsconfig(project: *const c_char) -> *mut c_char;
    }
}

#[napi]
pub fn resolve_tsconfig(project: String) -> Result<RawCString> {
    let resolved = unsafe { sys::ResolveTsconfig(CString::new(project)?.as_ptr()) };
    if resolved.is_null() {
        Err(Error::new(
            Status::GenericFailure,
            "Failed to resolve tsconfig",
        ))
    } else {
        Ok(RawCString::new(resolved, NAPI_AUTO_LENGTH))
    }
}
