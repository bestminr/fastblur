mod blur;

pub mod utils;

pub use blur::fast_blur;

#[cfg(feature = "webassembly")]
pub mod wasm;