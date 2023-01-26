use bevy::prelude::*;
use iyes_loopless::prelude::*;

use crate::{
    art::Art,
    grid::Grid,
    input::{LEFT_KEYS, RIGHT_KEYS},
    mesh_generation::{MulticolorMesh, MulticolorMeshMaterial},
    ordering::CurrentOrdering,
    side::{Corner, Side},
    world_pos, GameState,
};

use super::{Colors, GameType};

const STATE: GameState = GameState::Play(GameType::Cart);

const CART_HEIGHT: f32 = 0.2;

const SPEED: f32 = 1.5;

pub struct CartPlugin;

impl Plugin for CartPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SystemLabel)]
        enum Label {
            ShowCart,
        }

        app.add_enter_system(STATE, enter_system)
            .add_system(move_cart_system.run_in_state(STATE).before(Label::ShowCart))
            .add_system(show_cart_system.run_in_state(STATE))
            .add_system(cart_color_system.run_in_state(STATE))
            .add_exit_system(STATE, exit_system);
    }
}

#[derive(Debug, Component)]
enum Cart {
    OnSide {
        pixel: UVec2,
        side: Side,
        distance: f32,
        on_outside: bool,
    },
    CrossingCorner {
        pixel: UVec2,
        corner: Corner,
        distance: f32,
        on_outside: bool,
    },
}

#[derive(Debug, Clone, Copy, Component)]
struct CartPiece;

#[derive(Debug, Component)]
struct NextPixel;

#[derive(Debug, Resource)]
struct SetPixels(Grid<bool>);

type WithCartOrPiece = Or<(With<Cart>, With<CartPiece>)>;

fn enter_system(
    mut commands: Commands,
    material: Res<MulticolorMeshMaterial>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut ordering: ResMut<CurrentOrdering>,
    art: Res<Art>,
    colors: Res<Colors>,
) {
    let (_, mut editor) = MulticolorMesh::generate(&mut commands, &material, &mut meshes);

    let pixel = ordering.next().unwrap();
    editor.add_pixel(pixel);

    let mut grid = Grid::new(art.size());
    grid[pixel.pos] = true;
    commands.insert_resource(SetPixels(grid));

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: colors.primary_color,
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)),
            ..default()
        },
        Cart::OnSide {
            pixel: pixel.pos,
            side: Side::Top,
            distance: 0.0,
            on_outside: true,
        },
    ));

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: colors.primary_color,
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 1.0)),
            ..default()
        },
        CartPiece,
    ));

    if let Some(pixel) = ordering.peek() {
        commands.spawn((
            SpriteBundle {
                transform: Transform::from_translation(pixel.world_pos().extend(0.0)),
                sprite: Sprite {
                    color: pixel.color.transparent().into(),
                    custom_size: Some(Vec2::splat(0.8)),
                    ..default()
                },
                ..default()
            },
            NextPixel,
        ));
    }
}

fn move_cart_system(
    mut cart_query: Query<&mut Cart>,
    ground: Res<SetPixels>,
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let direction = match (keys.any_pressed(LEFT_KEYS), keys.any_pressed(RIGHT_KEYS)) {
        (true, false) => -1.0,
        (false, true) => 1.0,
        _ => return,
    };
    let mut cart = cart_query.single_mut();
    match &mut *cart {
        Cart::OnSide {
            pixel,
            side,
            distance,
            on_outside,
        } => {
            *distance += direction * time.delta_seconds() * SPEED;
            if *on_outside {
                if *distance < -CART_HEIGHT {
                    *cart = Cart::CrossingCorner {
                        pixel: *pixel,
                        corner: side.rotate_left_corner(),
                        distance: *distance + 1.0,
                        on_outside: true,
                    };
                } else if *distance > CART_HEIGHT {
                    *cart = Cart::CrossingCorner {
                        pixel: *pixel,
                        corner: side.rotate_right_corner(),
                        distance: *distance - CART_HEIGHT,
                        on_outside: true,
                    };
                }
            } else {
                todo!()
            }
        }
        Cart::CrossingCorner {
            pixel,
            corner,
            distance,
            on_outside,
        } => {
            *distance += direction * time.delta_seconds() * SPEED;
            if *on_outside {
                if *distance < 0.0 {
                    *cart = Cart::OnSide {
                        pixel: *pixel,
                        side: corner.rotate_left_side(),
                        distance: CART_HEIGHT - *distance,
                        on_outside: true,
                    };
                } else if *distance > 1.0 - CART_HEIGHT {
                    *cart = Cart::OnSide {
                        pixel: *pixel,
                        side: corner.rotate_right_side(),
                        distance: *distance - 1.0,
                        on_outside: true,
                    };
                }
            } else {
                todo!()
            }
        }
    }
}

