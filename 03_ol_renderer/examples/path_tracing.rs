extern crate image;
extern crate rayon;
extern crate renderer;

use renderer::*;

const IMG_W: u32 = 640;
const IMG_H: u32 = 480;
const CAM_W: Real = 0.4;
const CAM_H: Real = CAM_W * (IMG_H as Real) / (IMG_W as Real);
const ITER_CNT: u32 = 512;

fn main() {
    use rayon::prelude::*;

    let camera = PerspectiveCamera::new(
        vec3(5.0, 3.0, 0.0),
        vec3(0.0, 0.0, 0.0),
        vec3(0.0, 1.0, 0.0),
        CAM_W,
        CAM_H,
        1.0,
    );

    let entities: Vec<Box<Entity>> = vec![
        Box::new(sphere::Sphere::new(
            vec3(0.0, 0.0, 0.0),
            0.4,
            Box::new(|_, loc_x, loc_y, _, _| {
                Box::new(Phong::new(BLACK, color3(0.4, 0.7, 0.8), loc_x, loc_y, 1.0))
            }),
        )),
        Box::new(sphere::Sphere::new(
            vec3(0.0, -5.0, 0.0),
            4.7,
            Box::new(|_, loc_x, loc_y, _, _| {
                Box::new(Phong::new(BLACK, color3(1.0, 0.6, 0.4), loc_x, loc_y, 1.0))
            }),
        )),
        Box::new(sphere::Sphere::new(
            vec3(0.32, -0.27, 0.13),
            0.1,
            Box::new(|_, _, loc_y, _, _| Box::new(DiffuseLight::new(loc_y, color3(0.0, 1.0, 1.8)))),
        )),
    ];

    let lights: Vec<Box<Light>> = vec![
        Box::new(PointLight::new(vec3(4.0, 10.0, 4.0), color3(0.0, 1.0, 1.0))),
        Box::new(PointLight::new(vec3(1.0, 1.0, -1.0), color3(0.8, 0.0, 0.0))),
    ];

    let renderer = PathTracer::new(entities, lights, BLACK, 3, 1);

    let img = image::ImageBuffer::from_fn(IMG_W, IMG_H, |x, y| {
        if x == IMG_W - 1 {
            println!("Finished: {}%", y as Real / (IMG_H - 1) as Real * 100.0);
        }

        let x = 2.0 * x as Real / (IMG_W - 1) as Real - 1.0;
        let y = -2.0 * y as Real / (IMG_H - 1) as Real + 1.0;
        let ray = camera.scr_to_ray(vec2(x, y));

        let c: Color3f = (0..ITER_CNT)
            .into_par_iter()
            .map(|_| renderer.render(ray.clone()))
            .sum();
        let c = (c / ITER_CNT as Real).clamp(0.0, 1.0);

        image::Rgb {
            data: [
                (c.r() * 255.0) as u8,
                (c.g() * 255.0) as u8,
                (c.b() * 255.0) as u8,
            ],
        }
    });
    img.save("./target/path_tracing.png").unwrap();
}
