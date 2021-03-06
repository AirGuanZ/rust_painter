//! Matrix, vector, point, ...

extern crate cgmath;

use self::cgmath::*;
use math::{Clamp, Real};

pub use self::cgmath::ElementWise;
pub use self::cgmath::InnerSpace;
pub use self::cgmath::Matrix;
pub use self::cgmath::SquareMatrix;

pub use self::cgmath::{dot, vec2, vec3, vec4};

pub type Mat2f = Matrix2<Real>;
pub type Mat3f = Matrix3<Real>;
pub type Mat4f = Matrix4<Real>;

pub type Vec2f = Vector2<Real>;
pub type Vec3f = Vector3<Real>;
pub type Vec4f = Vector4<Real>;

pub const ZERO_VEC3: Vec3f = Vec3f {
    x: 0.0,
    y: 0.0,
    z: 0.0,
};

pub const X_VEC3: Vec3f = Vec3f {
    x: 1.0,
    y: 0.0,
    z: 0.0,
};

pub const Y_VEC3: Vec3f = Vec3f {
    x: 0.0,
    y: 1.0,
    z: 0.0,
};

pub const Z_VEC3: Vec3f = Vec3f {
    x: 0.0,
    y: 0.0,
    z: 1.0,
};

pub fn reflect_vec(nor: Vec3f, in_vec: Vec3f) -> Vec3f {
    2.0 * nor.dot(in_vec) * nor - in_vec
}

pub fn max_elememt_wise_vec3(a: Vec3f, b: Vec3f) -> Vec3f {
    vec3(a.x.max(b.x), a.y.max(b.y), a.z.max(b.z))
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
