//! Commonly used math tools like vector, matrix, color, etc

extern crate cgmath;
extern crate std;

pub use self::cgmath::*;

pub type Real = f32;
pub const REAL_MAX: Real = std::f32::MAX;

pub type Mat2f = Matrix2<Real>;
pub type Mat3f = Matrix3<Real>;
pub type Mat4f = Matrix4<Real>;

pub type Vec2f = Vector2<Real>;
pub type Vec3f = Vector3<Real>;
pub type Vec4f = Vector4<Real>;

pub type Pnt2f = Point2<Real>;
pub type Pnt3f = Point3<Real>;

pub type RGBf = Vec3f;
pub type RGBAf = Vec4f;

pub trait ColorTrait3<T> {
    fn r(&self) -> T;
    fn g(&self) -> T;
    fn b(&self) -> T;

    fn set_r(&mut self, v: T);
    fn set_g(&mut self, v: T);
    fn set_b(&mut self, v: T);
}

pub trait ColorTrait4<T>: ColorTrait3<T> {
    fn a(&self) -> T;

    fn set_a(&mut self, v: T);
}

impl<T: Clone> ColorTrait3<T> for Vector3<T> {
    fn r(&self) -> T {
        self.x.clone()
    }

    fn g(&self) -> T {
        self.y.clone()
    }

    fn b(&self) -> T {
        self.z.clone()
    }

    fn set_r(&mut self, v: T) {
        self.x = v;
    }

    fn set_g(&mut self, v: T) {
        self.y = v;
    }

    fn set_b(&mut self, v: T) {
        self.z = v;
    }
}

impl<T: Clone> ColorTrait3<T> for Vector4<T> {
    fn r(&self) -> T {
        self.x.clone()
    }

    fn g(&self) -> T {
        self.y.clone()
    }

    fn b(&self) -> T {
        self.z.clone()
    }

    fn set_r(&mut self, v: T) {
        self.x = v;
    }

    fn set_g(&mut self, v: T) {
        self.y = v;
    }

    fn set_b(&mut self, v: T) {
        self.z = v;
    }
}

impl<T: Clone> ColorTrait4<T> for Vector4<T> {
    fn a(&self) -> T {
        self.w.clone()
    }

    fn set_a(&mut self, v: T) {
        self.w = v;
    }
}

pub trait Clamp<T> {
    fn clamp(&self, min_v: T, max_v: T) -> Self;
}

impl Clamp<Real> for Real {
    fn clamp(&self, min_v: Real, max_v: Real) -> Self {
        self.max(min_v).min(max_v)
    }
}

impl Clamp<Real> for Vec2f {
    fn clamp(&self, min_v: Real, max_v: Real) -> Self {
        Vec2f {
            x: self.x.clamp(min_v, max_v),
            y: self.y.clamp(min_v, max_v),
        }
    }
}

impl Clamp<Real> for Vec3f {
    fn clamp(&self, min_v: Real, max_v: Real) -> Self {
        Vec3f {
            x: self.x.clamp(min_v, max_v),
            y: self.y.clamp(min_v, max_v),
            z: self.z.clamp(min_v, max_v),
        }
    }
}

impl Clamp<Real> for Vec4f {
    fn clamp(&self, min_v: Real, max_v: Real) -> Self {
        Vec4f {
            x: self.x.clamp(min_v, max_v),
            y: self.y.clamp(min_v, max_v),
            z: self.z.clamp(min_v, max_v),
            w: self.w.clamp(min_v, max_v),
        }
    }
}
