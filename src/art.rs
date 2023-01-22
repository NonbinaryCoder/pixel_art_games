use std::{ops::Index, path::Path};

use bevy::prelude::*;
use bevy_editor_pls::egui::{self, RichText};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PixelColor([u8; 4]);

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
        self.pos.as_vec2() * Vec2::new(1.0, -1.0)
    }
}

#[derive(Debug, Resource)]
pub struct Art {
    width: usize,
    data: Vec<Option<PixelColor>>,
}

impl Index<UVec2> for Art {
    type Output = Option<PixelColor>;

    fn index(&self, index: UVec2) -> &Self::Output {
        &self.data[index.y as usize * self.width + index.x as usize]
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

        Ok(Self {
            width: image.width() as usize,
            data: image
                .pixels()
                .map(|&image::Rgba(p)| (p[3] > 0).then_some(p.into()))
                .collect(),
        })
    }

    pub fn width(&self) -> u32 {
        self.width as u32
    }

    pub fn height(&self) -> u32 {
        (self.data.len() / self.width) as u32
    }

    pub fn size(&self) -> UVec2 {
        UVec2::new(self.width(), self.height())
    }

    pub fn rows(&self) -> std::slice::Chunks<Option<PixelColor>> {
        self.data.chunks(self.width)
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
