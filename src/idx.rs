//! idx.rs -- Indexer values
//!
//!
//!

use super::ERROR;

pub enum IdxXYZ {
    X,
    Y,
    Z,
}
pub enum IdxUVW {
    U,
    V,
    W,
}
pub enum IdxRGB {
    R,
    G,
    B,
}
pub enum IdxVec3d {
    Fst,
    Snd,
    Trd,
}
pub enum IdxXY {
    X,
    Y,
}
pub enum IdxUV {
    U,
    V,
}

pub enum IdxVec2d {
    Fst,
    Snd,
}

impl From<usize> for IdxRGB {
    fn from(value: usize) -> Self {
        assert!(value < 3);
        match value {
            0 => Self::R,
            1 => Self::G,
            2 => Self::B,
            x => panic!("{ERROR} can't convert the value into IdxRGB: {}", x),
        }
    }
}

impl From<usize> for IdxUVW {
    fn from(value: usize) -> Self {
        assert!(value < 3);
        match value {
            0 => Self::U,
            1 => Self::V,
            2 => Self::W,
            x => panic!("{ERROR} can't convert the value into IdxUVW: {}", x),
        }
    }
}

impl From<usize> for IdxXYZ {
    fn from(value: usize) -> Self {
        assert!(value < 3);
        match value {
            0 => Self::X,
            1 => Self::Y,
            2 => Self::Z,
            x => panic!("{ERROR} can't convert the value into IdxXYZ: {}", x),
        }
    }
}

impl From<usize> for IdxUV {
    fn from(value: usize) -> Self {
        assert!(value < 2);
        match value {
            0 => Self::U,
            1 => Self::V,
            x => panic!("{ERROR} can't convert the value into IdxUV: {}", x),
        }
    }
}

impl From<usize> for IdxXY {
    fn from(value: usize) -> Self {
        assert!(value < 2);
        match value {
            0 => Self::X,
            1 => Self::Y,
            x => panic!("{ERROR} can't convert the value into IdxXY: {}", x),
        }
    }
}

pub trait Indexer {
    type Label;

    fn idx_label(&self) -> Self::Label;
}

impl Indexer for IdxXYZ {
    type Label = IdxVec3d;

    fn idx_label(&self) -> Self::Label {
        match self {
            Self::X => IdxVec3d::Fst,
            Self::Y => IdxVec3d::Snd,
            Self::Z => IdxVec3d::Trd,
        }
    }
}

impl Indexer for IdxUVW {
    type Label = IdxVec3d;

    fn idx_label(&self) -> Self::Label {
        match self {
            Self::U => IdxVec3d::Fst,
            Self::V => IdxVec3d::Snd,
            Self::W => IdxVec3d::Trd,
        }
    }
}

impl Indexer for IdxRGB {
    type Label = IdxVec3d;

    fn idx_label(&self) -> Self::Label {
        match self {
            Self::R => IdxVec3d::Fst,
            Self::G => IdxVec3d::Snd,
            Self::B => IdxVec3d::Trd,
        }
    }
}

impl Indexer for IdxXY {
    type Label = IdxVec2d;

    fn idx_label(&self) -> Self::Label {
        match self {
            Self::X => IdxVec2d::Fst,
            Self::Y => IdxVec2d::Snd,
        }
    }
}

impl Indexer for IdxUV {
    type Label = IdxVec2d;

    fn idx_label(&self) -> Self::Label {
        match self {
            Self::U => IdxVec2d::Fst,
            Self::V => IdxVec2d::Snd,
        }
    }
}
