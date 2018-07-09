extern crate image;
extern crate renderer;

use renderer::math::*;

const IMG_W: u32 = 640;
const IMG_H: u32 = 480;
const CAM_W: Real = 1.0;
const CAM_H: Real = CAM_W * (IMG_H as Real) / (IMG_W as Real);

#[test]
fn test_whitted_renderer() {
    use renderer::camera::*;
    use renderer::entity::*;
    use renderer::material::prelude::*;
    use renderer::renderer::*;

    let camera = PerspectiveCamera::new(
        vec3(-3.0, 4.0, -8.0),
        vec3(0.0, 0.0, 0.0),
        vec3(0.0, 1.0, 0.0),
        CAM_W,
        CAM_H,
        1.0,
    );

    let entities: Vec<std::boxed::Box<renderer::entity::Entity + 'static>> =
        vec![Box::new(sphere::Sphere::new(
            vec3(0.0, 0.0, 0.0),
            0.4,
            Box::new(|_, loc_x, loc_y, _, _| {
                Box::new(Phong::new(
                    color3(0.1, 0.1, 0.1),
                    color3(0.3, 0.7, 0.4),
                    loc_x,
                    loc_y,
                    2.2,
                ))
            }),
        ))];

    let lights = vec![];

    let renderer = WhittedRenderer::new(entities, lights, color3(0.0, 0.0, 0.0), 5);

    let img = image::ImageBuffer::from_fn(IMG_W, IMG_H, |x, y| {
        let x = 2.0 * x as Real / (IMG_W - 1) as Real - 1.0;
        let y = 2.0 * y as Real / (IMG_H - 1) as Real - 1.0;
        let ray = camera.scr_to_ray(vec2(x, y));
        let c = renderer.render(ray).clamp(0.0, 1.0);
        image::Rgb {
            data: [
                (c.r() * 255.0) as u8,
                (c.g() * 255.0) as u8,
                (c.b() * 255.0) as u8,
            ],
        }
    });
    img.save("./target/whitted.png").unwrap();
}