#[allow(clippy::type_complexity)]
fn show_cart_system(
    mut cart_query: Query<
        (&Cart, &mut Transform, &mut Sprite),
        (Changed<Cart>, Without<CartPiece>),
    >,
    mut piece_query: Query<(&mut Transform, &mut Sprite), (With<CartPiece>, Without<Cart>)>,
) {
    let outside_y = (1.0 + CART_HEIGHT) * 0.5;
    let inside_y = (1.0 - CART_HEIGHT) * 0.5;
    let get_y = |on_outside| match on_outside {
        true => outside_y,
        false => inside_y,
    };
    let cart_extent = |on_outside| match on_outside {
        true => 0.5 + CART_HEIGHT,
        false => 0.5,
    };

    if let Ok((cart, mut cart_transform, mut cart_sprite)) = cart_query.get_single_mut() {
        let (mut piece_transform, mut piece_sprite) = piece_query.single_mut();
        dbg!(&cart);
        match cart {
            Cart::OnSide {
                pixel,
                side,
                distance,
                on_outside,
            } => {
                let y = get_y(*on_outside);
                let offset = side.rotate_direction(Vec2::new(*distance, y));
                let pos = world_pos(*pixel) + offset;
                cart_transform.translation.x = pos.x;
                cart_transform.translation.y = pos.y;

                cart_sprite.custom_size = Some(match *side {
                    Side::Top | Side::Bottom => Vec2::new(1.0, CART_HEIGHT),
                    Side::Left | Side::Right => Vec2::new(CART_HEIGHT, 1.0),
                });

                piece_transform.translation = cart_transform.translation;
                piece_sprite.custom_size = cart_sprite.custom_size;
            }
            Cart::CrossingCorner {
                pixel,
                corner,
                distance,
                on_outside,
            } => {
                let world_pos = world_pos(*pixel);
                let y = get_y(*on_outside);
                let cart_extent = cart_extent(*on_outside);

                let left_side = corner.rotate_left_side();
                let left_width = 1.0 - distance;
                let left_offset =
                    left_side.rotate_direction(Vec2::new(cart_extent - left_width * 0.5, y));
                let left_pos = world_pos + left_offset;

                let right_side = corner.rotate_right_side();
                let right_width = distance + CART_HEIGHT;
                let right_offset =
                    right_side.rotate_direction(Vec2::new(-(cart_extent - right_width * 0.5), y));
                let right_pos = world_pos + right_offset;

                cart_transform.translation.x = left_pos.x;
                cart_transform.translation.y = left_pos.y;
                cart_sprite.custom_size = Some(match left_side {
                    Side::Top | Side::Bottom => Vec2::new(left_width, CART_HEIGHT),
                    Side::Left | Side::Right => Vec2::new(CART_HEIGHT, left_width),
                });

                // piece_transform.translation = cart_transform.translation;
                // piece_sprite.custom_size = cart_sprite.custom_size;
                piece_transform.translation.x = right_pos.x;
                piece_transform.translation.y = right_pos.y;
                piece_sprite.custom_size = Some(match right_side {
                    Side::Top | Side::Bottom => Vec2::new(right_width, CART_HEIGHT),
                    Side::Left | Side::Right => Vec2::new(CART_HEIGHT, right_width),
                })
            }
        }
    }
}

fn cart_color_system(mut query: Query<&mut Sprite, WithCartOrPiece>, colors: Res<Colors>) {
    if colors.is_changed() {
        for mut sprite in &mut query {
            sprite.color = colors.primary_color;
        }
    }
}

#[allow(clippy::type_complexity)]
fn exit_system(
    mut commands: Commands,
    query: Query<Entity, Or<(With<Cart>, With<CartPiece>, With<NextPixel>)>>,
) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
    commands.remove_resource::<SetPixels>();
}
