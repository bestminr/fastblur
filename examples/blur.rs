extern crate image;
extern crate fastblur;

use image::{GenericImage,Pixel};
use fastblur::gaussian_blur;
use fastblur::utils;

fn main() {
    use std::path::Path;
    let img1 = image::open(Path::new("data/img1.jpg")).unwrap();
    let (width, height) = img1.dimensions();

    let mut image_data = vec![];
    for (_, _, p1) in img1.pixels()  {
        let d = p1.channels4();
        image_data.push([d.0, d.1, d.2]);
    }

    gaussian_blur(&mut image_data, width as usize, height as usize, 2.0);
    utils::write_image("output/img1.ppm", &image_data, width as usize, height as usize).unwrap();
}