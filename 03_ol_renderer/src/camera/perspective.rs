/// Perspective (Pinhole) camera model

use camera::*;
use math::*;

/// Perspective camera model
pub struct PerspectiveCamera {
    eye: Vec3f,
    scr_o: Vec3f,
    scr_x: Vec3f,
    scr_y: Vec3f,
}

impl Camera for PerspectiveCamera {
    fn scr_to_ray(&self, scr_point: Vec3f) -> Ray {
        let pnt = self.scr_o + scr_point.x * self.scr_x + scr_point.y * self.scr_y;
        Ray::new(pnt, (pnt - self.eye).normalize())
    }
}

impl PerspectiveCamera {
    pub fn new(
        eye: Vec3f,
        look_at: Vec3f,
        up_dir: Vec3f,
        scr_width: Real,
        scr_height: Real,
        near_dis: Real,
    ) -> PerspectiveCamera {
        let dir = (look_at - eye).normalize();
        let scr_y = scr_height / 2.0 * (up_dir - dot(up_dir, dir) * dir).normalize();
        PerspectiveCamera {
            eye,
            scr_o: eye + near_dis * dir,
            scr_x: scr_width / 2.0 * (dir.cross(scr_y)),
            scr_y,
        }
    }
}
