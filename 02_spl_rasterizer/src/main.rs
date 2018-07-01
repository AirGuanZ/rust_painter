/*
    简单的软件光栅渲染器，大概流程是：
        vertex shader -> rasterizer -> pixel shader -> depth test
*/

extern crate cgmath;

mod math;
use math::*;

mod scrbuf;
use scrbuf::ScrBuf;

struct Vertex {
    pub pos: Vec3f,
    pub nor: Vec3f,
}

struct Vtx2Pxl {
    pub spos: Vec2f,
    pub wpos: Vec3f,
    pub nor: Vec3f,
    pub depth: Real,
}

struct VertexShader {
    world: Mat4f,
    trans: Mat4f,

    width: u32,
    height: u32,
}

impl VertexShader {
    pub fn new(world: &Mat4f, trans: &Mat4f, scr: &ScrBuf) -> VertexShader {
        VertexShader {
            world: *world,
            trans: *trans,
            width: scr.get_width(),
            height: scr.get_height(),
        }
    }

    pub fn shade(&self, v: &Vertex) -> Vtx2Pxl {
        use cgmath::InnerSpace;

        let epos = vec4(v.pos.x, v.pos.y, v.pos.z, 1.0);
        let hpos = self.trans * epos;

        Vtx2Pxl {
            spos: 0.5 * vec2(
                (hpos.x / hpos.w + 1.0) * (self.width as Real - 1.0),
                (hpos.y / hpos.w + 1.0) * (self.height as Real - 1.0),
            ),
            wpos: (self.world * epos).xyz(),
            nor: (self.trans * vec4(v.nor.x, v.nor.y, v.nor.z, 0.0))
                .xyz()
                .normalize(),
            depth: hpos.z + 1.0,
        }
    }
}

struct PointLight {
    pub pos: Vec3f,
    pub color: Vec3f,
}

struct PixelShader {
    lights: Vec<PointLight>,
    dis_fac: Vec3f,
}

impl PixelShader {
    fn new(lights: Vec<PointLight>, dis_fac: &Vec3f) -> PixelShader {
        PixelShader {
            lights,
            dis_fac: *dis_fac,
        }
    }

    fn shade(&self, pos: &Vec3f, nor: &Vec3f) -> Vec3f {
        use cgmath::InnerSpace;

        let mut light_color = vec3(0.0, 0.0, 0.0);
        for light in &self.lights {
            let dis_v = light.pos - pos;
            let dis = dis_v.magnitude();
            let lambert = dot(nor.normalize(), dis_v.normalize());
            if lambert > 0.0 {
                light_color += lambert / dot(self.dis_fac, vec3(dis, dis * dis, dis * dis * dis))
                    * light.color;
            }
        }

        vec3(
            light_color.x.max(0.0).min(1.0),
            light_color.y.max(0.0).min(1.0),
            light_color.z.max(0.0).min(1.0),
        )
    }
}

fn raster_pxl(
    pxl_x: u32,
    pxl_y: u32,
    scr: &mut ScrBuf,
    pxl: &PixelShader,
    a: &Vtx2Pxl,
    b: &Vtx2Pxl,
    c: &Vtx2Pxl,
) {
    use cgmath::SquareMatrix;

    let pnt = vec2(pxl_x as Real, (scr.get_height() - 1 - pxl_y) as Real);
    let wgh = match (Mat2f::from_cols(a.spos - c.spos, b.spos - c.spos)).invert() {
        Some(i) => i * (-c.spos + pnt),
        None => {
            panic!("Fatal error in vertex transformation");
        }
    };
    let wgh = vec3(wgh.x, wgh.y, 1.0 - wgh.x - wgh.y);
    if !(wgh.x >= 0.0 && wgh.y >= 0.0 && wgh.z >= 0.0) {
        return;
    }

    let dp = dot(vec3(a.depth, b.depth, c.depth), wgh);
    if dp <= 0.0 || dp >= scr.get_depth(pxl_x, pxl_y) {
        return;
    }

    scr.set_color(
        pxl_x,
        pxl_y,
        &pxl.shade(
            &(Mat3f::from_cols(a.wpos, b.wpos, c.wpos) * wgh),
            &(Mat3f::from_cols(a.nor, b.nor, c.nor) * wgh),
        ),
    );
    scr.set_depth(pxl_x, pxl_y, dp);
}

fn tri_raster(scr: &mut ScrBuf, pxl: &PixelShader, a: &Vtx2Pxl, b: &Vtx2Pxl, c: &Vtx2Pxl) {
    for pxl_x in 0..scr.get_width() {
        for pxl_y in 0..scr.get_height() {
            raster_pxl(pxl_x, pxl_y, scr, pxl, a, b, c);
        }
    }
}

fn main() {
    use cgmath::SquareMatrix;

    const SCR_W: u32 = 640;
    const SCR_H: u32 = 480;

    let mut sb = ScrBuf::new(SCR_W, SCR_H);

    let world = Mat4f::identity();
    let view = Mat4f::look_at(
        Pnt3f::new(0.0, 0.0, -5.0),
        Pnt3f::new(0.0, 0.0, 0.0),
        vec3(0.0, 1.0, 0.0),
    );
    let proj = cgmath::perspective(
        cgmath::Rad::<Real>(3.1415 / 5.0),
        SCR_W as Real / SCR_H as Real,
        0.1,
        100.0,
    );

    let lights = vec![
        PointLight {
            pos: vec3(0.5, 0.0, -0.7),
            color: vec3(0.0, 0.6, 0.6),
        },
        PointLight {
            pos: vec3(-0.3, 0.0, -1.2),
            color: vec3(0.8, 0.0, 0.0),
        },
    ];

    let vtx_shader = VertexShader::new(&world, &(proj * view * world), &sb);
    let pxl_shader = PixelShader::new(lights, &vec3(0.8, 0.3, 0.1));

    sb.clear(&vec3(0.0, 0.0, 0.0), REAL_MAX);

    let tri0 = [
        Vertex {
            pos: vec3(-1.0, -1.0, 0.0),
            nor: vec3(0.0, 0.0, -1.0),
        },
        Vertex {
            pos: vec3(0.0, 1.0, 0.0),
            nor: vec3(0.0, 0.0, -1.0),
        },
        Vertex {
            pos: vec3(1.0, -1.0, 0.0),
            nor: vec3(0.0, 0.0, -1.0),
        },
    ];

    let tri1 = [
        Vertex {
            pos: vec3(-0.5, 0.2, -0.2),
            nor: vec3(0.0, 0.0, -1.0),
        },
        Vertex {
            pos: vec3(0.2, 0.75, -0.2),
            nor: vec3(0.0, 0.0, -1.0),
        },
        Vertex {
            pos: vec3(2.0, 0.1, -0.2),
            nor: vec3(0.0, 0.0, -1.0),
        },
    ];

    tri_raster(
        &mut sb,
        &pxl_shader,
        &vtx_shader.shade(&tri0[0]),
        &vtx_shader.shade(&tri0[1]),
        &vtx_shader.shade(&tri0[2]),
    );

    tri_raster(
        &mut sb,
        &pxl_shader,
        &vtx_shader.shade(&tri1[0]),
        &vtx_shader.shade(&tri1[1]),
        &vtx_shader.shade(&tri1[2]),
    );

    sb.save_to_file("output.png");
}
