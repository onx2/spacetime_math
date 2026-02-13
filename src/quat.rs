use crate::Scalar;
use spacetimedb::SpacetimeType;

/// A quaternion representing 3D rotation (orientation) in a right-handed, Y-up coordinate system.
///
/// Positive rotation is counter-clockwise when looking down the axis toward the origin.
///
/// ```text
/// Identity (w=1, x=0, y=0, z=0):
///   Aligns local Forward to World -Z
///   Aligns local Up to World +Y
/// ```
///
#[derive(SpacetimeType, Debug, Clone, Copy, PartialEq)]
pub struct Quat {
    /// Vector part (imaginary i)
    pub x: Scalar,
    /// Vector part (imaginary j)
    pub y: Scalar,
    /// Vector part (imaginary k)
    pub z: Scalar,
    /// Scalar part (real) - Set to 1.0 for Identity
    pub w: Scalar,
}

impl Default for Quat {
    fn default() -> Self {
        Self::IDENTITY
    }
}

impl Quat {
    /// The "No Rotation" quaternion.
    /// Aligns the entity with the global axes (Forward = -Z, Up = +Y).
    pub const IDENTITY: Self = Self::new(0.0, 0.0, 0.0, 1.0);

    #[inline(always)]
    pub const fn new(x: Scalar, y: Scalar, z: Scalar, w: Scalar) -> Self {
        Quat { x, y, z, w }
    }
}

#[cfg(feature = "nalgebra")]
mod nalgebra_impls {
    use super::*;

    impl From<Quat> for nalgebra::UnitQuaternion<Scalar> {
        fn from(q: Quat) -> Self {
            // nalgebra: Quaternion::new(w, i, j, k)
            let raw = nalgebra::Quaternion::new(q.w, q.x, q.y, q.z);
            Self::from_quaternion(raw)
        }
    }

    impl From<nalgebra::UnitQuaternion<Scalar>> for Quat {
        fn from(uq: nalgebra::UnitQuaternion<Scalar>) -> Self {
            let q = uq.into_inner();
            Self {
                x: q.i,
                y: q.j,
                z: q.k,
                w: q.w,
            }
        }
    }
}

#[cfg(feature = "glam")]
mod glam_impls {
    use super::*;

    #[cfg(feature = "f32")]
    impl From<glam::Quat> for Quat {
        fn from(q: glam::Quat) -> Self {
            Self::new(q.x, q.y, q.z, q.w)
        }
    }

    #[cfg(feature = "f32")]
    impl From<Quat> for glam::Quat {
        fn from(q: Quat) -> Self {
            glam::Quat::from_xyzw(q.x, q.y, q.z, q.w)
        }
    }

    #[cfg(feature = "f64")]
    impl From<glam::DQuat> for Quat {
        fn from(q: glam::DQuat) -> Self {
            Self::new(q.x, q.y, q.z, q.w)
        }
    }

    #[cfg(feature = "f64")]
    impl From<Quat> for glam::DQuat {
        fn from(q: Quat) -> Self {
            glam::DQuat::from_xyzw(q.x, q.y, q.z, q.w)
        }
    }
}
