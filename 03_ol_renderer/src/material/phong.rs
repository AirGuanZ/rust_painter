//! Phong lighting model

extern crate rand;

use material::*;
use math::*;

/// Phong lighting model:
///
/// L = Specular * cos(Vout, Ref(Vin, Normal)) + Ambient
///
pub struct Phong {
    ambient: Color3f,
    specular: Color3f,
    trans: Mat3f,
    local_y: Vec3f,
    shininess: Real,
}

impl BxDF for Phong {
    fn get_type(&self) -> BxDFType {
        BxDFType::BRDF
    }

    fn ambient(&self) -> Color3f {
        self.ambient
    }

    fn emit(&self, _v: Vec3f) -> Color3f {
        BLACK
    }

    fn f(&self, vin: Vec3f, vout: Vec3f) -> Color3f {
        if dot(vin, self.local_y) <= 0.0 || dot(vout, self.local_y) <= 0.0 {
            return BLACK;
        }
        let r = reflect_vec(self.local_y, vin.normalize());
        let alpha = dot(r, vout).max(0.0);
        self.specular * alpha.powf(self.shininess)
    }

    fn sample(&self, _v: &Vec3f, n: u32) -> Vec<Vec3f> {
        let mut ret = Vec::with_capacity(n as usize);
        for _ in 0..n {
            ret.push(self.trans * hemisphere_uniform());
        }
        ret
    }

    fn pdf(&self, _v: &Vec3f, _vsample: &Vec3f) -> Real {
        1.0 / (2.0 * REAL_PI)
    }
}

impl Phong {
    pub fn new(
        ambient: Color3f,
        specular: Color3f,
        local_x: Vec3f,
        local_y: Vec3f,
        shininess: Real,
    ) -> Phong {
        let local_z = local_x.cross(local_y);
        let trans = Mat3f::from_cols(local_x, local_y, local_z);
        Phong {
            ambient,
            specular,
            trans,
            local_y,
            shininess,
        }
    }
}

pub struct PhongPrototype {
    pub ambient: Color3f,
    pub specular: Color3f,
    pub shininess: Real,
}

impl PhongPrototype {
    pub fn gen_phong(&self, lx: Vec3f, ly: Vec3f) -> Phong {
        Phong::new(self.ambient, self.specular, lx, ly, self.shininess)
    }
}
