use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString, rand_core::OsRng},
};

use std::ffi::{CStr, CString};
use std::os::raw::c_char;

#[unsafe(no_mangle)]
pub extern "C" fn hash_password(password: *const c_char) -> *mut c_char {
    if password.is_null() {
        return std::ptr::null_mut();
    }

    let password = unsafe {
        match CStr::from_ptr(password).to_str() {
            Ok(p) => p,
            Err(_) => return std::ptr::null_mut(),
        }
    };

    let salt = SaltString::generate(&mut OsRng);

    let hash = match Argon2::default().hash_password(password.as_bytes(), &salt) {
        Ok(h) => h.to_string(),
        Err(_) => return std::ptr::null_mut(),
    };

    match CString::new(hash) {
        Ok(s) => s.into_raw(),
        Err(_) => std::ptr::null_mut(),
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn verify_password(password: *const c_char, hash: *const c_char) -> i32 {
    if password.is_null() || hash.is_null() {
        return 0;
    }

    let password = unsafe {
        match CStr::from_ptr(password).to_str() {
            Ok(p) => p,
            Err(_) => return 0,
        }
    };

    let hash = unsafe {
        match CStr::from_ptr(hash).to_str() {
            Ok(h) => h,
            Err(_) => return 0,
        }
    };

    let parsed_hash = match PasswordHash::new(hash) {
        Ok(h) => h,
        Err(_) => return 0,
    };

    if Argon2::default()
        .verify_password(password.as_bytes(), &parsed_hash)
        .is_ok()
    {
        1
    } else {
        0
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn free_string(ptr: *mut c_char) {
    if ptr.is_null() {
        return;
    }

    unsafe {
        let _ = CString::from_raw(ptr);
    }
}
