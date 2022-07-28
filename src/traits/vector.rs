use super::scalar::Scalar;

pub trait Vector<T: Scalar>: Sized + Copy + Clone {
    const ZERO: Self;
    const ONE: Self;

    fn splat(value: T) -> Self;

    fn add(self, rhs: Self) -> Self;
    fn sub(self, rhs: Self) -> Self;
    fn mul(self, rhs: Self) -> Self;
    fn div(self, rhs: Self) -> Self;
    fn rem(self, rhs: Self) -> Self;

    fn add_scalar(self, rhs: T) -> Self;
    fn sub_scalar(self, rhs: T) -> Self;
    fn mul_scalar(self, rhs: T) -> Self;
    fn div_scalar(self, rhs: T) -> Self;
    fn rem_scalar(self, rhs: T) -> Self;

    #[inline(always)]
    fn scale(self, rhs: T) -> Self {
        self.mul_scalar(rhs)
    }

    fn min(self, rhs: Self) -> Self;
    fn max(self, rhs: Self) -> Self;
    fn min_element(self) -> T;
    fn max_element(self) -> T;
    fn clamp(self, min: Self, max: Self) -> Self;

    fn dot(self, rhs: Self) -> T;
    fn dot_into_vec(self, rhs: Self) -> Self {
        Self::splat(self.dot(rhs))
    }
}

pub trait Vector2<T: Scalar>: Vector<T> {
    const X: Self;
    const Y: Self;

    fn new(x: T, y: T) -> Self;
    fn x(self) -> T;
    fn y(self) -> T;

    #[inline(always)]
    fn splat_x(self) -> Self {
        Self::splat(self.x())
    }
    #[inline(always)]
    fn splat_y(self) -> Self {
        Self::splat(self.y())
    }
}

pub trait Vector3<T: Scalar>: Vector<T> {
    const X: Self;
    const Y: Self;
    const Z: Self;

    fn new(x: T, y: T, z: T) -> Self;
    fn x(self) -> T;
    fn y(self) -> T;
    fn z(self) -> T;

    #[inline(always)]
    fn splat_x(self) -> Self {
        Self::splat(self.x())
    }
    #[inline(always)]
    fn splat_y(self) -> Self {
        Self::splat(self.y())
    }
    #[inline(always)]
    fn splat_z(self) -> Self {
        Self::splat(self.z())
    }
}

pub trait Vector4<T: Scalar>: Vector<T> {
    const X: Self;
    const Y: Self;
    const Z: Self;
    const W: Self;

    fn new(x: T, y: T, z: T, w: T) -> Self;
    fn x(self) -> T;
    fn y(self) -> T;
    fn z(self) -> T;
    fn w(self) -> T;

    #[inline(always)]
    fn splat_x(self) -> Self {
        Self::splat(self.x())
    }
    #[inline(always)]
    fn splat_y(self) -> Self {
        Self::splat(self.y())
    }
    #[inline(always)]
    fn splat_z(self) -> Self {
        Self::splat(self.z())
    }
    #[inline(always)]
    fn splat_w(self) -> Self {
        Self::splat(self.w())
    }
}