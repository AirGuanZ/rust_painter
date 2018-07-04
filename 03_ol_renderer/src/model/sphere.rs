//! Ideal sphere

use math::*;
use model::AABB;

/// Ideal sphere model
#[derive(Clone)]
pub struct Sphere {
    centre: Pnt3f,
    radius: Real,
}

impl Sphere {
    pub fn new(centre: Pnt3f, radius: Real) -> Sphere {
        Sphere { centre, radius }
    }

    pub fn get_centre(&self) -> &Pnt3f {
        &self.centre
    }

    pub fn get_radius(&self) -> Real {
        self.radius
    }

    pub fn set_centre(&mut self, centre: Pnt3f) -> &mut Self {
        self.centre = centre;
        self
    }

    pub fn set_radius(&mut self, radius: Real) -> &mut Self {
        self.radius = radius;
        self
    }

    /// Is a given point in the interior of the sphere
    pub fn is_point_in(&self, p: Pnt3f) -> bool {
        (self.centre - p).magnitude() < self.radius
    }

    /// Is a given point in the outside of the sphere
    pub fn is_point_out(&self, p: Pnt3f) -> bool {
        (self.centre - p).magnitude() > self.radius
    }

    /// Is a given ray intersected with the sphere
    pub fn is_intersected(&self, r: Ray) -> bool {
        let pc = self.centre - r.p;
        let pt = pc.clone().project_on(r.d.clone());
        (pc - pt).magnitude() <= self.radius
    }

    /// Compute the nearest intersection with given ray
    pub fn nearest_inct_point(&self, r: Ray) -> Option<(Real, Pnt3f)> {
        // (p + td - c)^2 = R^2，求小的那个t
        let p_c = r.p - self.centre;
        let a = r.d.magnitude2();
        let b = 2.0 * r.d.dot(p_c);
        let c = p_c.magnitude2() - self.radius * self.radius;

        let delta = b * b - 4.0 * a * c;
        if delta < 0.0 {
            return None;
        }
        let delta = delta.sqrt();

        let recip_2_a = 0.5 / a;
        let t1 = (-b + delta) * recip_2_a;
        let t2 = (-b - delta) * recip_2_a;

        if t1 <= 0.0 && t2 <= 0.0 {
            return None;
        }

        let t1 = if t1 < 0.0 { REAL_MAX } else { t1 };
        let t = if t2 < 0.0 { t1 } else { t1.min(t2) };

        Some((t, r.t_to_point(t)))
    }

    /// Axis-aligned bounding box
    pub fn to_aabb_bounding(&self) -> AABB {
        let rv = vec3(self.radius, self.radius, self.radius);
        AABB::new(self.centre - rv, self.centre + rv)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inct_0() {
        let sph = Sphere::new(pnt3(1.0, 1.0, 0.0), 1.0);
        let ray = Ray::new(pnt3(0.0, -4.0, 0.0), vec3(-1.0, 1.0, 0.0));

        assert!(sph.is_intersected(ray.clone()) == false);
        match sph.nearest_inct_point(ray.clone()) {
            Some(_) => panic!("Lalala"),
            None => (),
        }
    }
}
