//! # fastblur
//!
//! Fast (linear time) implementation of the Gaussian Blur algorithm in Rust.
//! Original taken from http://blog.ivank.net/fastest-gaussian-blur.html
//!
//! ## Usage
//!
//! The image is assumed to be an RGB image with three channels.
//! This should change in the future, so you can blur as many channels as you want. Still very WIP.
//!
//! ```rust,ignore
//! #[dependencies]
//! fastblur = { git = "https://github.com/fschutt/fastblur" }
//! ```
//!
//! ```rust,ignore
//! use fastblur::fast_blur;
//!
//! // data is a Vec<[u8;3]> - 3 items for R, G and B.
//! // This format will probably change.
//! fast_blur(&mut data, width, height, 10.0);
//! ```

mod blur;

pub mod utils;

pub use blur::fast_blur;

extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

// #[wasm_bindgen]
// extern "C" {
//     // Use `js_namespace` here to bind `console.log(..)` instead of just
//     // `log(..)`
//     #[wasm_bindgen(js_namespace = console)]
//     fn log(s: &str);

//     // The `console.log` is quite polymorphic, so we can bind it with multiple
//     // signatures. Note that we need to use `js_name` to ensure we always call
//     // `log` in JS.
//     #[wasm_bindgen(js_namespace = console, js_name = log)]
//     fn log_u32(a: u32);

//     // Multiple arguments too!
//     #[wasm_bindgen(js_namespace = console, js_name = log)]
//     fn log_many(a: &str, b: &str);
// }

// #[wasm_bindgen]
// macro_rules! console_log {
//     // Note that this is using the `log` function imported above during
//     // `bare_bones`
//     ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
// }

/// will mutate in_image_data
#[wasm_bindgen]
pub fn do_fast_blur(in_image_data: &mut [u8], width: usize, height: usize, blur_radius: f32) {
    // let arr_len = (width * height * 4) as usize;
    let mut blur_input_data: Vec<[u8; 3]> = vec![];
    let pixel_count = width * height;

    for pi in 0..pixel_count {
        let start_i = pi * 4;
        blur_input_data.push([
            in_image_data[start_i],
            in_image_data[start_i + 1],
            in_image_data[start_i + 2],
        ]);
    }
    blur::fast_blur(&mut blur_input_data, width, height, blur_radius);

    // copy result to output image
    // let mut output_image_data: Vec<u8> = Vec::with_capacity(arr_len);
    for pi in 0..pixel_count {
        let result_seg = &blur_input_data[pi];
        let start_i = pi * 4;
        in_image_data[start_i] = result_seg[0];
        in_image_data[start_i + 1] = result_seg[1];
        in_image_data[start_i + 2] = result_seg[2];
    }
}
