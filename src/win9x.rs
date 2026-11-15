//! Windows XP compatibility shim for the rust9x target.
//!
//! The rust9x target can still be linked against a modern MSVC CRT, which
//! imports `InitializeCriticalSectionEx`. XP exports `InitializeCriticalSection`
//! instead, so provide a local definition; the CRT's reference then resolves to
//! this code instead of a missing kernel32 import.

#![allow(non_snake_case)]

use core::ffi::{c_int, c_void};

type Bool = c_int;

#[link(name = "kernel32")]
unsafe extern "system" {
    fn InitializeCriticalSection(cs: *mut c_void);
}

#[unsafe(no_mangle)]
pub unsafe extern "system" fn rust9x_initialize_critical_section_ex(
    cs: *mut c_void,
    _spin_count: u32,
    _flags: u32,
) -> Bool {
    if cs.is_null() {
        return 0;
    }
    unsafe { InitializeCriticalSection(cs) };
    1
}

pub fn keep_shims() {
    // Force the linker to keep this definition so the CRT resolves its
    // reference locally instead of importing a Vista-only symbol.
    let _ = core::hint::black_box(
        rust9x_initialize_critical_section_ex
            as unsafe extern "system" fn(*mut c_void, u32, u32) -> Bool as usize,
    );
}
