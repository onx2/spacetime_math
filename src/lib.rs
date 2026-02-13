#[cfg(all(feature = "f32", feature = "f64"))]
compile_error!("Features 'f32' and 'f64' are mutually exclusive.");

pub mod quat;
pub mod scalar;
pub mod vec2;
pub mod vec3;

pub use quat::*;
pub use scalar::*;
pub use vec2::*;
pub use vec3::*;
