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

const SPEED: f32 = 3.0;
const FALL_SPEED: f32 = 2.0;

pub struct CartPlugin;

impl Plugin for CartPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SystemLabel)]
        enum CartSystem {
            MoveCart,
        }

        app.add_enter_system(STATE, enter_system)
            .add_system(
                move_cart_system
                    .run_in_state(STATE)
                    .label(CartSystem::MoveCart),
            )
            .add_system(
                show_cart_system
                    .run_in_state(STATE)
                    .after(CartSystem::MoveCart),
            )
            .add_system(
                draw_pixel_system
                    .run_in_state(STATE)
                    .after(CartSystem::MoveCart),
            )
            .add_system(move_next_pixel_system.run_in_state(STATE))
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
    },
    OnOutsideCorner {
        pixel: UVec2,
        corner: Corner,
        distance: f32,
    },
    OnInsideCorner {
        pixel: UVec2,
        corner: Corner,
        distance: f32,
    },
    Falling {
        pixel: UVec2,
        side: Side,
        offset: f32,
        velocity: f32,
        finished_drawling: bool,
    },
}

#[derive(Debug, Clone, Copy, Component)]
struct CartPiece;

#[derive(Debug, Component)]
struct NextPixel {
    start_pos: Vec2,
    ideal_pos: Vec2,
    t: f32,
}

#[derive(Debug, Component)]
struct DrawlingPixel {
    start_pos: Vec2,
    grow_dir: Side,
}

#[derive(Debug, Resource)]
struct SetPixels(Grid<bool>);

type WithCartOrPiece = Or<(With<Cart>, With<CartPiece>)>;

type WithChangedCartOnly = (
    Changed<Cart>,
    Without<CartPiece>,
    Without<NextPixel>,
    Without<DrawlingPixel>,
);
type WithCartOnly = (
    With<Cart>,
    Without<CartPiece>,
    Without<NextPixel>,
    Without<DrawlingPixel>,
);
type WithCartPieceOnly = (
    Without<Cart>,
    With<CartPiece>,
    Without<NextPixel>,
    Without<DrawlingPixel>,
);
type WithNextPixelOnly = (
    Without<Cart>,
    Without<CartPiece>,
    With<NextPixel>,
    Without<DrawlingPixel>,
);
type WithDrawlingPixelOnly = (
    Without<Cart>,
    Without<CartPiece>,
    Without<NextPixel>,
    With<DrawlingPixel>,
);

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
        let pos = pixel.world_pos();
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
}

