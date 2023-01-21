use bevy::prelude::*;
use iyes_loopless::prelude::*;

use crate::{
    input::{FORWARD_KEYS, REPEAT_STEP_DURATION, WAIT_REPEAT_DURATION},
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

#[derive(Debug, Resource)]
enum State {
    None,
    Wait(Timer),
    Repeat(Timer),
}

fn enter_system(
    mut commands: Commands,
    material: Res<MulticolorMeshMaterial>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    MulticolorMesh::generate(&mut commands, &material, &mut meshes);
    commands.insert_resource(State::None);
}

fn step_system(
    query: Query<&MulticolorMesh>,
    mut order: ResMut<CurrentOrdering>,
    mut meshes: ResMut<Assets<Mesh>>,
    keys: Res<Input<KeyCode>>,
    mut state: ResMut<State>,
    time: Res<Time>,
) {
    if keys.any_just_pressed(FORWARD_KEYS) {
        *state = State::Wait(Timer::new(WAIT_REPEAT_DURATION, TimerMode::Once));
        query
            .single()
            .edit(&mut meshes)
            .add_next_from_ordering(&mut order);
    } else if keys.any_pressed(FORWARD_KEYS) {
        match &mut *state {
            State::None => *state = State::Wait(Timer::new(WAIT_REPEAT_DURATION, TimerMode::Once)),
            State::Wait(timer) => {
                if timer.tick(time.delta()).finished() {
                    *state = State::Repeat(Timer::new(REPEAT_STEP_DURATION, TimerMode::Repeating));
                }
            }
            State::Repeat(timer) => {
                let mut editor = query.single().edit(&mut meshes);
                for _ in 0..timer.tick(time.delta()).times_finished_this_tick() {
                    editor.add_next_from_ordering(&mut order);
                }
            }
        }
    } else {
        *state = State::None;
    }
}

fn exit_system(mut commands: Commands, mesh_query: Query<Entity, With<MulticolorMesh>>) {
    commands.entity(mesh_query.single()).despawn();
}
