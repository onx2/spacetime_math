use crate::Scalar;
use spacetimedb::SpacetimeType;

/// A 3-dimensional vector in a right-handed, Y-up coordinate system.
///
/// +X is "right", -X is "left"
/// +Y is "up", -Y is "down"
/// +Z is "backward", -Z is "forward"
///
/// ```text
///      Y (Up)
///      |
///      |   -Z (Forward / Into Screen)
///      |  /
///      | /
///      o --------- X (Right)
///     /
///    /
///   Z (Backward / Out of Screen)
/// ```
#[derive(SpacetimeType, Debug, Default, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Vec3 {
    /// +X is "right", -X is "left"
    pub x: Scalar,
    /// +Y is "up", -Y is "down"
    pub y: Scalar,
    /// +Z is "backward", -Z is "forward"
    pub z: Scalar,
}

impl Vec3 {
    // Basic Constants
    pub const ZERO: Vec3 = Vec3::new(0.0, 0.0, 0.0);
    pub const ONE: Vec3 = Vec3::new(1.0, 1.0, 1.0);

    // Y-axis constants
    pub const UP: Vec3 = Vec3::new(0.0, 1.0, 0.0);
    pub const DOWN: Vec3 = Vec3::new(0.0, -1.0, 0.0);

    // X-axis constants
    pub const RIGHT: Vec3 = Vec3::new(1.0, 0.0, 0.0);
    pub const LEFT: Vec3 = Vec3::new(-1.0, 0.0, 0.0);

    // Z-axis constants
    pub const BACKWARD: Vec3 = Vec3::new(0.0, 0.0, 1.0);
    pub const FORWARD: Vec3 = Vec3::new(0.0, 0.0, -1.0);

    #[inline(always)]
    pub const fn new(x: Scalar, y: Scalar, z: Scalar) -> Self {
        Vec3 { x, y, z }
    }
}

#[cfg(feature = "nalgebra")]
mod nalgebra_impls {
    use super::*;

    impl From<nalgebra::Vector3<Scalar>> for Vec3 {
        #[inline(always)]
        fn from(v: nalgebra::Vector3<Scalar>) -> Self {
            Vec3::new(v.x, v.y, v.z)
        }
    }
    impl From<Vec3> for nalgebra::Vector3<Scalar> {
        #[inline(always)]
        fn from(v: Vec3) -> Self {
            nalgebra::Vector3::new(v.x, v.y, v.z)
        }
    }
}

#[cfg(feature = "glam")]
mod glam_impls {
    use super::*;

    #[cfg(feature = "f32")]
    impl From<glam::Vec3> for Vec3 {
        fn from(v: glam::Vec3) -> Self {
            Self {
                x: v.x,
                y: v.y,
                z: v.z,
            }
        }
    }

    #[cfg(feature = "f32")]
    impl From<Vec3> for glam::Vec3 {
        fn from(v: Vec3) -> Self {
            glam::Vec3::new(v.x, v.y, v.z)
        }
    }

    #[cfg(feature = "f64")]
    impl From<glam::DVec3> for Vec3 {
        fn from(v: glam::DVec3) -> Self {
            Self {
                x: v.x,
                y: v.y,
                z: v.z,
            }
        }
    }

    #[cfg(feature = "f64")]
    impl From<Vec3> for glam::DVec3 {
        fn from(v: Vec3) -> Self {
            glam::DVec3::new(v.x, v.y, v.z)
        }
    }
}
