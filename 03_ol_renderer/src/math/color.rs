//! Color traits and impl for vectors

extern crate cgmath;

use self::cgmath::*;
use math::Real;

pub type Color3f = Vector3<Real>;
pub type Color4f = Vector4<Real>;

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

pub fn color3<T>(r: T, g: T, b: T) -> Vector3<T> {
    vec3(r, g, b)
}

pub fn color4<T>(r: T, g: T, b: T, a: T) -> Vector4<T> {
    vec4(r, g, b, a)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn color() {
        let c = Color3f::new(1.0, 1.0, 0.0);
        assert_eq!(c.r(), 1.0);
        assert_eq!(c.b(), 0.0);

        let mut c = Color4f::new(0.0, 0.0, 0.5, 1.0);
        assert_eq!(c.b(), 0.5);
        assert_eq!(c.a(), 1.0);

        c += Color4f::new(1.0, 0.0, 0.0, 0.0);
        assert_eq!(c.r(), 1.0);
        assert_eq!(c.g(), 0.0);
    }
}
