//! Cook-Torrance reflection model

use math::*;

/// BRDF given by Cook-Torrance reflection model
/// 
/// # Params
/// 
/// - fresnel: Fresnel reflection coefficient
/// - d: Mircofacet distribution function
/// - n: Normalized normal direction
/// - phi, theta: In/Out direction
/// 
pub fn cook_torrance<MD>(fresnel: Real, d: MD, n: Vec3f, phi: Vec3f, theta: Vec3f) -> Real
where
    MD: Fn(Vec3f, Vec3f) -> Real,
{
    let h = (theta + phi).normalize();
    let dh = d(n, h);
    let g = (1.0 as Real)
        .min(2.0 * dot(n, h) * dot(n, theta) / dot(theta, h))
        .min(2.0 * dot(n, h) * dot(n, phi) / dot(theta, h));
    fresnel / REAL_PI * dh * g / dot(n, phi) / dot(n, theta)
}
