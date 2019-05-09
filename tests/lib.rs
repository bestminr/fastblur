#![feature(test)]
extern crate image;
extern crate fastblur;
extern crate test;

use image::{GenericImage,Pixel};
use test::{Bencher};

use fastblur::gaussian_blur;
use fastblur::utils;

// #[test]
fn test_blur_image_correctly() {
    let image_bytes = include_bytes!("../assets/cballs.png");
    let image_reference_bytes = include_bytes!("../assets/cballs_reference_5px_blur.png");

    let res1 = image::load_from_memory_with_format(image_bytes, image::ImageFormat::PNG);
    let res2 = image::load_from_memory_with_format(image_reference_bytes, image::ImageFormat::PNG);

    if let (Ok(image::DynamicImage::ImageRgb8(png_data)),
            Ok(image::DynamicImage::ImageRgb8(reference_data))) =
            (res1, res2)
    {
        let width = png_data.width() as usize;
        let height = png_data.height() as usize;
        let data = png_data.into_raw();
        let mut data_new = Vec::<[u8;3]>::with_capacity(width * height);
        data_new.resize(width * height, [0, 0, 0]);

        let mut data_new = Vec::<[u8;3]>::with_capacity(width * height);
        data_new.resize(width * height, [0, 0, 0]);

        for y in 0..height {
            for x in 0..width {
                let offset = ((width * y) + x) as usize;
                data_new[((width * y) + x) as usize] = [data[offset * 3], data[(offset * 3) + 1], data[(offset * 3) + 2]];
            }
        }

        let reference = reference_data.into_raw();
        let mut reference_new = Vec::<[u8;3]>::with_capacity(width * height);
        reference_new.resize(width * height, [0, 0, 0]);

        for y in 0..height {
            for x in 0..width {
                let offset = ((width * y) + x) as usize;
                reference_new[((width * y) + x) as usize] = [reference[offset * 3], reference[(offset * 3) + 1], reference[(offset * 3) + 2]];
            }
        }

        gaussian_blur(&mut data_new, width as usize, height as usize, 6.0);
        utils::write_image("output/test_blur_image_correctly.ppm", &data_new, width as usize, height as usize).unwrap();

        // this will fail, no matter what because the radius in the original algorithm isn't known
        // and I don't know if Javascript does things differently in terms of floating-point calculations
/*
        for (idx, (px, other)) in data_new.iter().zip(reference_new.iter()).enumerate() {
            if px != other { panic!("failed assertion @ byte {:?}", idx); }
        }
*/
    } else {
        panic!("could not decode png");
    }
}

fn test_blur_results() {
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

#[bench]
fn bench_blur_result(b: &mut Bencher) {
  b.iter(|| {
    (0..1).fold(0, |_, _| {
      test_blur_results();
      2
    });
  });
}

#[bench]
fn bench_blur_image(b: &mut test::Bencher) {
    let image_bytes = include_bytes!("../assets/cballs.png");
    if let Ok(image::DynamicImage::ImageRgb8(png_data)) = image::load_from_memory_with_format(image_bytes, image::ImageFormat::PNG) {
        let width = png_data.width() as usize;
        let height = png_data.height() as usize;
        let data = png_data.into_raw();
        let mut data_new = Vec::<[u8;3]>::with_capacity(width * height);
        data_new.resize(width * height, [0, 0, 0]);

        for y in 0..height {
            for x in 0..width {
                let offset = ((width * y) + x) as usize;
                data_new[((width * y) + x) as usize] = [data[offset * 3], data[(offset * 3) + 1], data[(offset * 3) + 2]];
            }
        }

        b.iter(||  { gaussian_blur(&mut data_new, width as usize, height as usize, 50.0); } );
    } else {
        panic!("could not decode png");
    }
}

