# fastblur

Fast (linear time) implementation of the Gaussian Blur algorithm in Rust.
Original taken from http://blog.ivank.net/fastest-gaussian-blur.html

## Usage

The image is assumed to be an RGB image with three channels.
This should change in the future, so you can blur as many channels as you want. Still very WIP.

### Rust Crate

```rust
#[dependencies]
fastblur = { git = "https://github.com/bestminr/fastblur" }
```

```rust
use fastblur::fast_blur;

// data is a Vec<[u8;3]> - 3 items for R, G and B.
// This format will probably change.
fast_blur(&mut data, width, height, 10.0);
```

### WebAssembly module

Thanks to [wasm-pack](https://github.com/rustwasm/wasm-pack), we can publish a WebAssembly module to work inside browsers.

Example usage:

```typescript
export function applyFastBlur(imageData: ImageData, blurRadius: number): Bluebird<ImageData> {
  return new Promise((resolve, reject) => {
    import('@bestminr/fastblur')
      .then((m) => {
        const { width, height } = imageData
        const inputDataArr = new Uint8Array(imageData.data)
        m.do_fast_blur(inputDataArr, width, height, blurRadius)
        const outputImageData = new ImageData(new Uint8ClampedArray(inputDataArr), width, height)
        return resolve(outputImageData)
      }).catch(reject)
  })
}
```