fn move_cart_system(
    mut commands: Commands,
    mut cart_query: Query<&mut Cart>,
    mut ground: ResMut<SetPixels>,
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
    ordering: Res<CurrentOrdering>,
    art: Res<Art>,
) {
    fn checked_add(a: UVec2, b: IVec2) -> Option<UVec2> {
        a.x.checked_add_signed(b.x)
            .zip(a.y.checked_add_signed(b.y))
            .map(Into::into)
    }

    let checked_add_inside =
        |a, b| checked_add(a, b).filter(|pos| pos.x < art.width() && pos.y < art.height());

    let ground_set = |pos, offset| {
        checked_add_inside(pos, offset)
            .map(|pos| ground.0[pos])
            .unwrap_or(false)
    };

    let get_direction = || match (keys.any_pressed(LEFT_KEYS), keys.any_pressed(RIGHT_KEYS)) {
        (true, false) => Some(-1.0),
        (false, true) => Some(1.0),
        _ => None,
    };
    let mut cart = cart_query.single_mut();
    match cart.bypass_change_detection() {
        Cart::OnSide {
            pixel,
            side,
            distance,
        } => {
            if let Some(direction) = get_direction() {
                let old_distance = *distance;
                *distance += direction * time.delta_seconds() * SPEED;
                if crossed(0.0, old_distance, *distance) {
                    if let Some(next_pixel) = ordering.peek() {
                        if Some(next_pixel.pos) == checked_add(*pixel, side.art_direction()) {
                            commands.spawn((
                                SpriteBundle {
                                    sprite: Sprite {
                                        color: next_pixel.color.into(),
                                        // Fixes appearing in corner for 1 frame
                                        custom_size: Some(Vec2::ZERO),
                                        ..default()
                                    },
                                    ..default()
                                },
                                DrawlingPixel {
                                    start_pos: (world_pos(*pixel)
                                        + side.rotate_world_direction(Vec2::new(0.0, 0.5))),
                                    grow_dir: *side,
                                },
                            ));
                            *cart = Cart::Falling {
                                pixel: *pixel,
                                side: *side,
                                offset: 0.0,
                                velocity: 0.0,
                                finished_drawling: false,
                            };
                            return;
                        }
                    }
                    if *distance < 0.0 {
                        if ground_set(
                            *pixel,
                            side.art_direction() + side.rotate_left().art_direction(),
                        ) {
                            *cart = Cart::OnInsideCorner {
                                pixel: checked_add(*pixel, side.art_direction()).unwrap(),
                                corner: side.flip().rotate_right_corner(),
                                distance: -*distance,
                            };
                            return;
                        }
                    } else if ground_set(
                        *pixel,
                        side.art_direction() + side.rotate_right().art_direction(),
                    ) {
                        *cart = Cart::OnInsideCorner {
                            pixel: checked_add(*pixel, side.art_direction()).unwrap(),
                            corner: side.flip().rotate_left_corner(),
                            distance: 1.0 - CART_HEIGHT - *distance,
                        };
                        return;
                    }
                }
                if *distance < -CART_HEIGHT {
                    let dir = side.rotate_left().art_direction();
                    if ground_set(*pixel, dir) {
                        if *distance < -0.5 {
                            *pixel = checked_add(*pixel, dir).unwrap();
                            *distance += 1.0;
                        }
                    } else {
                        *cart = Cart::OnOutsideCorner {
                            pixel: *pixel,
                            corner: side.rotate_left_corner(),
                            distance: *distance + 1.0,
                        };
                    }
                } else if *distance > CART_HEIGHT {
                    let dir = side.rotate_right().art_direction();
                    if ground_set(*pixel, dir) {
                        if *distance > 0.5 {
                            *pixel = checked_add(*pixel, dir).unwrap();
                            *distance -= 1.0;
                        }
                    } else {
                        *cart = Cart::OnOutsideCorner {
                            pixel: *pixel,
                            corner: side.rotate_right_corner(),
                            distance: *distance - CART_HEIGHT,
                        };
                    }
                }
                cart.set_changed();
            }
        }
        Cart::OnOutsideCorner {
            pixel,
            corner,
            distance,
        } => {
            if let Some(direction) = get_direction() {
                *distance += direction * time.delta_seconds() * SPEED;
                if *distance < 0.0 {
                    *cart = Cart::OnSide {
                        pixel: *pixel,
                        side: corner.rotate_left_side(),
                        distance: CART_HEIGHT + *distance,
                    };
                } else if *distance > 1.0 - CART_HEIGHT {
                    *cart = Cart::OnSide {
                        pixel: *pixel,
                        side: corner.rotate_right_side(),
                        distance: *distance - 1.0,
                    };
                }
                cart.set_changed();
            }
        }
        Cart::OnInsideCorner {
            pixel,
            corner,
            distance,
        } => {
            if let Some(direction) = get_direction() {
                *distance += (-direction) * time.delta_seconds() * SPEED;
                if *distance < 0.0 {
                    if ground_set(
                        *pixel,
                        corner.rotate_left_side().rotate_left().art_direction(),
                    ) {
                        *corner = corner.rotate_left();
                        *distance += 1.0 - CART_HEIGHT;
                    } else {
                        let side = corner.rotate_left_side();
                        *cart = Cart::OnSide {
                            pixel: checked_add(*pixel, side.art_direction()).unwrap(),
                            side: side.flip(),
                            distance: -*distance,
                        };
                    }
                } else if *distance > 1.0 - CART_HEIGHT {
                    if ground_set(
                        *pixel,
                        corner.rotate_right_side().rotate_right().art_direction(),
                    ) {
                        *corner = corner.rotate_right();
                        *distance -= 1.0 - CART_HEIGHT;
                    } else {
                        let side = corner.rotate_right_side();
                        *cart = Cart::OnSide {
                            pixel: checked_add(*pixel, side.art_direction()).unwrap(),
                            side: side.flip(),
                            distance: 1.0 - CART_HEIGHT - *distance,
                        };
                    }
                }
                cart.set_changed();
            }
        }
        Cart::Falling {
            pixel,
            side,
            offset,
            velocity,
            finished_drawling,
        } => {
            *velocity += time.delta_seconds() * FALL_SPEED;
            *offset += *velocity * time.delta_seconds();
            if *offset > 1.0 {
                let direction = side.art_direction();
                *pixel = checked_add_inside(*pixel, direction).unwrap();
                let above_set = ground_set(*pixel, direction);
                if !*finished_drawling {
                    ground.0[*pixel] = true;
                    *finished_drawling = true;
                }
                if !above_set {
                    *cart = Cart::OnSide {
                        pixel: *pixel,
                        side: *side,
                        distance: 0.0,
                    };
                }
            }
            cart.set_changed();
        }
    }
}

