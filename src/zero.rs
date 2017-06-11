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


