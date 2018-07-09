//! Path tracing renderer

use entity::*;
use light::*;
use material::*;
use math::*;
use renderer::*;

pub struct PathTracer {
    entities: Vec<Box<Entity>>,
    lights: Vec<Box<Light>>,
    background: Color3f,
    max_depth: u32,
    spp: u32,
}

impl Renderer for PathTracer {
    fn is_visible(&self, p1: Vec3f, p2: Vec3f) -> bool {
        let d = p2 - p1;
        let dis = d.magnitude() - 1e-6;
        let r = Ray::new(p1, d);
        !self.entities.iter().any(|ent| {
            if let Some(i) = ent.has_inct(r.clone()) {
                i.0 < dis
            } else {
                false
            }
        })
    }

    fn render(&self, r: Ray) -> Color3f {
        self.render_d(r, 0)
    }
}

impl PathTracer {
    fn render_d(&self, r: Ray, depth: u32) -> Color3f {
        if depth > self.max_depth {
            return BLACK;
        }

        let inct = self.entities.iter().fold(None, |acc, ent| {
            if let Some(i) = ent.inct(r.clone()) {
                match acc {
                    None => Some(i),
                    Some(v) => Some(v.nearer(i)),
                }
            } else {
                acc
            }
        });

        if inct.is_none() {
            return self.background;
        }
        let inct = inct.unwrap();

        self.direct_illu(inct.position, -r.d, &inct.material, depth)
            + self.indirect_illu(inct.position, -r.d, &inct.material, depth)
    }

    fn direct_illu(&self, p: Vec3f, vin: Vec3f, material: &Box<BxDF>, depth: u32) -> Color3f {
        color3(0.0, 0.0, 0.0)
    }

    fn indirect_illu(&self, p: Vec3f, vin: Vec3f, material: &Box<BxDF>, depth: u32) -> Color3f {
        color3(0.0, 0.0, 0.0)
    }

    pub fn new(
        entities: Vec<Box<Entity>>,
        lights: Vec<Box<Light>>,
        background: Color3f,
        max_depth: u32,
        spp: u32,
    ) -> PathTracer {
        PathTracer {
            entities,
            lights,
            background,
            max_depth,
            spp,
        }
    }
}
