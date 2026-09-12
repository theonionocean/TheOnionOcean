pub trait Equal {
    fn is_equal(&self, other: &Self) -> bool;
}

macro_rules! impl_equal {
    ($($ty:ty),* $(,)?) => {
        $(
            impl Equal for $ty {
                fn is_equal(&self, other: &$ty) -> bool {
                    self == other
                }
            }
        )*
    };
}

impl_equal!(
    str, String, i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, f32, f64, bool
);
