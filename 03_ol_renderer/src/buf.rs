//! Buffer objects

use std::ops::Index;
use std::ops::IndexMut;

/// 2-demensional array object
///
/// ```
/// use renderer::buf::Buf2D;
/// 
/// let mut buf = Buf2D::from_fn(128, 128, |x, y| {
///     if (x % 2 == 0) ^ (y % 2 == 0) {
///         255
///     } else {
///         0
///     }
/// });
/// 
/// assert_eq!(*buf.at(0, 0), 0);
/// assert_eq!(buf[(0, 1)], 255);
///
/// buf.clear_with_fn(|x, y| x * y)
///     .set(0, 0, &128)
///     .set(1, 2, &256);
///
/// assert_eq!(buf[(1, 2)], 256);
/// assert_eq!(buf[(127, 127)], 127 * 127);
/// ```
///
#[derive(Clone)]
pub struct Buf2D<T: Clone> {
    width: u32,
    height: u32,
    data: Vec<T>,
}

impl<T: Clone> Index<(u32, u32)> for Buf2D<T> {
    type Output = T;
    fn index(&self, idx: (u32, u32)) -> &T {
        self.at(idx.0, idx.1)
    }
}

impl<T: Clone> IndexMut<(u32, u32)> for Buf2D<T> {
    fn index_mut(&mut self, idx: (u32, u32)) -> &mut T {
        self.at_mut(idx.0, idx.1)
    }
}

impl<T: Clone> Buf2D<T> {
    fn xy2idx(&self, x: u32, y: u32) -> usize {
        assert!(x < self.width && y < self.height);
        (y * self.width + x) as usize
    }

    /// Create a new 2d buffer object with specified width and height.
    /// All elements will be initialized to `v.clone()`
    pub fn new(width: u32, height: u32, v: &T) -> Buf2D<T> {
        assert!(width > 0 && height > 0);
        let mut data = Vec::new();
        data.resize((width * height) as usize, v.clone());
        Buf2D {
            width,
            height,
            data,
        }
    }

    /// Create a new 2d buffer object with specified width and height.
    /// Element at `(x, y)` will be initialized to `f(x, y)`
    pub fn from_fn<F>(width: u32, height: u32, f: F) -> Buf2D<T>
    where
        F: Fn(u32, u32) -> T,
    {
        assert!(width > 0 && height > 0);
        let mut data = Vec::with_capacity((width * height) as usize);
        for y in 0..height {
            for x in 0..width {
                data.push(f(x, y));
            }
        }
        Buf2D {
            width,
            height,
            data,
        }
    }

    /// Buffer width
    pub fn get_width(&self) -> u32 {
        self.width
    }

    /// Buffer height
    pub fn get_height(&self) -> u32 {
        self.height
    }

    /// Reference to element at `(x, y)`
    pub fn at(&self, x: u32, y: u32) -> &T {
        &self.data[self.xy2idx(x, y)]
    }

    /// Mutable reference to element at `(x, y)`
    pub fn at_mut(&mut self, x: u32, y: u32) -> &mut T {
        let idx = self.xy2idx(x, y);
        &mut self.data[idx]
    }

    /// Set element at `(x, y)`.
    #[inline]
    pub fn set(&mut self, x: u32, y: u32, v: &T) -> &mut Self {
        let idx = self.xy2idx(x, y);
        self.data[idx] = v.clone();
        self
    }

    /// Reinitialize all elements to `v.clone()`
    pub fn clear(&mut self, v: &T) -> &mut Self {
        for i in 0..self.width * self.height {
            self.data[i as usize] = v.clone();
        }
        self
    }

    /// For all `(x, y)`, reinitialize element at `(x, y)` to `f(x, y)`
    pub fn clear_with_fn<F>(&mut self, f: F) -> &mut Self
    where
        F: Fn(u32, u32) -> T,
    {
        let mut i = 0_usize;
        for y in 0..self.height {
            for x in 0..self.width {
                self.data[i] = f(x, y);
                i += 1;
            }
        }
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gen_buf2d() {
        let mut buf = Buf2D::from_fn(640, 480, |x, y| {
            if (x % 2 == 0) ^ (y % 2 == 0) {
                255_u8
            } else {
                0_u8
            }
        });
        assert_eq!(buf[(0, 0)], 0_u8);

        buf[(0, 1)] = 122_u8;
        assert_eq!(buf[(0, 1)], 122_u8);

        buf.clear(&50_u8);
        assert_eq!(buf[(20, 20)], 50_u8);

        let f = |x: u32, y: u32| (x + y) as u8;
        buf.clear_with_fn(f);
        assert_eq!(buf[(23, 46)], f(23, 46));
    }
}