fn show_cart_system(
    mut cart_query: Query<(&Cart, &mut Transform, &mut Sprite), WithChangedCartOnly>,
    mut piece_query: Query<(&mut Transform, &mut Sprite), WithCartPieceOnly>,
) {
    let outside_y = (1.0 + CART_HEIGHT) * 0.5;
    let inside_y = (1.0 - CART_HEIGHT) * 0.5;
    let outside_cart_extent = 0.5 + CART_HEIGHT;

    if let Ok((cart, mut cart_transform, mut cart_sprite)) = cart_query.get_single_mut() {
        let (mut piece_transform, mut piece_sprite) = piece_query.single_mut();
        dbg!(&cart);

        let mut on_side = |pixel, side: Side, distance, y| {
            let offset = side.rotate_world_direction(Vec2::new(distance, y));
            let pos = world_pos(pixel) + offset;
            cart_transform.translation.x = pos.x;
            cart_transform.translation.y = pos.y;

            cart_sprite.custom_size = Some(match side {
                Side::Top | Side::Bottom => Vec2::new(1.0, CART_HEIGHT),
                Side::Left | Side::Right => Vec2::new(CART_HEIGHT, 1.0),
            });

            piece_transform.translation = cart_transform.translation;
            piece_sprite.custom_size = cart_sprite.custom_size;
        };

        match *cart {
            Cart::OnSide {
                pixel,
                side,
                distance,
            } => on_side(pixel, side, distance, outside_y),
            Cart::OnOutsideCorner {
                pixel,
                corner,
                distance,
            } => {
                let world_pos = world_pos(pixel);

                let left_side = corner.rotate_left_side();
                let left_width = 1.0 - distance;
                let left_offset = left_side.rotate_world_direction(Vec2::new(
                    outside_cart_extent - left_width * 0.5,
                    outside_y,
                ));
                let left_pos = world_pos + left_offset;

                let right_side = corner.rotate_right_side();
                let right_width = distance + CART_HEIGHT;
                let right_offset = right_side.rotate_world_direction(Vec2::new(
                    -(outside_cart_extent - right_width * 0.5),
                    outside_y,
                ));
                let right_pos = world_pos + right_offset;

                cart_transform.translation.x = left_pos.x;
                cart_transform.translation.y = left_pos.y;
                cart_sprite.custom_size = Some(match left_side {
                    Side::Top | Side::Bottom => Vec2::new(left_width, CART_HEIGHT),
                    Side::Left | Side::Right => Vec2::new(CART_HEIGHT, left_width),
                });

                piece_transform.translation.x = right_pos.x;
                piece_transform.translation.y = right_pos.y;
                piece_sprite.custom_size = Some(match right_side {
                    Side::Top | Side::Bottom => Vec2::new(right_width, CART_HEIGHT),
                    Side::Left | Side::Right => Vec2::new(CART_HEIGHT, right_width),
                });
            }
            Cart::OnInsideCorner {
                pixel,
                corner,
                distance,
            } => {
                let world_pos = world_pos(pixel);

                let left_side = corner.rotate_left_side();
                let left_width = 1.0 - distance;
                let left_offset =
                    left_side.rotate_world_direction(Vec2::new(0.5 - left_width * 0.5, inside_y));
                let left_pos = world_pos + left_offset;

                let right_side = corner.rotate_right_side();
                let right_width = CART_HEIGHT + distance;
                let right_offset = right_side
                    .rotate_world_direction(Vec2::new(-0.5 + right_width * 0.5, inside_y));
                let right_pos = world_pos + right_offset;

                cart_transform.translation.x = left_pos.x;
                cart_transform.translation.y = left_pos.y;
                cart_sprite.custom_size = Some(match left_side {
                    Side::Top | Side::Bottom => Vec2::new(left_width, CART_HEIGHT),
                    Side::Left | Side::Right => Vec2::new(CART_HEIGHT, left_width),
                });

                // piece_sprite.custom_size = Some(Vec2::ZERO);
                piece_transform.translation.x = right_pos.x;
                piece_transform.translation.y = right_pos.y;
                piece_sprite.custom_size = Some(match right_side {
                    Side::Top | Side::Bottom => Vec2::new(right_width, CART_HEIGHT),
                    Side::Left | Side::Right => Vec2::new(CART_HEIGHT, right_width),
                });
            }
            Cart::Falling {
                pixel,
                side,
                offset,
                ..
            } => on_side(pixel, side, 0.0, outside_y + offset),
        }
    }
}

