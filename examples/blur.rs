extern crate fastblur;
extern crate image;

use fastblur::gaussian_blur;
use image::{GenericImage, ImageBuffer, Pixel, Rgb};
use std::path::Path;

struct TestItem {
    input_name: String,
    radius: f32,
}

fn main() {
    let test_items: Vec<TestItem> = vec![
        // TestItem { input_name: String::from("test-images/blur1x100.jpg"), radius: 1.0, },
        //        TestItem { input_name: String::from ("test-images/blur10x100.jpg"), radius: 10.0 },
               TestItem { input_name: String::from("test-images/blur15x100.jpg"), radius: 15.0 },
        //        TestItem { input_name: String::from("test-images/blur25x100.jpg"), radius: 25.0 },
        //        TestItem { input_name: String::from("test-images/blur30x100.jpg"), radius: 30.0 },
    ];

    for item in test_items {
        run_test_item(item);
    }
}

fn run_test_item(item: TestItem) {
    let img1 = image::open(Path::new(&item.input_name)).unwrap();
    let (width, height) = img1.dimensions();

    let mut image_data: Vec<[u8; 3]> = vec![];
    for (_, _, p1) in img1.pixels() {
        let d = p1.channels4();
        image_data.push([d.0, d.1, d.2]);
    }
    //    println!("dims {}x{}, image_data length {}", width, height, image_data.len());

    // 结果近似的经验值, 根据以下文章观点，盒模糊模拟高斯模糊时，标准差 sigma 与 radius 相等
    // http://blog.ivank.net/fastest-gaussian-blur.html
    let radius = item.radius.sqrt();

    gaussian_blur(&mut image_data, width as usize, height as usize, radius);

    // use fastblur::utils;
    // utils::write_image("output/f_blur10x100.ppm", &image_data, width as usize, height as usize).unwrap();

    let mut imgbuf: ImageBuffer<Rgb<u8>, Vec<u8>> = image::ImageBuffer::new(width, height);

    let img_data_len = image_data.len();

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let start_i = y * width + x;
        if start_i >= img_data_len as u32 {
            continue;
        }
        let colors = image_data[start_i as usize];
        let p = image::Rgb(colors);
        *pixel = p
    }
    let output_name = [
        "test-images/f_blur".to_string(),
        format!("{}", item.radius),
        ".jpg".to_string(),
    ]
    .join("");
    imgbuf.save(output_name).unwrap();
}
