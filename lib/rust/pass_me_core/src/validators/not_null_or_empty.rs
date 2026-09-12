pub trait NotNullOrEmpty {
    fn is_not_empty(&self) -> bool;
}

impl NotNullOrEmpty for str {
    fn is_not_empty(&self) -> bool {
        !self.trim().is_empty()
    }
}

impl NotNullOrEmpty for String {
    fn is_not_empty(&self) -> bool {
        !self.trim().is_empty()
    }
}

impl<T: NotNullOrEmpty> NotNullOrEmpty for Option<T> {
    fn is_not_empty(&self) -> bool {
        match self {
            Some(value) => value.is_not_empty(),
            None => false,
        }
    }
}

impl<T> NotNullOrEmpty for Vec<T> {
    fn is_not_empty(&self) -> bool {
        !self.is_empty()
    }
}

impl<T> NotNullOrEmpty for [T] {
    fn is_not_empty(&self) -> bool {
        !self.is_empty()
    }
}

macro_rules! impl_not_empty_neq_default {
    ($($ty:ty),* $(,)?) => {
        $(
            impl NotNullOrEmpty for $ty {
                fn is_not_empty(&self) -> bool {
                    self != &<$ty as Default>::default()
                }
            }
        )*
    };
}

impl_not_empty_neq_default!(
    i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, f32, f64, bool,
);
