/// Zeros for each supported type are implemented here.
///
/// Used for generic `Sample`s.

pub trait Zero {
    fn zero() -> Self;
}
impl Zero for f64 {
    fn zero() -> Self {
        0f64
    }
}
impl Zero for f32 {
    fn zero() -> Self {
        0f32
    }
}
impl Zero for i16 {
    fn zero() -> Self {
        0i16
    }
}
impl Zero for u16 {
    fn zero() -> Self {
        0u16
    }
}
impl Zero for i32 {
    fn zero() -> Self {
        0i32
    }
}
impl Zero for u32 {
    fn zero() -> Self {
        0u32
    }
}
impl Zero for u8 {
    fn zero() -> Self {
        0u8
    }
}
impl Zero for i8 {
    fn zero() -> Self {
        0i8
    }
}
