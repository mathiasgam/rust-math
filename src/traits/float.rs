use std::ops::Neg;

use super::num::Num;

pub trait Float: 
    Num + 
    PartialOrd + 
    Neg<Output = Self> 
{
    fn floor(self) -> Self;
    fn ceil(self) -> Self;
    fn round(self) -> Self;
    fn trunc(self) -> Self;
    fn fract(self) -> Self;
    fn abs(self) -> Self;
    fn signum(self) -> Self;
    fn mul_add(self, a: Self, b: Self) -> Self;
    fn sqrt(self) -> Self;
    fn min(self, rhs: Self) -> Self;
    fn max(self, rhs: Self) -> Self;
    fn sin(self) -> Self;
    fn cos(self) -> Self;
    fn tan(self) -> Self;
    fn asin(self) -> Self;
    fn acos(self) -> Self;
    fn atan(self) -> Self;
    fn atan2(self, other: Self) -> Self;
}

macro_rules! forward {
    ($( Self :: $method:ident ( self $( , $arg:ident : $ty:ty )* ) -> $ret:ty ;)*) => {$(
        #[inline]
        fn $method(self $( , $arg : $ty )* ) -> $ret {
            Self::$method(self $( , $arg )* )
        }
    )*};
}


impl Float for f32 {
    forward! {
        Self::floor(self) -> Self;
        Self::ceil(self) -> Self;
        Self::round(self) -> Self;
        Self::trunc(self) -> Self;
        Self::fract(self) -> Self;
        Self::abs(self) -> Self;
        Self::signum(self) -> Self;
        Self::mul_add(self, a: Self, b: Self) -> Self;
        Self::sqrt(self) -> Self;
        Self::min(self, rhs: Self) -> Self;
        Self::max(self, rhs: Self) -> Self;
        Self::sin(self) -> Self;
        Self::cos(self) -> Self;
        Self::tan(self) -> Self;
        Self::asin(self) -> Self;
        Self::acos(self) -> Self;
        Self::atan(self) -> Self;
        Self::atan2(self, other: Self) -> Self;
    }
}

impl Float for f64 {
    forward! {
        Self::floor(self) -> Self;
        Self::ceil(self) -> Self;
        Self::round(self) -> Self;
        Self::trunc(self) -> Self;
        Self::fract(self) -> Self;
        Self::abs(self) -> Self;
        Self::signum(self) -> Self;
        Self::mul_add(self, a: Self, b: Self) -> Self;
        Self::sqrt(self) -> Self;
        Self::min(self, rhs: Self) -> Self;
        Self::max(self, rhs: Self) -> Self;
        Self::sin(self) -> Self;
        Self::cos(self) -> Self;
        Self::tan(self) -> Self;
        Self::asin(self) -> Self;
        Self::acos(self) -> Self;
        Self::atan(self) -> Self;
        Self::atan2(self, other: Self) -> Self;
    }
}