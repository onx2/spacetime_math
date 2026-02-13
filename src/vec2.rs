use crate::Scalar;
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
