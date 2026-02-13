use crate::{Scalar, Vec3};
use spacetimedb::SpacetimeType;

/// A 2D vector using `x/y`.
///
/// ```
/// use spacetimedb_math::Vec2;
///
/// let v = Vec2::new(1.0, 2.0);
/// assert_eq!(v.x, 1.0);
/// assert_eq!(v.y, 2.0);
/// ```
#[derive(SpacetimeType, Debug, Default, Clone, Copy, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Vec2 {
    pub x: Scalar,
    pub y: Scalar,
}

impl Vec2 {
    pub const ZERO: Vec2 = Vec2::new(0.0, 0.0);
    pub const ONE: Vec2 = Vec2::new(1.0, 1.0);

    #[inline(always)]
    pub const fn new(x: Scalar, y: Scalar) -> Self {
        Vec2 { x, y }
    }

    /// Extend this vector into 3D by inserting `y` and treating this vector as XZ.
    ///
    /// This is the inverse of `Vec3::xz()`.
    #[inline]
    pub const fn extend_y(&self, y: Scalar) -> Vec3 {
        Vec3::new(self.x, y, self.y)
    }

    /// Extend this vector into 3D by inserting `z` and treating this vector as XY.
    #[inline]
    pub const fn extend_z(&self, z: Scalar) -> Vec3 {
        Vec3::new(self.x, self.y, z)
    }

    /// Returns the dot product of this vector and `other`.
    ///
    /// The dot product returns a single number (a scalar) that tells you the relationship between the two directions:
    /// - Positive (> 0): The vectors are facing generally the same direction (angle < 90°).
    /// - Zero (0): The vectors are perpendicular (exactly 90°).
    /// - Negative (< 0): The vectors are facing away from each other (angle > 90°).
    #[inline]
    pub fn dot(&self, other: Vec2) -> Scalar {
        self.x * other.x + self.y * other.y
    }

    /// Returns the squared length (magnitude) of this vector.
    #[inline]
    pub fn length_squared(&self) -> Scalar {
        self.x * self.x + self.y * self.y
    }

    /// Returns the length (magnitude) of this vector.
    pub fn length(&self) -> Scalar {
        self.length_squared().sqrt()
    }

    /// Returns the squared distance between this vector and `other`.
    #[inline]
    pub fn distance_squared(&self, other: Vec2) -> Scalar {
        let dx = other.x - self.x;
        let dy = other.y - self.y;
        dx * dx + dy * dy
    }

    /// Returns the distance between this vector and `other`.
    pub fn distance(&self, other: Vec2) -> Scalar {
        self.distance_squared(other).sqrt()
    }

    /// Returns a normalized vector, or `fallback` if length is below `epsilon`.
    pub fn normalize_or(&self, epsilon: Scalar, fallback: Vec2) -> Vec2 {
        let len_sq = self.length_squared();
        let epsilon_sq = epsilon * epsilon;
        if len_sq <= epsilon_sq {
            fallback
        } else {
            let len = len_sq.sqrt();
            Vec2::new(self.x / len, self.y / len)
        }
    }

    /// Returns a normalized vector, or `Vec2::ZERO` if length is below `epsilon`.
    pub fn normalize_or_zero(&self, epsilon: Scalar) -> Vec2 {
        self.normalize_or(epsilon, Vec2::ZERO)
    }

    /// Attempts to normalize this vector, returning `None` if length is below `epsilon`.
    pub fn try_normalize(&self, epsilon: Scalar) -> Option<Vec2> {
        let len_sq = self.length_squared();
        let epsilon_sq = epsilon * epsilon;
        if len_sq <= epsilon_sq {
            None
        } else {
            let len = len_sq.sqrt();
            Some(Vec2::new(self.x / len, self.y / len))
        }
    }
}

#[cfg(feature = "nalgebra")]
mod nalgebra_impls {
    use super::*;

    impl From<nalgebra::Vector2<Scalar>> for Vec2 {
        #[inline(always)]
        fn from(v: nalgebra::Vector2<Scalar>) -> Self {
            Vec2::new(v.x, v.y)
        }
    }

    impl From<Vec2> for nalgebra::Vector2<Scalar> {
        #[inline(always)]
        fn from(v: Vec2) -> Self {
            nalgebra::Vector2::new(v.x, v.y)
        }
    }
}

#[cfg(feature = "glam")]
mod glam_impls {
    use super::*;

    #[cfg(feature = "f32")]
    impl From<glam::Vec2> for Vec2 {
        #[inline(always)]
        fn from(v: glam::Vec2) -> Self {
            Vec2::new(v.x, v.y)
        }
    }

    #[cfg(feature = "f32")]
    impl From<Vec2> for glam::Vec2 {
        #[inline(always)]
        fn from(v: Vec2) -> Self {
            glam::Vec2::new(v.x, v.y)
        }
    }

    #[cfg(feature = "f64")]
    impl From<glam::DVec2> for Vec2 {
        #[inline(always)]
        fn from(v: glam::DVec2) -> Self {
            Vec2::new(v.x, v.y)
        }
    }

