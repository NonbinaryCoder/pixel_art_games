use bevy::{math::Vec3Swizzles, prelude::*};
use iyes_loopless::prelude::*;

use crate::{
    art::{Art, Pixel},
    game::{ColorType, Colors},
    input::FORWARD_KEYS,
    mesh_generation::{MulticolorMesh, MulticolorMeshMaterial},
    ordering::{lines::Line, Ordering, OrderingType, SPEED},
    prefabs::{self, CircleMesh},
    GameState,
};

const STATE: GameState = GameState::Generate(OrderingType::Default);

const INITIAL_FALL_VELOCITY: f32 = 0.25;

pub struct DefaultPlugin;

impl Plugin for DefaultPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_enter_system(STATE, enter_system)
            .add_system(step_system.run_in_state(STATE))
            .add_exit_system(STATE, exit_system);
    }
}

pub fn generate_fast(art: &Art) -> Ordering {
    let mut data = Vec::new();
    for (row, y) in art.rows().zip(0..) {
        for (&color, x) in row.iter().zip(0..) {
            if let Some(color) = color {
                data.push(Pixel::new(x, y, color));
            }
        }
    }
    Ordering { data }
}

#[derive(Debug, Component)]
enum Dot {
    MovingForward { art_pos: UVec2, pos: Vec2, t: f32 },
    MovingDown { y: u32, t: f32, start_pos: Vec2 },
    Falling { pos: Vec2, y_velocity: f32 },
}

impl Dot {
    fn tick(&mut self, time: &Time, art: &Art) -> Option<Pixel> {
        match self {
            Dot::MovingForward { art_pos, pos, t } => {
                *t += time.delta_seconds() * SPEED;
                if *t > 1.0 {
                    *t -= 1.0;
                    art_pos.x += 1;
                    pos.x += 1.0;
                    let ret = art.pixel(*art_pos);
                    if art_pos.x + 1 >= art.width() {
                        if art_pos.y + 1 < art.height() {
                            *self = Dot::MovingDown {
                                y: art_pos.y + 1,
                                t: *t,
                                start_pos: *pos,
                            }
                        } else {
                            *self = Dot::Falling {
                                pos: *pos,
                                y_velocity: INITIAL_FALL_VELOCITY,
                            }
                        }
                    }
                    ret
                } else {
                    None
                }
            }
            Dot::MovingDown { y, t, start_pos } => {
                *t += time.delta_seconds() * SPEED;
                if *t > 1.0 {
                    let art_pos = UVec2::new(0, *y);
                    *self = Dot::MovingForward {
                        art_pos,
                        pos: Vec2::new(0.0, start_pos.y - 1.0),
                        t: *t - 1.0,
                    };
                    art.pixel(art_pos)
                } else {
                    None
                }
            }
            Dot::Falling { pos, y_velocity } => {
                *y_velocity -= time.delta_seconds();
                pos.y += *y_velocity;
                None
            }
        }
    }

    fn update_position(&self, transform: &mut Transform) {
        let pos = match self {
            Dot::MovingForward { pos, t, .. } => *pos + Vec2::new(*t, 0.0),
            Dot::MovingDown { t, start_pos, .. } => {
                start_pos.lerp(Vec2::new(0.0, start_pos.y - 1.0), *t)
            }
            Dot::Falling { pos, .. } => *pos,
        };
        transform.translation.x = pos.x;
        transform.translation.y = pos.y;
    }
}

#[derive(Debug, Component)]
struct DotLine;

fn enter_system(
    mut commands: Commands,
    colors: Res<Colors>,
    circle_mesh: Res<CircleMesh>,
    mut meshes: ResMut<Assets<Mesh>>,
    material: Res<MulticolorMeshMaterial>,
    art: Res<Art>,
) {
    prefabs::circle(
        &mut commands,
        &colors,
        &circle_mesh,
        Vec3::new(0.0, 0.0, 1.0),
    )
    .insert(Dot::MovingForward {
        art_pos: UVec2::ZERO,
        pos: Vec2::ZERO,
        t: 0.0,
    });

    let (_, mut editor) = MulticolorMesh::generate(&mut commands, &material, &mut meshes);
    if let Some(pixel) = art.pixel(UVec2::ZERO) {
        editor.add_small_pixel(pixel);
    }
}

#[allow(clippy::too_many_arguments)]
fn step_system(
    mut commands: Commands,
    mut dot_query: Query<(&mut Dot, &mut Transform)>,
    mesh_query: Query<&MulticolorMesh>,
    mut line_query: Query<&mut Line, With<DotLine>>,
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
    art: Res<Art>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    if keys.any_pressed(FORWARD_KEYS) {
        let (mut dot, mut transform) = dot_query.single_mut();
        if let Some(pixel) = dot.tick(&time, &art) {
            let current_pos = pixel.world_pos();
            let line = line_query.get_single_mut();
            if let Some(point) = line
                .as_ref()
                .ok()
                .map(|l| l.points[0])
                .or_else(|| art[UVec2::ZERO].map(|_| Vec2::ZERO))
            {
                Line {
                    points: [point, current_pos],
                    color: ColorType::Primary,
                }
                .spawn(&mut commands, -1.0);
            }
            if let Ok(mut line) = line {
                line.points[0] = current_pos;
            } else {
                Line {
                    points: [current_pos; 2],
                    color: ColorType::Primary,
                }
                .spawn(&mut commands, -1.0)
                .insert(DotLine);
            }
            mesh_query.single().edit(&mut meshes).add_small_pixel(pixel);
        }
        dot.update_position(&mut transform);
        if let Ok(mut line) = line_query.get_single_mut() {
            line.points[1] = transform.translation.xy();
        }
    }
}

#[allow(clippy::type_complexity)]
fn exit_system(
    mut commands: Commands,
    query: Query<Entity, Or<(With<Dot>, With<MulticolorMesh>, With<Line>)>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
