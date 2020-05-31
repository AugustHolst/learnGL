#![macro_use]

/// Macro to get c strings from literals without runtime overhead
/// Literal must not contain any interior nul bytes!
macro_rules! c_str {
    ($literal:expr) => {
        CStr::from_bytes_with_nul_unchecked(concat!($literal, "\0").as_bytes())
    }
}

macro_rules! c_void_pointer {
    (($array:ident),($index)) => {
        &ident[0] as *const f32 as *const c_void
    }
}