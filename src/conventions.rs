//! Coordinate system conventions and axis presets.

use crate::{Scalar, Vec3};

/// Orthonormal axes describing a coordinate system's orientation.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Axes {
    /// The up direction.
    pub up: Vec3,
    /// The forward direction.
    pub forward: Vec3,
    /// The right direction.
    pub right: Vec3,
}

impl Axes {
    /// Builds a right-handed orthonormal basis from `up` and `forward`.
    /// Returns `None` if the inputs are too small or nearly parallel.
    pub fn try_right_handed(up: Vec3, forward: Vec3, epsilon: Scalar) -> Option<Self> {
        let up = up.try_normalize(epsilon)?;
        let forward = forward.try_normalize(epsilon)?;
        let right = forward.cross(up).try_normalize(epsilon)?;
        let forward = up.cross(right);
        Some(Self { up, forward, right })
    }

    /// Builds a left-handed orthonormal basis from `up` and `forward`.
    /// Returns `None` if the inputs are too small or nearly parallel.
    pub fn try_left_handed(up: Vec3, forward: Vec3, epsilon: Scalar) -> Option<Self> {
        let up = up.try_normalize(epsilon)?;
        let forward = forward.try_normalize(epsilon)?;
        let right = up.cross(forward).try_normalize(epsilon)?;
        let forward = right.cross(up);
        Some(Self { up, forward, right })
    }
}

/// Default coordinate convention.
pub const DEFAULT: Axes = Y_UP_RIGHT_HANDED_FWD_NEG_Z;

/// Right-handed, Y-up convention with forward = -Z.
/// Common in Bevy and Godot.
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
pub const Y_UP_RIGHT_HANDED_FWD_NEG_Z: Axes = Axes {
    up: Vec3::new(0.0, 1.0, 0.0),
    forward: Vec3::new(0.0, 0.0, -1.0),
    right: Vec3::new(1.0, 0.0, 0.0),
};

/// Right-handed, Y-up convention with forward = +Z.
/// ```text
///      Y (Up)
///      |
///      |   +Z (Forward / Into Screen)
///      |  /
///      | /
///      o --------- X (Right)
///     /
///    /
///   -Z (Backward / Out of Screen)
/// ```
pub const Y_UP_RIGHT_HANDED_FWD_POS_Z: Axes = Axes {
    up: Vec3::new(0.0, 1.0, 0.0),
    forward: Vec3::new(0.0, 0.0, 1.0),
    right: Vec3::new(-1.0, 0.0, 0.0),
};

/// Left-handed, Y-up convention with forward = +Z.
/// Common in Unity.
/// ```text
///      Y (Up)
///      |
///      |   +Z (Forward / Into Screen)
///      |  /
///      | /
///      o --------- X (Right)
///     /
///    /
///   -Z (Backward / Out of Screen)
/// ```
pub const Y_UP_LEFT_HANDED_FWD_POS_Z: Axes = Axes {
    up: Vec3::new(0.0, 1.0, 0.0),
    forward: Vec3::new(0.0, 0.0, 1.0),
    right: Vec3::new(1.0, 0.0, 0.0),
};

/// Right-handed, Z-up convention with forward = +Y.
/// Common in Blender; also used by some editors (e.g., TrenchBroom), depending on the game.
/// ```text
///      Z (Up)
///      |
///      |   +Y (Forward / Into Screen)
///      |  /
///      | /
///      o --------- X (Right)
///     /
///    /
///   -Y (Backward / Out of Screen)
/// ```
pub const Z_UP_RIGHT_HANDED_FWD_POS_Y: Axes = Axes {
    up: Vec3::new(0.0, 0.0, 1.0),
    forward: Vec3::new(0.0, 1.0, 0.0),
    right: Vec3::new(1.0, 0.0, 0.0),
};

/// Left-handed, Z-up convention with forward = +X.
/// Common in Unreal.
/// ```text
///      Z (Up)
///      |
///      |   +X (Forward / Into Screen)
///      |  /
///      | /
///      o --------- Y (Right)
///     /
///    /
///   -X (Backward / Out of Screen)
/// ```
pub const Z_UP_LEFT_HANDED_FWD_POS_X: Axes = Axes {
    up: Vec3::new(0.0, 0.0, 1.0),
    forward: Vec3::new(1.0, 0.0, 0.0),
    right: Vec3::new(0.0, 1.0, 0.0),
};

#[cfg(test)]
mod tests {
    use super::*;

    const EPS: Scalar = 1.0e-6 as Scalar;

    fn assert_axes_orthonormal(axes: Axes) {
        assert_eq!(axes.up.dot(axes.forward), 0.0 as Scalar);
        assert_eq!(axes.up.dot(axes.right), 0.0 as Scalar);
        assert_eq!(axes.forward.dot(axes.right), 0.0 as Scalar);
        assert_eq!(axes.up.length_squared(), 1.0 as Scalar);
        assert_eq!(axes.forward.length_squared(), 1.0 as Scalar);
        assert_eq!(axes.right.length_squared(), 1.0 as Scalar);
    }

    #[test]
    fn presets_are_orthonormal() {
        assert_axes_orthonormal(Y_UP_RIGHT_HANDED_FWD_NEG_Z);
        assert_axes_orthonormal(Y_UP_RIGHT_HANDED_FWD_POS_Z);
        assert_axes_orthonormal(Y_UP_LEFT_HANDED_FWD_POS_Z);
        assert_axes_orthonormal(Z_UP_RIGHT_HANDED_FWD_POS_Y);
        assert_axes_orthonormal(Z_UP_LEFT_HANDED_FWD_POS_X);
    }

    #[test]
    fn try_right_handed_matches_preset() {
        let axes = Axes::try_right_handed(Vec3::new(0.0, 1.0, 0.0), Vec3::new(0.0, 0.0, -1.0), EPS)
            .expect("expected axes");
        assert_eq!(axes, Y_UP_RIGHT_HANDED_FWD_NEG_Z);
    }

    #[test]
    fn try_left_handed_matches_preset() {
        let axes = Axes::try_left_handed(Vec3::new(0.0, 1.0, 0.0), Vec3::new(0.0, 0.0, 1.0), EPS)
            .expect("expected axes");
        assert_eq!(axes, Y_UP_LEFT_HANDED_FWD_POS_Z);
    }

    #[test]
    fn try_right_handed_rejects_parallel_vectors() {
        let axes = Axes::try_right_handed(Vec3::new(0.0, 1.0, 0.0), Vec3::new(0.0, 1.0, 0.0), EPS);
        assert!(axes.is_none());
    }
}
