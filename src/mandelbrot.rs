use std::io::Cursor;

use image::{ImageBuffer, ImageError, Rgba};

fn mandelbrot(c: num_complex::Complex<f64>) -> u8 {
    let mut z = num_complex::Complex::new(0.0, 0.0);
    for i in 0..255 {
        if z.norm_sqr() > 4.0 {
            return i;
        }
        z = z * z + c;
    }
    255
}

pub fn get_image() -> Result<Vec<u8>, ImageError> {
    let width = 640;
    let height = 480;
    let real_min = -2.0;
    let real_max = 1.0;
    let imag_min = -1.0;
    let imag_max = 1.0;
    let mut imgbuf = ImageBuffer::new(width, height);
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let real = real_min + (x as f64 / (width - 1) as f64) * (real_max - real_min);
        let imag = imag_min + (y as f64 / (height - 1) as f64) * (imag_max - imag_min);
        let c = num_complex::Complex::new(real, imag);
        let val = 255 - mandelbrot(c);
        *pixel = Rgba([val, val, val, 255]);
    }

    let mut buffer = Cursor::new(Vec::new());
    image::write_buffer_with_format(
        &mut buffer,
        &imgbuf,
        width,
        height,
        image::ColorType::Rgba8,
        image::ImageFormat::Png,
    )
    .map(|_| buffer.into_inner())
}
