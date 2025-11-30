use libc::{c_char, c_int, c_float};

unsafe extern "C" {
    pub fn launch_score_kernel(
        ciphertext: *const c_char,
        text_len: c_int,
        keys: *const c_char,
        num_keys: c_int,
        scores: *mut c_float,
    );
}
