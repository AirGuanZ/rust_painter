//! AABB bounding box

use math::*;

/// Axis-aligned bounding box
#[derive(Clone)]
pub struct AABB {
    lower: Vec3f,
    upper: Vec3f,
}

impl AABB {
    fn is_valid(&self) -> bool {
        self.lower.x <= self.upper.x && self.lower.y <= self.upper.y && self.lower.z <= self.upper.z
    }

    pub fn new(lower: Vec3f, upper: Vec3f) -> AABB {
        AABB { lower, upper }
    }

    pub fn get_lower(&self) -> &Vec3f {
        &self.lower
    }

    pub fn get_upper(&self) -> &Vec3f {
        &self.upper
    }

    pub fn set_lower(&mut self, lower: Vec3f) -> &mut Self {
        self.lower = lower;
        assert!(self.is_valid());
        self
    }

    pub fn set_upper(&mut self, upper: Vec3f) -> &mut Self {
        self.upper = upper;
        assert!(self.is_valid());
        self
    }

    pub fn set_box(&mut self, lower: Vec3f, upper: Vec3f) -> &mut Self {
        self.lower = lower;
        self.upper = upper;
        assert!(self.is_valid());
        self
    }

    pub fn to_aabb_bounding(&self) -> AABB {
        self.clone()
    }
}
