use std::time::{UNIX_EPOCH, SystemTime};
use std::ops::{Sub, Add, Rem};

mod traits;
use traits::ToFromI128;




/// Implement for all primitive signed/unsigned integer types
macro_rules! impl_to_from_i128 {
    ($($t:ty),*) => {
        $(
            impl ToFromI128 for $t {
                fn to_i128(self) -> i128 {
                    self as i128
                }
                fn from_i128(v: i128) -> Self {
                    v as $t
                }
            }
        )*
    };
}

#[cfg(feature = "num-u8")]
impl_to_from_i128!(u8);

#[cfg(feature = "num-u16")]
impl_to_from_i128!(u16);

#[cfg(feature = "num-u32")]
impl_to_from_i128!(u32);

#[cfg(feature = "num-u64")]
impl_to_from_i128!(u64);

impl_to_from_i128!(u128);

#[cfg(feature = "num-i8")]
impl_to_from_i128!(i8);

#[cfg(feature = "num-i16")]
impl_to_from_i128!(i16);

#[cfg(feature = "num-i32")]
impl_to_from_i128!(i32);

#[cfg(feature = "num-i64")]
impl_to_from_i128!(i64);

#[cfg(feature = "num-i128")]
impl_to_from_i128!(i128);


fn random_base() -> i128 {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let mut x = nanos;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        x.to_i128()
}

pub struct RandomInt<T> {
    min: T,
    max: T
}

impl<T> RandomInt<T>
where
    T: Copy + ToFromI128 + Add<Output = T> + Sub<Output = T> + Rem<Output = T>, u32: Add<T>{
    pub fn new(min: T, max: T) -> T {
        let rng = Self { min, max };
        rng.random()

    }

    fn random(&self) -> T {
        let min = self.min.to_i128();
        let max = self.max.to_i128() + 1;
        let range = (max.wrapping_sub(min)).max(1);
        let r = random_base() % range;
        T::from_i128(r.wrapping_add(min))
    }
    
}