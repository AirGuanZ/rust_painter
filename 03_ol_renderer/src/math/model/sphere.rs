//! Ideal sphere

use super::Ray;
use super::AABB;
use math::*;

/// Ideal sphere model
#[derive(Clone)]
pub struct Sphere {
    centre: Vec3f,
    radius: Real,
}

impl Sphere {
    pub fn new(centre: Vec3f, radius: Real) -> Sphere {
        Sphere { centre, radius }
    }

    pub fn get_centre(&self) -> &Vec3f {
        &self.centre
    }

    pub fn get_radius(&self) -> Real {
        self.radius
    }

    pub fn set_centre(&mut self, centre: Vec3f) -> &mut Self {
        self.centre = centre;
        self
    }

    pub fn set_radius(&mut self, radius: Real) -> &mut Self {
        self.radius = radius;
        self
    }

    /// Is a given point in the sphere
    pub fn is_point_in(&self, p: Vec3f) -> bool {
        (self.centre - p).magnitude() < self.radius
    }

    /// Is a given point out of the sphere
    pub fn is_point_out(&self, p: Vec3f) -> bool {
        (self.centre - p).magnitude() > self.radius
    }

    /// Is a given ray intersected with the sphere
    pub fn is_intersected(&self, r: Ray) -> bool {
        let pc = self.centre - r.p;
        let pt = pc.clone().project_on(r.d.clone());
        (pc - pt).magnitude() <= self.radius
    }

    /// Compute the nearest intersection with given ray
    pub fn nearest_inct(&self, r: Ray) -> Option<(Real, Vec3f)> {
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

        if t1 <= Real::default_epsilon() && t2 <= Real::default_epsilon() {
            return None;
        }

        let t1 = if t1 < 0.0 { REAL_MAX } else { t1 };
        let t = if t2 < 0.0 { t1 } else { t1.min(t2) };

        Some((t, r.t_to_point(t)))
    }

    /// 将球面上的一点转为球面参数坐标
    pub fn inct_to_uv(&self, _p: Vec3f) -> (Real, Real) {
        (0.0, 0.0) // TODO
    }

    pub fn inct_to_local_x(&self, p: Vec3f) -> Vec3f {
        self.inct_to_local_y(p).cross(self.inct_to_local_z(p))
    }

    pub fn inct_to_local_y(&self, p: Vec3f) -> Vec3f {
        (p - self.centre).normalize()
    }

    pub fn inct_to_local_z(&self, p: Vec3f) -> Vec3f {
        let ly = self.inct_to_local_y(p);
        let lyy = ly.y;
        let hor = lyy.relative_eq(&1.0, Real::default_epsilon(), Real::default_max_relative())
            || lyy.relative_eq(&-1.0, Real::default_epsilon(), Real::default_max_relative());
        if hor {
            return vec3(1.0, 0.0, 0.0);
        }
        -ly.y * vec3(ly.x, 0.0, ly.z).normalize()
            + (ly.x * ly.x + ly.z * ly.z).sqrt() * vec3(0.0, 1.0, 0.0)
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
        let sph = Sphere::new(vec3(1.0, 1.0, 0.0), 1.0);
        let ray = Ray::new(vec3(0.0, -4.0, 0.0), vec3(-1.0, 1.0, 0.0));

        assert!(sph.is_intersected(ray.clone()) == false);
        match sph.nearest_inct(ray.clone()) {
            Some(_) => panic!("Lalala"),
            None => (),
        }
    }

    #[test]
    fn inct_2() {
        let sph = Sphere::new(vec3(0.0, 0.0, 0.0), 1.0);
        let ray = Ray::new(vec3(0.0, -4.0, 0.0), vec3(0.0, 1.0, 0.0));

        assert!(sph.is_intersected(ray.clone()));
        let _ = sph.nearest_inct(ray.clone()).unwrap();
    }

    #[test]
    fn inct_to_local() {
        let sph = Sphere::new(vec3(0.0, 0.0, 0.0), 1.0);
        let lxz = sph.inct_to_local_x(vec3(1.0, 0.0, 0.0)).z;
        assert_eq!(
            lxz.relative_eq(&1.0, Real::default_epsilon(), Real::default_max_relative()),
            true
        );

        let incts = [
            vec3(1.0, 0.0, 0.0),
            vec3(0.0, 1.0, 0.0),
            vec3(1.0, 2.0, 4.0),
            vec3(5.0, 1.0, 9.0),
            vec3(12.0, -5.0, 0.0),
        ].iter()
            .map(|s| s.normalize())
            .collect::<Vec<Vec3f>>();
        for inct in incts {
            let nor = sph.inct_to_local_y(inct);
            assert!(nor.relative_eq(&inct, Real::default_epsilon(), Real::default_max_relative()));
        }

        assert!(sph.inct_to_local_z(vec3(1.0, 0.0, 0.0)).relative_eq(
            &vec3(0.0, 1.0, 0.0),
            Real::default_epsilon(),
            Real::default_max_relative()
        ));

        assert!(sph.inct_to_local_z(vec3(-1.0, 0.0, 0.0)).relative_eq(
            &vec3(0.0, 1.0, 0.0),
            Real::default_epsilon(),
            Real::default_max_relative()
        ));

        assert!(sph.inct_to_local_z(vec3(0.0, 1.0, 0.0)).relative_eq(
            &vec3(1.0, 0.0, 0.0),
            Real::default_epsilon(),
            Real::default_max_relative()
        ));
    }
}
