//! Commonly used sampling strategy

extern crate rand;

use math::*;

/// Uniform sampling on hemisphere (top facing positive y-direction)
pub fn hemisphere_uniform() -> Vec3f {
    let y = rand::random::<Real>();
    let r = (1.0 - y * y).sqrt();
    let phi = rand::random::<Real>() * 2.0 * REAL_PI;
    vec3(r * phi.cos(), y, r * phi.sin())
}

pub fn sphere_uniform() -> Vec3f {
    let ret = hemisphere_uniform();
    if rand::random::<Real>() < 0.5 {
        ret
    } else {
        vec3(ret.x, -ret.y, ret.z)
    }
}
