use std::ops::Index;

use bevy::prelude::*;

use crate::PixelColor;

#[derive(Debug, Clone, Copy)]
pub struct Pixel {
    pub pos: UVec2,
    pub color: PixelColor,
}

impl Pixel {
    pub fn new(x: u32, y: u32, color: PixelColor) -> Self {
        Self {
            pos: UVec2 { x, y },
            color,
        }
    }
}

#[derive(Debug, Resource)]
pub struct Art {
    width: usize,
    data: Vec<Option<PixelColor>>,
}

impl Default for Art {
    fn default() -> Self {
        Self {
            width: 4,
            data: vec![
                Some([0.0, 0.0, 0.0, 1.0]),
                Some([1.0, 1.0, 1.0, 1.0]),
                Some([1.0, 1.0, 1.0, 1.0]),
                Some([1.0, 1.0, 0.0, 1.0]),
                Some([1.0, 1.0, 1.0, 1.0]),
                None,
                None,
                Some([1.0, 1.0, 1.0, 1.0]),
                Some([1.0, 1.0, 1.0, 1.0]),
                None,
                None,
                Some([1.0, 1.0, 1.0, 1.0]),
                Some([1.0, 0.0, 1.0, 1.0]),
                Some([1.0, 1.0, 1.0, 1.0]),
                Some([1.0, 1.0, 1.0, 1.0]),
                Some([0.0, 1.0, 1.0, 1.0]),
            ],
        }
    }
}

impl Index<UVec2> for Art {
    type Output = Option<PixelColor>;

    fn index(&self, index: UVec2) -> &Self::Output {
        &self.data[index.y as usize * self.width + index.x as usize]
    }
}

impl Art {
    pub fn rows(&self) -> std::slice::Chunks<Option<PixelColor>> {
        self.data.chunks(self.width)
    }
}
