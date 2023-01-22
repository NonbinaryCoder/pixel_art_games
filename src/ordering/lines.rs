use bevy::{ecs::system::EntityCommands, prelude::*};

use crate::game::{ColorType, Colors};

pub struct LinesPlugin;

impl Plugin for LinesPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system_to_stage(CoreStage::PostUpdate, adjust_lines_system)
            .add_system(update_colors_system);
    }
}

#[derive(Debug, Component)]
pub struct Line {
    pub points: [Vec2; 2],
    pub color: ColorType,
}

impl Line {
    pub fn spawn<'w, 's, 'a>(
        self,
        commands: &'a mut Commands<'w, 's>,
        z: f32,
    ) -> EntityCommands<'w, 's, 'a> {
        commands.spawn((
            SpriteBundle {
                transform: Transform::from_translation(Vec3::new(0.0, 0.0, z)),
                ..default()
            },
            self,
        ))
    }
}

fn adjust_lines_system(
    mut query: Query<(&Line, &mut Transform, &mut Sprite), Changed<Line>>,
    colors: Res<Colors>,
) {
    for (line, mut transform, mut sprite) in query.iter_mut() {
        sprite.color = colors.color(line.color);

        let center_point = line.points[0].lerp(line.points[1], 0.5);
        transform.translation.x = center_point.x;
        transform.translation.y = center_point.y;

        let slope = (line.points[0].y - line.points[1].y) / (line.points[0].x - line.points[1].x);
        transform.rotation = Quat::from_rotation_z(slope.atan());

        let size = Vec2::new(line.points[0].distance(line.points[1]), 0.2);
        sprite.custom_size = Some(size);
    }
}

fn update_colors_system(mut query: Query<(&Line, &mut Sprite)>, colors: Res<Colors>) {
    if colors.is_changed() {
        for (line, mut sprite) in query.iter_mut() {
            sprite.color = colors.color(line.color);
        }
    }
}
