use std::{ops::Index, path::Path};

use bevy::prelude::*;
use bevy_egui::egui::{self, RichText};

use crate::{grid::Grid, world_pos};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PixelColor([u8; 4]);

impl PixelColor {
    pub fn transparent(self) -> Self {
        let p = self.0;
        PixelColor([p[0], p[1], p[2], p[3] / 2])
    }
}

impl From<[u8; 4]> for PixelColor {
    fn from(value: [u8; 4]) -> Self {
        Self(value)
    }
}

impl From<PixelColor> for Color {
    fn from(value: PixelColor) -> Self {
        Color::rgba_u8(value.0[0], value.0[1], value.0[2], value.0[3])
    }
}

impl From<PixelColor> for [f32; 4] {
    fn from(value: PixelColor) -> Self {
        Color::from(value).as_linear_rgba_f32()
    }
}

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

    pub fn world_pos(&self) -> Vec2 {
        world_pos(self.pos)
    }
}

#[derive(Debug, Resource)]
pub struct Art(Grid<Option<PixelColor>>);

impl Index<UVec2> for Art {
    type Output = Option<PixelColor>;

    fn index(&self, index: UVec2) -> &Self::Output {
        &self.0[index]
    }
}

impl Art {
    pub fn load_from_path(path: &Path) -> Result<Self, String> {
        let image = image::io::Reader::open(path)
            .map_err(|e| format!("Unable to open file: {e}"))?
            .decode()
            .map_err(|e| format!("Unable to decode image: {e}"))?
            .into_rgba8();

        if image.width() < 2 || image.height() < 2 {
            return Err("Image must be at least 2x2".to_owned());
        }

        let data: Vec<_> = image
            .pixels()
            .map(|&image::Rgba(p)| (p[3] > 0).then_some(p.into()))
            .collect();

        if data.iter().any(|p| p.is_some()) {
            Ok(Art(Grid::from_vec(data, image.width() as usize)))
        } else {
            Err("Image must have at least one pixel".to_owned())
        }
    }

    pub fn width(&self) -> u32 {
        self.0.width()
    }

    pub fn height(&self) -> u32 {
        self.0.height()
    }

    pub fn size(&self) -> UVec2 {
        self.0.size()
    }

    pub fn rows(&self) -> std::slice::Chunks<Option<PixelColor>> {
        self.0.rows()
    }

    pub fn pixel(&self, pos: UVec2) -> Option<Pixel> {
        self[pos].map(|color| Pixel { pos, color })
    }
}

#[derive(Debug, Resource)]
pub struct ArtName(pub String);

impl ArtName {
    pub fn show(&self, context: &egui::Context) {
        egui::TopBottomPanel::bottom("art_name")
            .show_separator_line(false)
            .resizable(false)
            .show(context, |ui| {
                ui.label(RichText::new(&self.0).size(30.0));
            });
    }
}
