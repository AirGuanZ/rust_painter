//! Point light source

use light::*;
use math::*;

pub struct PointLight {
    pos: Vec3f,
    color: Color3f,
    dis_fac: Vec3f,
}
