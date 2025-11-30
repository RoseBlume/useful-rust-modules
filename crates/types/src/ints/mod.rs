pub trait IntExt {
    fn is_even(&self) -> bool;
    fn is_odd(&self) -> bool;
    fn is_prime(self) -> bool;
    fn as_hex_char(&self) -> char;
    fn as_bin_string(&self) -> String;
}

macro_rules! impl_int_ext {
    ($($t:ty),*) => {
        $(
            impl IntExt for $t {
                fn is_even(&self) -> bool {
                    self % 2 == 0
                }
                fn is_odd(&self) -> bool {
                    self % 2 != 0
                }
                fn is_prime(self) -> bool {
                    if self < 2 {
                        return false;
                    }
                    if self <= 3 {
                        return true;
                    }
                    if (self % 2 == 0) || (self % 3 == 0) {
                        return false;
                    }
                    let mut i = 5;
                    while (i * i) <= self {
                        if self % i == 0 || self % (i + 2) == 0 {
                            return false;
                        }
                        i += 6;
                    }
                    return true;
                }
                fn as_hex_char(&self) -> char {
                    const HEX_CHARACTERS: [u8; 16] = *b"0123456789ABCDEF";
                    let index: usize = (self % 16).try_into().unwrap();
                    HEX_CHARACTERS[index].into()
                }
                fn as_bin_string(&self) -> String {
                    let mut n = *self;
                    if n == 0 {
                        return "0".to_string();
                    }

                    let mut binary = String::new();
                    while n > 0 {
                        binary = format!("{}{}", n % 2, binary);
                        n /= 2;
                    }
                    binary
                }
            }
        )*
    }
}

impl_int_ext!(
    u8, u16, u32, u64, u128, usize,
    i8, i16, i32, i64, i128, isize
);
