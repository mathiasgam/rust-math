pub mod num;
pub mod float;
pub mod vector;

pub trait One {
    const ONE: Self;
}

pub trait Zero {
    const ZERO: Self;
}