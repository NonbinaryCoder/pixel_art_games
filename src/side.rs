use std::ops::Not;

use bevy::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Side {
    Top = 0,
    Right = 1,
    Bottom = 2,
    Left = 3,
}

impl Side {
    pub const SIDES: [Side; 4] = [Side::Top, Side::Right, Side::Bottom, Side::Left];

    pub const fn art_direction(self) -> IVec2 {
        match self {
            Side::Top => IVec2::NEG_Y,
            Side::Right => IVec2::X,
            Side::Bottom => IVec2::Y,
            Side::Left => IVec2::NEG_X,
        }
    }

    pub const fn world_direction(self) -> Vec2 {
        match self {
            Side::Top => Vec2::Y,
            Side::Right => Vec2::X,
            Side::Bottom => Vec2::NEG_Y,
            Side::Left => Vec2::NEG_X,
        }
    }

    /// The angle of `self`, `0.0` for `Self::Top`
    pub fn angle_between(self, other: Self) -> f32 {
        let s = self as u8;
        let o = other as u8;
        if s == o {
            0.0
        } else if s + 1 == o {
            -std::f32::consts::FRAC_PI_2
        } else if s == o + 1 {
            std::f32::consts::FRAC_PI_2
        } else {
            std::f32::consts::PI
        }
    }

    /// If the provided vector is facing toward `Side::Top`,
    /// returns a vector facing `self`
    pub fn rotate_world_direction(self, dir: Vec2) -> Vec2 {
        let Vec2 { x, y } = dir;
        match self {
            Side::Top => Vec2::new(x, y),
            Side::Right => Vec2::new(y, -x),
            Side::Bottom => Vec2::new(-x, -y),
            Side::Left => Vec2::new(-y, x),
        }
    }

    pub const fn flip(self) -> Self {
        match self {
            Side::Top => Side::Bottom,
            Side::Right => Side::Left,
            Side::Bottom => Side::Top,
            Side::Left => Side::Right,
        }
    }

    pub const fn rotate_left(self) -> Self {
        match self {
            Side::Top => Side::Left,
            Side::Right => Side::Top,
            Side::Bottom => Side::Right,
            Side::Left => Side::Bottom,
        }
    }

    pub const fn rotate_right(self) -> Self {
        match self {
            Side::Top => Side::Right,
            Side::Right => Side::Bottom,
            Side::Bottom => Side::Left,
            Side::Left => Side::Top,
        }
    }

    pub const fn rotate(self, dir: LeftRight) -> Self {
        match dir {
            LeftRight::Left => self.rotate_left(),
            LeftRight::Right => self.rotate_right(),
        }
    }

    pub const fn rotate_left_corner(self) -> Corner {
        match self {
            Side::Top => Corner::TopLeft,
            Side::Right => Corner::TopRight,
            Side::Bottom => Corner::BottomRight,
            Side::Left => Corner::BottomLeft,
        }
    }

    pub const fn rotate_right_corner(self) -> Corner {
        match self {
            Side::Top => Corner::TopRight,
            Side::Right => Corner::BottomRight,
            Side::Bottom => Corner::BottomLeft,
            Side::Left => Corner::TopLeft,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Corner {
    TopLeft,
    TopRight,
    BottomRight,
    BottomLeft,
}

impl Corner {
    pub const fn rotate_left(self) -> Self {
        match self {
            Corner::TopLeft => Corner::BottomLeft,
            Corner::TopRight => Corner::TopLeft,
            Corner::BottomRight => Corner::TopRight,
            Corner::BottomLeft => Corner::BottomRight,
        }
    }

    pub const fn rotate_right(self) -> Self {
        match self {
            Corner::TopLeft => Corner::TopRight,
            Corner::TopRight => Corner::BottomRight,
            Corner::BottomRight => Corner::BottomLeft,
            Corner::BottomLeft => Corner::TopLeft,
        }
    }

    pub const fn rotate_left_side(self) -> Side {
        match self {
            Corner::TopLeft => Side::Left,
            Corner::TopRight => Side::Top,
            Corner::BottomRight => Side::Right,
            Corner::BottomLeft => Side::Bottom,
        }
    }

    pub const fn rotate_right_side(self) -> Side {
        match self {
            Corner::TopLeft => Side::Top,
            Corner::TopRight => Side::Right,
            Corner::BottomRight => Side::Bottom,
            Corner::BottomLeft => Side::Left,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Edge {
    pub pos: UVec2,
    pub side: Side,
}

impl Edge {
    pub fn new(pos: UVec2, side: Side) -> Edge {
        Self { pos, side }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum LeftRight {
    Left,
    Right,
}

impl Not for LeftRight {
    type Output = Self;

    fn not(self) -> Self::Output {
        match self {
            LeftRight::Left => LeftRight::Right,
            LeftRight::Right => LeftRight::Left,
        }
    }
}
