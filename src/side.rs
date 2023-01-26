use std::ops::Neg;

use bevy::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Side {
    Top,
    Right,
    Bottom,
    Left,
}

impl Side {
    /// If the provided vector is facing toward `Side::Top`,
    /// returns a vector facing `self`
    pub fn rotate_direction<T>(self, dir: T) -> T
    where
        T: Vector2d,
        T::Val: Neg<Output = T::Val>,
    {
        let [x, y] = dir.decompose();
        match self {
            Side::Top => T::new(x, y),
            Side::Right => T::new(y, -x),
            Side::Bottom => T::new(-x, -y),
            Side::Left => T::new(-y, x),
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

pub trait Vector2d {
    type Val;

    fn decompose(self) -> [Self::Val; 2];

    fn new(x: Self::Val, y: Self::Val) -> Self;
}

impl Vector2d for Vec2 {
    type Val = f32;

    fn decompose(self) -> [Self::Val; 2] {
        [self.x, self.y]
    }

    fn new(x: Self::Val, y: Self::Val) -> Self {
        Self::new(x, y)
    }
}
