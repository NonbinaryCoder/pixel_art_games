use std::{env, path::Path};

use art::{Art, ArtName};
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
    let mut enter_state = GameState::AwaitingImage;
    let mut app = App::new();
    if let Some(path) = env::args().nth(1) {
        let path = Path::new(&path);
        match Art::load_from_path(path) {
            Ok(art) => {
                app.insert_resource(art);
                app.insert_resource(ArtName(path.file_name().map_or_else(
                    || "{unknown}".to_owned(),
                    |name| name.to_string_lossy().to_string(),
                )));
                enter_state = GameState::MainMenu;
            }
            Err(err) => {
                app.insert_resource(ArtName(err));
            }
        }
    }
    app.add_loopless_state(enter_state)
        .insert_resource(ClearColor(Color::WHITE))
        .add_plugins(DefaultPlugins)
        .add_plugin(camera::CameraPlugin)
        .add_plugin(game::GamePlugin)
        .add_plugin(menu::MenuPlugin)
        .add_plugin(mesh_generation::MeshGenerationPlugin)
        .add_plugin(ordering::OrderingPlugin)
        .run();
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
enum GameState {
    #[default]
    AwaitingImage,
    MainMenu,
    Generate(OrderingType),
    Play(GameType),
}
