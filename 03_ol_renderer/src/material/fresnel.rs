//! Fresnel equations

use math::*;

/// Ratio of reflected energy on (conductor) surface
pub fn conductor_ref_energy(eta: Real, k: Real, theta_in: Radf) -> Real {
    let alpha = eta * eta + k * k;
    let cos_tin = theta_in.cos();
    let cos_tin_s = cos_tin * cos_tin;
    let beta = 2.0 * eta * cos_tin;
    let r_para = (alpha * cos_tin_s - beta + 1.0) / (alpha * cos_tin_s + beta + 1.0);
    let r_prep = (alpha - beta + cos_tin_s) / (alpha + beta + cos_tin_s);
    0.5 * (r_para + r_prep)
}

/// Ratio of reflected energy on (unconductor) surface
pub fn unconductor_ref_energy(etai: Real, etat: Real, thetai: Radf, thetat: Radf) -> Real {
    let cosi = thetai.cos();
    let cost = thetat.cos();
    let r_para = (etat * cosi - etai * cost) / (etat * cosi + etai * cost);
    let r_perp = (etai * cosi - etai * cost) / (etai * cosi + etat * cost);
    0.5 * (r_para * r_para + r_perp * r_perp)
}
