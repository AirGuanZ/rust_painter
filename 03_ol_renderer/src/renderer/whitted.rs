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
}

impl Renderer for WhittedRenderer {
    fn Render(&self, r: Ray) -> Color3f {
        color3(0.0, 0.0, 0.0)
    }
}