fn draw_pixel_system(
    mut commands: Commands,
    mut pixel_query: Query<
        (&DrawlingPixel, &mut Transform, &mut Sprite, Entity),
        WithDrawlingPixelOnly,
    >,
    cart_query: Query<&Cart, WithCartOnly>,
    mesh_query: Query<&MulticolorMesh>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut ordering: ResMut<CurrentOrdering>,
    mut next_pixel_query: Query<(&mut NextPixel, &mut Sprite, Entity), WithNextPixelOnly>,
) {
    if let Ok((pixel, mut pixel_transform, mut pixel_sprite, entity)) = pixel_query.get_single_mut()
    {
        let cart = cart_query.single();
        let (dist, finished) = if let Cart::Falling {
            offset,
            finished_drawling,
            ..
        } = *cart
        {
            (offset, finished_drawling)
        } else {
            (0.0, true)
        };
        if finished {
            commands.entity(entity).despawn();
            mesh_query
                .single()
                .edit(&mut meshes)
                .add_next_from_ordering(&mut ordering);
            let (mut next_pixel_move, mut next_pixel_sprite, next_pixel_entity) =
                next_pixel_query.single_mut();
            if let Some(next_pixel) = ordering.peek() {
                next_pixel_move.start_pos = next_pixel_move.ideal_pos;
                next_pixel_move.ideal_pos = next_pixel.world_pos();
                next_pixel_move.t = 0.0;
                next_pixel_sprite.color = next_pixel.color.transparent().into();
            } else {
                commands.entity(next_pixel_entity).despawn();
            }
        } else {
            pixel_transform.translation =
                (pixel.start_pos + pixel.grow_dir.world_direction() * dist * 0.5).extend(0.0);
            pixel_sprite.custom_size = Some(match pixel.grow_dir {
                Side::Top | Side::Bottom => Vec2::new(1.0, dist),
                Side::Left | Side::Right => Vec2::new(dist, 1.0),
            });
        }
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
    query: Query<
        Entity,
        Or<(
            With<Cart>,
            With<CartPiece>,
            With<NextPixel>,
            With<DrawlingPixel>,
            With<MulticolorMesh>,
        )>,
    >,
) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
    commands.remove_resource::<SetPixels>();
}

fn crossed<T: PartialOrd>(bound: T, a: T, b: T) -> bool {
    (a <= bound && b >= bound) || (a >= bound && b <= bound)
}
