use bevy::prelude::*;

pub mod default;
pub mod side_to_side;
pub mod spiral;

pub struct OrderingsPlugin;

impl Plugin for OrderingsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(default::DefaultPlugin);
    }
}
