use bevy::prelude::*;
use iyes_loopless::prelude::*;

use crate::{
    input::FORWARD_KEYS,
    mesh_generation::{MulticolorMesh, MulticolorMeshMaterial},
    ordering::CurrentOrdering,
    GameState,
};

use super::GameType;

pub struct AppearTestPlugin;

impl Plugin for AppearTestPlugin {
    fn build(&self, app: &mut App) {
        app.add_enter_system(GameState::Play(GameType::AppearTest), enter_system)
            .add_system(step_system.run_in_state(GameState::Play(GameType::AppearTest)))
            .add_exit_system(GameState::Play(GameType::AppearTest), exit_system);
    }
}

fn enter_system(
    mut commands: Commands,
    material: Res<MulticolorMeshMaterial>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    MulticolorMesh::generate(&mut commands, &material, &mut meshes);
}

fn step_system(
    query: Query<&MulticolorMesh>,
    mut order: ResMut<CurrentOrdering>,
    mut meshes: ResMut<Assets<Mesh>>,
    keys: Res<Input<KeyCode>>,
) {
    if keys.any_just_pressed(FORWARD_KEYS) {
        if let Some(pixel) = order.next() {
            query.single().edit(&mut meshes).add_pixel(pixel);
        }
    }
}

fn exit_system() {}
