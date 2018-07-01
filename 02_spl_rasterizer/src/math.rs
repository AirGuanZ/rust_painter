extern crate cgmath;
extern crate std;

pub type Real = f32;
pub const REAL_MAX: Real = std::f32::MAX;

pub type Mat2f = cgmath::Matrix2<Real>;
pub type Mat3f = cgmath::Matrix3<Real>;
pub type Mat4f = cgmath::Matrix4<Real>;
pub type Vec2f = cgmath::Vector2<Real>;
pub type Vec3f = cgmath::Vector3<Real>;
pub type Pnt3f = cgmath::Point3<Real>;

pub use cgmath::{dot, vec2, vec3, vec4};
