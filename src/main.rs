use bevy::prelude::*;

mod game;
mod menu;
mod ordering;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(menu::MenuPlugin)
        .run();
}
