use bevy::prelude::*;

mod appear_test;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(appear_test::AppearTestPlugin);
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameType {
    #[default]
    AppearTest,
    Cart,
}
