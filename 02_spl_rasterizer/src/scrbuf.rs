extern crate image;
extern crate std;

use math::*;

pub struct ScrBuf {
    width: u32,
    height: u32,
    color_buf: Vec<Vec3f>,
    depth_buf: Vec<Real>,
}

impl ScrBuf {
    fn buf_index(&self, x: u32, y: u32) -> usize {
        assert!(x < self.width);
        assert!(y < self.height);
        (x * self.height + y) as usize
    }

    pub fn new(w: u32, h: u32) -> ScrBuf {
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

    pub fn set_color(&mut self, x: u32, y: u32, c: &Vec3f) {
        let index = self.buf_index(x, y);
        self.color_buf[index] = *c;
    }

    pub fn set_depth(&mut self, x: u32, y: u32, d: Real) {
        let index = self.buf_index(x, y);
        self.depth_buf[index] = d;
    }

    pub fn get_color(&self, x: u32, y: u32) -> Vec3f {
        self.color_buf[self.buf_index(x, y)]
    }

    pub fn get_depth(&self, x: u32, y: u32) -> Real {
        self.depth_buf[self.buf_index(x, y)]
    }

    pub fn get_width(&self) -> u32 {
        self.width
    }

    pub fn get_height(&self) -> u32 {
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
        let img = image::ImageBuffer::from_fn(self.width, self.height, |x, y| {
            let c = self.get_color(x, y);
            image::Rgb([
                (c.x * 255.0) as u8,
                (c.y * 255.0) as u8,
                (c.z * 255.0) as u8,
            ])
        });

        img.save(std::path::Path::new(filename)).unwrap();
    }
}
