//! Matrix, Vector, Point, ...

extern crate cgmath;

use self::cgmath::*;
use math::{Clamp, Real};

pub use self::cgmath::ElementWise;
pub use self::cgmath::InnerSpace;

pub type Mat2f = Matrix2<Real>;
pub type Mat3f = Matrix3<Real>;
pub type Mat4f = Matrix4<Real>;

pub type Vec2f = Vector2<Real>;
pub type Vec3f = Vector3<Real>;
pub type Vec4f = Vector4<Real>;

pub type Pnt2f = Point2<Real>;
pub type Pnt3f = Point3<Real>;

pub fn reflect_vec(nor: Vec3f, in_vec: Vec3f) -> Vec3f {
    2.0 * nor.dot(in_vec) * nor - in_vec
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
