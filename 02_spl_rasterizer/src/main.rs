/*
    简单的软件光栅渲染器，大概流程是：
        vertex shader -> rasterizer -> pixel shader -> depth test
*/

extern crate cgmath;
extern crate image;

type Real = f32;
type Mat4f = cgmath::Matrix4<Real>;
type Vec2f = cgmath::Vector2<Real>;
type Vec3f = cgmath::Vector3<Real>;
type Vec4f = cgmath::Vector4<Real>;

use cgmath::{dot, vec2, vec3, vec4};

trait Clamp {
    type ClampValue;
    fn clamp(&self, min: Self::ClampValue, max: Self::ClampValue) -> Self;
}

impl Clamp for Real {
    type ClampValue = Real;
    fn clamp(&self, min: Real, max: Real) -> Self {
        self.max(min).min(max)
    }
}

impl Clamp for Vec3f {
    type ClampValue = Real;
    fn clamp(&self, min: Real, max: Real) -> Self {
        vec3(
            self.x.clamp(min, max),
            self.y.clamp(min, max),
            self.z.clamp(min, max),
        )
    }
}

struct ScrBuf {
    width: i32,
    height: i32,
    color_buf: Vec<Vec3f>,
    depth_buf: Vec<Real>,
}

impl ScrBuf {
    fn buf_index(&self, x: i32, y: i32) -> usize {
        assert!(0 <= x && x < self.width);
        assert!(0 <= y && y < self.height);
        (x * self.height + y) as usize
    }

    pub fn new(w: i32, h: i32) -> ScrBuf {
        assert!(w > 0 && h > 0);
        let mut rt = ScrBuf {
            width: w,
            height: h,
            color_buf: Vec::new(),
            depth_buf: Vec::new(),
        };
        let buf_size = (w * h) as usize;
        rt.color_buf.resize(buf_size, vec3(0.0, 0.0, 0.0));
        rt.depth_buf.resize(buf_size, 1.0);
        rt
    }

    pub fn set_color(&mut self, x: i32, y: i32, c: &Vec3f) {
        let index = self.buf_index(x, y);
        self.color_buf[index] = *c;
    }

    pub fn set_depth(&mut self, x: i32, y: i32, d: Real) {
        let index = self.buf_index(x, y);
        self.depth_buf[index] = d;
    }

    pub fn get_color(&self, x: i32, y: i32) -> Vec3f {
        let index = self.buf_index(x, y);
        self.color_buf[index]
    }

    pub fn get_depth(&self, x: i32, y: i32) -> Real {
        let index = self.buf_index(x, y);
        self.depth_buf[index]
    }

    pub fn get_width(&self) -> i32 {
        self.width
    }

    pub fn get_height(&self) -> i32 {
        self.height
    }

    pub fn clear(&mut self, color: &Vec3f, depth: Real) {
        for x in 0..self.width {
            for y in 0..self.height {
                self.set_color(x, y, color);
                self.set_depth(x, y, depth);
            }
        }
    }

    pub fn save_to_file(&self, filename: &str) {
        let img = image::ImageBuffer::from_fn(self.width as u32, self.height as u32, |x, y| {
            let c = self.get_color(x as i32, y as i32);
            image::Rgb([
                (c.x * 255.0) as u8,
                (c.y * 255.0) as u8,
                (c.z * 255.0) as u8,
            ])
        });
        img.save(std::path::Path::new(filename)).unwrap();
    }
}

struct Vertex {
    pub pos: Vec3f,
    pub nor: Vec3f,
    pub tex: Vec2f,
}

struct Vtx2Pxl {
    pub spos: Vec2f,
    pub wpos: Vec3f,
    pub nor: Vec3f,
    pub tex: Vec2f,
    pub depth: Real,
}

struct VertexShader<'a> {
    world: Mat4f,
    trans: Mat4f,

    scr: &'a ScrBuf,
}

impl<'a> VertexShader<'a> {
    pub fn new<'b>(world: &Mat4f, trans: &Mat4f, scr: &'b ScrBuf) -> VertexShader<'b> {
        VertexShader {
            world: *world,
            trans: *trans,
            scr,
        }
    }

    pub fn shade(&self, v: &Vertex) -> Vtx2Pxl {
        let epos = vec4(v.pos.x, v.pos.y, v.pos.z, 1.0);
        let hpos = self.trans * epos;
        Vtx2Pxl {
            spos: 0.5 * vec2(
                (hpos.x / hpos.w + 1.0) * (self.scr.get_width() as Real - 1.0),
                (hpos.y / hpos.w + 1.0) * (self.scr.get_height() as Real - 1.0),
            ),
            wpos: (self.world * epos).xyz(),
            nor: (self.trans * vec4(v.nor.x, v.nor.y, v.nor.z, 0.0)).xyz(),
            tex: v.tex,
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

    fn shade(&self, pos: &Vec3f, nor: &Vec3f, tex: &Vec3f) -> Vec3f {
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

        light_color.clamp(0.0, 1.0)
    }
}

fn main() {
    let mut sb = ScrBuf::new(640, 480);
    sb.set_color(0, 0, &vec3(0.0, 1.0, 1.0));
    sb.set_depth(0, 0, 0.5);
    sb.save_to_file("output.png");
}