    #[cfg(feature = "f64")]
    impl From<Vec2> for glam::DVec2 {
        #[inline(always)]
        fn from(v: Vec2) -> Self {
            glam::DVec2::new(v.x, v.y)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn vec2_new_sets_xy() {
        let v = Vec2::new(1.0 as Scalar, 2.0 as Scalar);
        assert_eq!(v.x, 1.0 as Scalar);
        assert_eq!(v.y, 2.0 as Scalar);
    }

    #[test]
    fn vec2_extend_y_treats_vec2_as_xz() {
        let v = Vec2::new(1.0 as Scalar, 3.0 as Scalar);
        let extended = v.extend_y(2.0 as Scalar);
        assert_eq!(
            extended,
            Vec3::new(1.0 as Scalar, 2.0 as Scalar, 3.0 as Scalar)
        );
    }

    #[test]
    fn vec2_extend_z_treats_vec2_as_xy() {
        let v = Vec2::new(1.0 as Scalar, 2.0 as Scalar);
        let extended = v.extend_z(3.0 as Scalar);
        assert_eq!(
            extended,
            Vec3::new(1.0 as Scalar, 2.0 as Scalar, 3.0 as Scalar)
        );
    }

    #[test]
    fn vec2_dot_is_sum_of_component_products() {
        let a = Vec2::new(1.0 as Scalar, 2.0 as Scalar);
        let b = Vec2::new(3.0 as Scalar, 4.0 as Scalar);
        assert_eq!(a.dot(b), 11.0 as Scalar);
    }

    #[test]
    fn vec2_length_squared_is_sum_of_squares() {
        let v = Vec2::new(3.0 as Scalar, 4.0 as Scalar);
        assert_eq!(v.length_squared(), 25.0 as Scalar);
    }

    #[test]
    fn vec2_length_is_square_root_of_length_squared() {
        let v = Vec2::new(3.0 as Scalar, 4.0 as Scalar);
        assert_eq!(v.length(), 5.0 as Scalar);
    }

    #[test]
    fn vec2_distance_squared_is_sum_of_delta_squares() {
        let a = Vec2::new(1.0 as Scalar, 2.0 as Scalar);
        let b = Vec2::new(4.0 as Scalar, 6.0 as Scalar);
        assert_eq!(a.distance_squared(b), 25.0 as Scalar);
    }

    #[test]
    fn vec2_distance_is_square_root_of_distance_squared() {
        let a = Vec2::new(1.0 as Scalar, 2.0 as Scalar);
        let b = Vec2::new(4.0 as Scalar, 6.0 as Scalar);
        assert_eq!(a.distance(b), 5.0 as Scalar);
    }

    #[test]
    fn vec2_normalize_or_zero_handles_zero_length() {
        let v = Vec2::ZERO;
        let normalized = v.normalize_or_zero(1.0e-5 as Scalar);
        assert_eq!(normalized, Vec2::ZERO);
    }

    #[test]
    fn vec2_normalize_or_uses_fallback_when_too_small() {
        let v = Vec2::new(0.0 as Scalar, 0.0 as Scalar);
        let fallback = Vec2::new(1.0 as Scalar, 0.0 as Scalar);
        let normalized = v.normalize_or(1.0e-5 as Scalar, fallback);
        assert_eq!(normalized, fallback);
    }

    #[test]
    fn vec2_normalize_produces_unit_length_for_non_zero() {
        let v = Vec2::new(3.0 as Scalar, 4.0 as Scalar);
        let normalized = v
            .try_normalize(1.0e-5 as Scalar)
            .expect("expected unit vector");
        let length = normalized.length();
        let epsilon = 1.0e-5 as Scalar;
        assert!((length - 1.0 as Scalar).abs() <= epsilon);
    }

    #[cfg(feature = "nalgebra")]
    #[test]
    fn vec2_nalgebra_round_trip() {
        let v = Vec2::new(3.0 as Scalar, 4.0 as Scalar);
        let n: nalgebra::Vector2<Scalar> = v.into();
        assert_eq!(n.x, 3.0 as Scalar);
        assert_eq!(n.y, 4.0 as Scalar);

        let back: Vec2 = n.into();
        assert_eq!(back, v);
    }

    #[cfg(all(feature = "glam", feature = "f32"))]
    #[test]
    fn vec2_glam_f32_round_trip() {
        let v = Vec2::new(5.0 as Scalar, 6.0 as Scalar);
        let g: glam::Vec2 = v.into();
        assert_eq!(g.x, 5.0);
        assert_eq!(g.y, 6.0);

        let back: Vec2 = g.into();
        assert_eq!(back, v);
    }

    #[cfg(all(feature = "glam", feature = "f64"))]
    #[test]
    fn vec2_glam_f64_round_trip() {
        let v = Vec2::new(7.0 as Scalar, 8.0 as Scalar);
        let g: glam::DVec2 = v.into();
        assert_eq!(g.x, 7.0);
        assert_eq!(g.y, 8.0);

        let back: Vec2 = g.into();
        assert_eq!(back, v);
    }
}
