#![warn(clippy::todo)]

use std::{env, path::Path};

use bevy::prelude::*;
use iyes_loopless::prelude::*;

use art::{Art, ArtName};
use game::GameType;
use ordering::OrderingType;

mod art;
mod camera;
mod game;
mod grid;
mod input;
mod menu;
mod mesh_generation;
mod ordering;
mod prefabs;
mod side;

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
        .insert_resource(ClearColor(Color::GRAY))
        .add_plugins(DefaultPlugins)
        .add_plugin(camera::CameraPlugin)
        .add_plugin(game::GamePlugin)
        .add_plugin(menu::MenuPlugin)
        .add_plugin(mesh_generation::MeshGenerationPlugin)
        .add_plugin(ordering::OrderingPlugin)
        .add_plugin(prefabs::PrefabsPlugin)
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

impl GameState {
    pub fn is_menu(self) -> bool {
        matches!(self, Self::AwaitingImage | Self::MainMenu)
    }

    pub fn current_is_menu(state: Res<CurrentState<Self>>) -> bool {
        state.0.is_menu()
    }
}

fn world_pos(pos: UVec2) -> Vec2 {
    pos.as_vec2() * Vec2::new(1.0, -1.0)
}
