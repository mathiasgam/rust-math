use std::ops::{Add, Sub, Mul, Div, Rem, AddAssign, SubAssign, MulAssign, DivAssign, RemAssign};

pub trait Num: 
    Sized + 
    Clone + 
    Copy + 
    NumOps
{}

pub trait NumOps:
    Sized +
    Add<Output = Self> + 
    Sub<Output = Self> + 
    Mul<Output = Self> + 
    Div<Output = Self> + 
    Rem<Output = Self> + 
    AddAssign +
    SubAssign +
    MulAssign +
    DivAssign +
    RemAssign
{}

macro_rules! impl_num_for_float {
    ($name:ident) => {
        impl Num for $name {}
        impl NumOps for $name {}
    };
}

impl_num_for_float!(f32);
impl_num_for_float!(f64);