use bevy::prelude::*;

pub mod next_pixel;

pub struct PixelPlugin;

impl Plugin for PixelPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(next_pixel::NextPixelPlugin);
    }
}
