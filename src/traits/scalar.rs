use std::ops::{Sub, Mul, Div, Rem, Add, AddAssign, SubAssign, MulAssign, DivAssign, RemAssign};

pub trait Scalar: 
    Sized + 
    Copy + 
    Clone + 
    Add<Output = Self> + 
    Sub<Output = Self> + 
    Mul<Output = Self> + 
    Div<Output = Self> + 
    Rem<Output = Self> + 
    AddAssign +
    SubAssign +
    MulAssign +
    DivAssign +
    RemAssign +
{
    const ZERO: Self;
    const ONE: Self;
    fn min(self, rhs: Self) -> Self;
    fn max(self, rhs: Self) -> Self;
}

pub trait Float: Scalar {
    const HALF: Self;
}