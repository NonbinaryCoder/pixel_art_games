use bevy::{
    ecs::system::EntityCommands,
    prelude::{shape::Circle, *},
    sprite::Mesh2dHandle,
};

use crate::game::Colors;

pub struct PrefabsPlugin;

impl Plugin for PrefabsPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(startup_system);
    }
}

#[derive(Debug, Resource)]
pub struct CircleMesh(Mesh2dHandle);

fn startup_system(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>) {
    commands.insert_resource(CircleMesh(meshes.add(Circle::new(0.5).into()).into()));
}

pub fn circle<'w, 's, 'a>(
    commands: &'a mut Commands<'w, 's>,
    colors: &Colors,
    mesh: &CircleMesh,
    translation: Vec3,
) -> EntityCommands<'w, 's, 'a> {
    commands.spawn(ColorMesh2dBundle {
        mesh: mesh.0.clone(),
        material: colors.secondary_material.clone(),
        transform: Transform::from_translation(translation).with_scale(Vec3::splat(0.8)),
        ..default()
    })
}
