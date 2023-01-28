use bevy::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Side {
    Top,
    Right,
    Bottom,
    Left,
}

impl Side {
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
