pub trait NumberValueLessThan {
    fn is_less_than(&self, value: &Self) -> bool;
}

pub trait NumberValueLessThanOrEqual {
    fn is_less_than_or_equal(&self, value: &Self) -> bool;
}

pub trait NumberValueGreaterThan {
    fn is_greater_than(&self, value: &Self) -> bool;
}

pub trait NumberValueGreaterThanOrEqual {
    fn is_greater_than_or_equal(&self, value: &Self) -> bool;
}

pub trait ExclusiveBetween {
    fn is_exclusive_between(&self, min: &Self, max: &Self) -> bool;
}

pub trait InclusiveBetween {
    fn is_inclusive_between(&self, min: &Self, max: &Self) -> bool;
}

macro_rules! impl_equal {
    ($($ty:ty),* $(,)?) => {
        $(
            impl NumberValueLessThan for $ty {
                fn is_less_than(&self, other: &$ty) -> bool {
                    self < other
                }
            }
        )*
    };
}

macro_rules! impl_less_than_or_equal {
    ($($ty:ty),* $(,)?) => {
        $(
            impl NumberValueLessThanOrEqual for $ty {
                fn is_less_than_or_equal(&self, other: &$ty) -> bool {
                    self <= other
                }
            }
        )*
    };
}

macro_rules! impl_greater_than {
    ($($ty:ty),* $(,)?) => {
        $(
            impl NumberValueGreaterThan for $ty {
                fn is_greater_than(&self, other: &$ty) -> bool {
                    self > other
                }
            }
        )*
    };
}

macro_rules! impl_greater_than_or_equal {
    ($($ty:ty),* $(,)?) => {
        $(
            impl NumberValueGreaterThanOrEqual for $ty {
                fn is_greater_than_or_equal(&self, other: &$ty) -> bool {
                    self >= other
                }
            }
        )*
    };
}

macro_rules! impl_exclusive_between {
    ($($ty:ty),* $(,)?) => {
        $(
            impl ExclusiveBetween for $ty {
                fn is_exclusive_between(&self, min: &$ty, max: &$ty) -> bool {
                    self > min && self < max
                }
            }
        )*
    };
}

macro_rules! impl_inclusive_between {
    ($($ty:ty),* $(,)?) => {
        $(
            impl InclusiveBetween for $ty {
                fn is_inclusive_between(&self, min: &$ty, max: &$ty) -> bool {
                    self >= min && self <= max
                }
            }
        )*
    };
}

impl_equal!(i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, f32, f64, bool);
impl_less_than_or_equal!(
    i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, f32, f64, bool
);
impl_greater_than!(i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, f32, f64, bool);
impl_greater_than_or_equal!(
    i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, f32, f64, bool
);
impl_exclusive_between!(i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, f32, f64);
impl_inclusive_between!(i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize, f32, f64);
