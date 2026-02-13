use crate::Scalar;
use spacetimedb::SpacetimeType;

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
