use bevy::{
    ecs::query::{self, QuerySingleError},
    prelude::*,
};

use crate::{art::Pixel, ordering::CurrentOrdering, world_pos};

pub struct NextPixelPlugin;

impl Plugin for NextPixelPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(move_next_pixel_system);
    }
}

#[derive(Debug, Component)]
pub struct NextPixel {
    start_pos: Vec2,
    ideal_pos: Vec2,
    t: f32,
}

impl NextPixel {
    pub fn spawn(pixel: Pixel, commands: &mut Commands) {
        let pos = world_pos(pixel.pos);
        commands.spawn((
            SpriteBundle {
                transform: Transform::from_translation(pos.extend(-1.0)),
                sprite: Sprite {
                    color: pixel.color.transparent().into(),
                    custom_size: Some(Vec2::splat(0.8)),
                    ..default()
                },
                ..default()
            },
            NextPixel {
                start_pos: pos,
                ideal_pos: pos,
                t: 2.0,
            },
        ));
    }

    pub fn show_current<T: query::ReadOnlyWorldQuery>(
        commands: &mut Commands,
        mut query: Query<(&mut Self, &mut Sprite, Entity), T>,
        ordering: &CurrentOrdering,
    ) -> Result<(), QuerySingleError> {
        query
            .get_single_mut()
            .map(|(mut this, mut sprite, entity)| {
                if let Some(pixel) = ordering.peek() {
                    this.start_pos = this.current_pos();
                    this.ideal_pos = pixel.world_pos();
                    this.t = 0.0;
                    sprite.color = pixel.color.transparent().into();
                } else {
                    commands.entity(entity).despawn();
                }
            })
    }

    pub fn current_pos(&self) -> Vec2 {
        self.start_pos
            .lerp(self.ideal_pos, ezing::quart_inout(self.t.min(1.0)))
    }
}

fn move_next_pixel_system(mut query: Query<(&mut NextPixel, &mut Transform)>, time: Res<Time>) {
    if let Ok((mut next_pixel, mut transform)) = query.get_single_mut() {
        if next_pixel.t < 1.0 {
            next_pixel.t += time.delta_seconds();
            let t = next_pixel.t.min(1.0);
            transform.translation = next_pixel
                .start_pos
                .lerp(next_pixel.ideal_pos, ezing::quart_inout(t))
                .extend(-1.0);
        }
    }
}
