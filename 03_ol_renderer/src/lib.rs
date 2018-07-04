//! A simple framework for ray-tracing based rendering

pub mod buf;
pub mod material;
pub mod math;

pub use buf::*;
pub use material::{BxDF, BxDFType};
pub use math::*;
