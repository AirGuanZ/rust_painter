//! Triangle

use std::ops::Index;
use std::ops::IndexMut;
use super::Ray;
use super::AABB;
use math::*;

#[derive(Clone)]
pub struct Triangle {
    vtx: [Vec3f; 3],
}

impl Index<usize> for Triangle {
    type Output = Vec3f;
    fn index(&self, index: usize) -> &Self::Output {
        assert!(index < 3);
        self.vtx[index]
    }
}

impl IndexMut<usize> for Triangle {
    type Output = Vec3f;
    fn index(&mut self, index: usize) -> &mut Self::Output {
        assert!(index < 3);
        self.vtx[index]
    }
}

impl Triangle {
    pub fn new(a: Vec3f, b: Vec3f, c: Vec3f) -> Triangle {
        Triangle { [a, b, c] }
    }

    pub fn is_intersected(&self, r: Ray) -> bool {
        false
    }
}
