use crate::{Scalar, Vec2};
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
///
/// # Examples
/// ```
/// use spacetimedb_math::Vec3;
///
/// let v = Vec3::new(1.0, 2.0, 3.0);
/// assert_eq!(v.x, 1.0);
/// assert_eq!(v.y, 2.0);
/// assert_eq!(v.z, 3.0);
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

    /// Returns the XY components of this vector.
    ///
    /// This is the vertical plane in the crate's right-handed, Y-up coordinate system.
    #[inline]
    pub const fn xy(&self) -> Vec2 {
        Vec2::new(self.x, self.y)
    }

    /// Returns the XZ components of this vector.
    ///
    /// This is the ground plane in the crate's right-handed, Y-up coordinate system.
    #[inline]
    pub const fn xz(&self) -> Vec2 {
        Vec2::new(self.x, self.z)
    }

    /// Returns the dot product of this vector and `other`.
    #[inline]
    pub fn dot(&self, other: Vec3) -> Scalar {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    /// Returns the squared length (magnitude) of this vector.
    #[inline]
    pub fn length_squared(&self) -> Scalar {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    /// Returns the length (magnitude) of this vector.
    pub fn length(&self) -> Scalar {
        self.length_squared().sqrt()
    }

    /// Returns the squared distance between this vector and `other`.
    #[inline]
    pub fn distance_squared(&self, other: Vec3) -> Scalar {
        let dx = other.x - self.x;
        let dy = other.y - self.y;
        let dz = other.z - self.z;
        dx * dx + dy * dy + dz * dz
    }

    /// Returns the distance between this vector and `other`.
    pub fn distance(&self, other: Vec3) -> Scalar {
        self.distance_squared(other).sqrt()
    }

    /// Returns a normalized vector, or `fallback` if length is below `epsilon`.
    pub fn normalize_or(&self, epsilon: Scalar, fallback: Vec3) -> Vec3 {
        let len_sq = self.length_squared();
        let epsilon_sq = epsilon * epsilon;
        if len_sq <= epsilon_sq {
            fallback
        } else {
            let len = len_sq.sqrt();
            Vec3::new(self.x / len, self.y / len, self.z / len)
        }
    }

    /// Returns a normalized vector, or `Vec3::ZERO` if length is below `epsilon`.
    pub fn normalize_or_zero(&self, epsilon: Scalar) -> Vec3 {
        self.normalize_or(epsilon, Vec3::ZERO)
    }

    /// Attempts to normalize this vector, returning `None` if length is below `epsilon`.
    pub fn try_normalize(&self, epsilon: Scalar) -> Option<Vec3> {
        let len_sq = self.length_squared();
        let epsilon_sq = epsilon * epsilon;
        if len_sq <= epsilon_sq {
            None
        } else {
            let len = len_sq.sqrt();
            Some(Vec3::new(self.x / len, self.y / len, self.z / len))
        }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn constants_match_constructor() {
        assert_eq!(
            Vec3::ZERO,
            Vec3::new(0.0 as Scalar, 0.0 as Scalar, 0.0 as Scalar)
        );
        assert_eq!(
            Vec3::ONE,
            Vec3::new(1.0 as Scalar, 1.0 as Scalar, 1.0 as Scalar)
        );
        assert_eq!(
            Vec3::UP,
            Vec3::new(0.0 as Scalar, 1.0 as Scalar, 0.0 as Scalar)
        );
        assert_eq!(
            Vec3::DOWN,
            Vec3::new(0.0 as Scalar, -1.0 as Scalar, 0.0 as Scalar)
        );
        assert_eq!(
            Vec3::RIGHT,
            Vec3::new(1.0 as Scalar, 0.0 as Scalar, 0.0 as Scalar)
        );
        assert_eq!(
            Vec3::LEFT,
            Vec3::new(-1.0 as Scalar, 0.0 as Scalar, 0.0 as Scalar)
        );
        assert_eq!(
            Vec3::BACKWARD,
            Vec3::new(0.0 as Scalar, 0.0 as Scalar, 1.0 as Scalar)
        );
        assert_eq!(
            Vec3::FORWARD,
            Vec3::new(0.0 as Scalar, 0.0 as Scalar, -1.0 as Scalar)
        );
    }

    #[test]
    fn vec3_xy_returns_xy_plane() {
        let v = Vec3::new(1.0 as Scalar, 2.0 as Scalar, 3.0 as Scalar);
        assert_eq!(v.xy(), Vec2::new(1.0 as Scalar, 2.0 as Scalar));
    }

    #[test]
    fn vec3_xz_returns_ground_plane() {
        let v = Vec3::new(1.0 as Scalar, 2.0 as Scalar, 3.0 as Scalar);
        assert_eq!(v.xz(), Vec2::new(1.0 as Scalar, 3.0 as Scalar));
    }

    #[test]
    fn vec3_dot_is_sum_of_component_products() {
        let a = Vec3::new(1.0 as Scalar, 2.0 as Scalar, 3.0 as Scalar);
        let b = Vec3::new(4.0 as Scalar, 5.0 as Scalar, 6.0 as Scalar);
        assert_eq!(a.dot(b), 32.0 as Scalar);
    }

    #[test]
    fn vec3_length_squared_is_sum_of_squares() {
        let v = Vec3::new(2.0 as Scalar, 3.0 as Scalar, 6.0 as Scalar);
        assert_eq!(v.length_squared(), 49.0 as Scalar);
    }

    #[test]
    fn vec3_length_is_square_root_of_length_squared() {
        let v = Vec3::new(2.0 as Scalar, 3.0 as Scalar, 6.0 as Scalar);
        assert_eq!(v.length(), 7.0 as Scalar);
    }

    #[test]
    fn vec3_distance_squared_is_sum_of_delta_squares() {
        let a = Vec3::new(1.0 as Scalar, 2.0 as Scalar, 3.0 as Scalar);
        let b = Vec3::new(4.0 as Scalar, 6.0 as Scalar, 9.0 as Scalar);
        assert_eq!(a.distance_squared(b), 61.0 as Scalar);
    }

    #[test]
    fn vec3_distance_is_square_root_of_distance_squared() {
        let a = Vec3::new(1.0 as Scalar, 2.0 as Scalar, 3.0 as Scalar);
        let b = Vec3::new(4.0 as Scalar, 6.0 as Scalar, 9.0 as Scalar);
        let expected = (61.0 as Scalar).sqrt();
        let actual = a.distance(b);
        let epsilon = 1.0e-5 as Scalar;
        assert!((actual - expected).abs() <= epsilon);
    }

    #[test]
    fn vec3_normalize_or_zero_handles_zero_length() {
        let v = Vec3::ZERO;
        let normalized = v.normalize_or_zero(1.0e-5 as Scalar);
        assert_eq!(normalized, Vec3::ZERO);
    }

    #[test]
    fn vec3_normalize_or_uses_fallback_when_too_small() {
        let v = Vec3::new(0.0 as Scalar, 0.0 as Scalar, 0.0 as Scalar);
        let fallback = Vec3::new(1.0 as Scalar, 0.0 as Scalar, 0.0 as Scalar);
        let normalized = v.normalize_or(1.0e-5 as Scalar, fallback);
        assert_eq!(normalized, fallback);
    }

    #[test]
    fn vec3_normalize_produces_unit_length_for_non_zero() {
        let v = Vec3::new(2.0 as Scalar, 3.0 as Scalar, 6.0 as Scalar);
        let normalized = v
            .try_normalize(1.0e-5 as Scalar)
            .expect("expected unit vector");
        let length = normalized.length();
        let epsilon = 1.0e-5 as Scalar;
        assert!((length - 1.0 as Scalar).abs() <= epsilon);
    }

    #[cfg(feature = "nalgebra")]
    #[test]
    fn nalgebra_roundtrip() {
        let n = nalgebra::Vector3::<Scalar>::new(1.0 as Scalar, 2.0 as Scalar, 3.0 as Scalar);
        let v: Vec3 = n.into();
        let back: nalgebra::Vector3<Scalar> = v.into();
        assert_eq!(back, nalgebra::Vector3::new(v.x, v.y, v.z));
    }

    #[cfg(all(feature = "glam", feature = "f32"))]
    #[test]
    fn glam_roundtrip_f32() {
        let g = glam::Vec3::new(1.0, 2.0, 3.0);
        let v: Vec3 = g.into();
        let back: glam::Vec3 = v.into();
        assert_eq!(back, g);
    }

    #[cfg(all(feature = "glam", feature = "f64"))]
    #[test]
    fn glam_roundtrip_f64() {
        let g = glam::DVec3::new(1.0, 2.0, 3.0);
        let v: Vec3 = g.into();
        let back: glam::DVec3 = v.into();
        assert_eq!(back, g);
    }
}
