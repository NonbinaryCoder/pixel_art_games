use bevy::prelude::*;
use iyes_loopless::prelude::*;

use crate::{input::EXIT_KEYS, GameState};

mod appear_test;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(appear_test::AppearTestPlugin)
            .add_system(exit_game_system.run_if_not(GameState::current_is_menu));
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameType {
    #[default]
    AppearTest,
    Cart,
}

fn exit_game_system(mut commands: Commands, keys: Res<Input<KeyCode>>) {
    if keys.any_just_pressed(EXIT_KEYS) {
        commands.insert_resource(NextState(GameState::MainMenu));
    }
}
