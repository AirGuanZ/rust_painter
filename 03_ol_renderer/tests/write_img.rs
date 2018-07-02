extern crate image;
extern crate renderer;

#[test]
fn test_write_img() {
    let buf = renderer::buf::Buf2D::from_fn(64, 64, |x, y| {
        if (x % 2 == 0) ^ (y % 2 == 0) {
            255_u8
        } else {
            0_u8
        }
    });

    let img = image::ImageBuffer::from_fn(buf.get_width(), buf.get_height(), |x, y| {
        image::Luma([*buf.at(x, y)])
    });

    img.save("./target/write_img.png").unwrap();
}
