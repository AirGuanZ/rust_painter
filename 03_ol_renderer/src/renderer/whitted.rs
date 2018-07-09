//! (Improved) Whitted Renderer
//!
//! See Whitted, T. (1979).
//! An improved illumination model for shaded display.
//! Acm Siggraph Computer Graphics, 13(2), 14.

use entity::*;
use light::*;
use math::*;
use renderer::*;

pub struct WhittedRenderer {
    entities: Vec<Box<Entity>>,
    lights: Vec<Box<Light>>,
    background: Color3f,
    max_depth: u32,
}

impl Renderer for WhittedRenderer {
    fn render(&self, r: Ray) -> Color3f {
        self.render_d(r, 0)
    }
}

impl WhittedRenderer {
    fn render_d(&self, r: Ray, depth: u32) -> Color3f {
        if depth > self.max_depth {
            return self.background;
        }

        let mut inct: Option<Intersection> = None;
        for ent in &self.entities {
            if let Some(i) = ent.inct(r.clone()) {
                inct = match inct {
                    None => Some(i),
                    Some(v) => Some(v.nearer(i)),
                }
            }
        }

        if inct.is_none() {
            return self.background;
        }
        let inct = inct.unwrap();

        // Direct illumination
        let mut direct_illu = BLACK;
        for light in &self.lights {
            let sam = light.sample_to(1, inct.position);
            if sam.is_empty() {
                continue;
            }
            let sam = &sam[0];
            direct_illu += inct.material
                .f(-r.d, -sam.ray.d)
                .mul_element_wise(sam.color)
                * dot(-sam.ray.d, inct.normal);
        }

        // Indirect illumination
        let ref_dir = reflect_vec(inct.normal, -r.d);
        let ref_ray = Ray::new(inct.position + inct.normal * 1e-6, ref_dir);
        let indirect_illu = self.render_d(ref_ray, depth + 1)
            .mul_element_wise(inct.material.f(-r.d, ref_dir));

        direct_illu + indirect_illu + inct.material.ambient()
    }

    pub fn new(
        entities: Vec<Box<Entity>>,
        lights: Vec<Box<Light>>,
        background: Color3f,
        max_depth: u32,
    ) -> WhittedRenderer {
        WhittedRenderer {
            entities,
            lights,
            background,
            max_depth,
        }
    }
}
