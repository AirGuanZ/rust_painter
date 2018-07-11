//! BxDF Combinator

extern crate rand;

use material::*;
use math::*;

fn single_vec_to_option(v: Vec<Vec3f>) -> Option<Vec3f> {
    if v.is_empty() {
        None
    } else {
        Some(v[0])
    }
}

pub struct MulBxDF {
    a: Box<BxDF>,
    b: Box<BxDF>,
    t: BxDFType,
    afac: Real,
    bfac: Real,
}

impl BxDF for MulBxDF {
    fn get_type(&self) -> BxDFType {
        self.t.clone()
    }

    fn ambient(&self) -> Color3f {
        max_elememt_wise_color3(self.a.ambient(), self.b.ambient())
    }

    fn emit(&self, v: Vec3f) -> Color3f {
        self.a.emit(v) + self.b.emit(v)
    }

    fn f(&self, vin: Vec3f, vout: Vec3f) -> Color3f {
        self.a.f(vin, vout).mul_element_wise(self.b.f(vin, vout))
    }

    fn sample(&self, v: &Vec3f, n: u32) -> Vec<Vec3f> {
        (0..n)
            .map(|_| self.sample_once(v))
            .filter(|s| s.is_some())
            .map(|s| s.unwrap())
            .collect()
    }

    fn pdf(&self, v: &Vec3f, vsample: &Vec3f) -> Real {
        let apdf = self.afac * self.a.pdf(v, vsample);
        let bpdf = self.bfac * self.b.pdf(v, vsample);
        apdf + bpdf
    }

    fn sample_upper(&self, v: &Vec3f, n: u32) -> Vec<Vec3f> {
        (0..n)
            .map(|_| self.sample_once_upper(v))
            .filter(|s| s.is_some())
            .map(|s| s.unwrap())
            .collect()
    }

    fn pdf_upper(&self, v: &Vec3f, vsample: &Vec3f) -> Real {
        let apdf = self.afac * self.a.pdf_upper(v, vsample);
        let bpdf = self.bfac * self.b.pdf_upper(v, vsample);
        apdf + bpdf
    }
}

impl MulBxDF {
    fn sample_once(&self, v: &Vec3f) -> Option<Vec3f> {
        let df = if rand::random::<Real>() < self.afac {
            &self.a
        } else {
            &self.b
        };
        match self.t {
            BxDFType::BRDF => single_vec_to_option(df.sample_upper(v, 1)),
            BxDFType::BSDF => single_vec_to_option(df.sample(v, 1)),
        }
    }

    fn sample_once_upper(&self, v: &Vec3f) -> Option<Vec3f> {
        if rand::random::<Real>() < self.afac {
            single_vec_to_option(self.a.sample_upper(v, 1))
        } else {
            single_vec_to_option(self.b.sample_upper(v, 1))
        }
    }

    fn mul_type(type_a: BxDFType, type_b: BxDFType) -> BxDFType {
        if type_a == BxDFType::BSDF && type_b == BxDFType::BSDF {
            BxDFType::BSDF
        } else {
            BxDFType::BRDF
        }
    }

    pub fn new(a: Box<BxDF>, b: Box<BxDF>) -> MulBxDF {
        let t = Self::mul_type(a.get_type(), b.get_type());
        MulBxDF {
            a,
            b,
            t,
            afac: 0.5,
            bfac: 0.5,
        }
    }
}

pub struct AddBxDF {
    a: Box<BxDF>,
    b: Box<BxDF>,
    t: BxDFType,
    afac: Real,
    bfac: Real,
}

impl BxDF for AddBxDF {
    fn get_type(&self) -> BxDFType {
        self.t.clone()
    }

    fn ambient(&self) -> Color3f {
        self.a.ambient() + self.b.ambient()
    }

    fn emit(&self, v: Vec3f) -> Color3f {
        self.a.emit(v) + self.b.emit(v)
    }

    fn f(&self, vin: Vec3f, vout: Vec3f) -> Color3f {
        self.a.f(vin, vout) + self.b.f(vin, vout)
    }

    fn sample(&self, v: &Vec3f, n: u32) -> Vec<Vec3f> {
        (0..n)
            .map(|_| self.sample_once(v))
            .filter(|s| s.is_some())
            .map(|s| s.unwrap())
            .collect()
    }

    fn pdf(&self, v: &Vec3f, vsample: &Vec3f) -> Real {
        let apdf = self.afac * self.a.pdf(v, vsample);
        let bpdf = self.bfac * self.b.pdf(v, vsample);
        apdf + bpdf
    }

    fn sample_upper(&self, v: &Vec3f, n: u32) -> Vec<Vec3f> {
        (0..n)
            .map(|_| self.sample_once_upper(v))
            .filter(|s| s.is_some())
            .map(|s| s.unwrap())
            .collect()
    }

    fn pdf_upper(&self, v: &Vec3f, vsample: &Vec3f) -> Real {
        let apdf = self.afac * self.a.pdf_upper(v, vsample);
        let bpdf = self.bfac * self.b.pdf_upper(v, vsample);
        apdf + bpdf
    }
}

impl AddBxDF {
    fn sample_once(&self, v: &Vec3f) -> Option<Vec3f> {
        let df = if rand::random::<Real>() < self.afac {
            &self.a
        } else {
            &self.b
        };
        match self.t {
            BxDFType::BRDF => single_vec_to_option(df.sample_upper(v, 1)),
            BxDFType::BSDF => single_vec_to_option(df.sample(v, 1)),
        }
    }

    fn sample_once_upper(&self, v: &Vec3f) -> Option<Vec3f> {
        if rand::random::<Real>() < self.afac {
            single_vec_to_option(self.a.sample_upper(v, 1))
        } else {
            single_vec_to_option(self.b.sample_upper(v, 1))
        }
    }

    fn mul_type(type_a: BxDFType, type_b: BxDFType) -> BxDFType {
        if type_a == BxDFType::BSDF && type_b == BxDFType::BSDF {
            BxDFType::BSDF
        } else {
            BxDFType::BRDF
        }
    }

    pub fn new(a: Box<BxDF>, b: Box<BxDF>) -> MulBxDF {
        let t = Self::mul_type(a.get_type(), b.get_type());
        MulBxDF {
            a,
            b,
            t,
            afac: 0.5,
            bfac: 0.5,
        }
    }
}
