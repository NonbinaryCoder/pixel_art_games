use bevy::prelude::*;
use iyes_loopless::prelude::*;

use game::GameType;
use ordering::OrderingType;

mod art;
mod camera;
mod game;
mod input;
mod menu;
mod mesh_generation;
mod ordering;

fn main() {
    App::new()
        .add_loopless_state(GameState::MainMenu)
        .init_resource::<art::Art>()
        .insert_resource(ClearColor(Color::PURPLE))
        .add_plugins(DefaultPlugins)
        .add_plugin(camera::CameraPlugin)
        .add_plugin(game::GamePlugin)
        .add_plugin(menu::MenuPlugin)
        .add_plugin(mesh_generation::MeshGenerationPlugin)
        .add_plugin(ordering::OrderingPlugin)
        .run();
}

type PixelColor = [f32; 4];

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
enum GameState {
    #[default]
    MainMenu,
    Generate(OrderingType),
    Play(GameType),
}
