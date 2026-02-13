//! Common math types and SpacetimeDB integrations for server-side Rust modules.
//!
//! # Features
//! - `f32` (default): use `f32` as `Scalar`.
//! - `f64`: use `f64` as `Scalar`.
//! - `serde`: enable `Serialize`/`Deserialize` derives.
//! - `glam`: enable `From` conversions with `glam` types.
//! - `nalgebra`: enable `From` conversions with `nalgebra` types.
//!
//! # Examples
//! ```ignore
//! use spacetimedb_math::{Vec3, Quat};
//!
//! #[spacetimedb::table(name = player_position)]
//! pub struct PlayerPosition {
//!     #[primary_key]
//!     #[auto_inc]
//!     pub id: u32,
//!
//!     pub translation: Vec3,
//!     pub rotation: Quat,
//! }
//! ```
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
