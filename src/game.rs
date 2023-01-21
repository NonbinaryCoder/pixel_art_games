use bevy::prelude::*;
use bevy_editor_pls::egui;
use bevy_egui::EguiContext;
use iyes_loopless::prelude::*;

use crate::{input::EXIT_KEYS, GameState};

mod appear_test;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(appear_test::AppearTestPlugin)
            .add_startup_system(startup_system)
            .add_system(exit_game_system.run_if_not(GameState::current_is_menu))
            .add_system(set_colors_system.run_if(GameState::current_is_play));
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GameType {
    #[default]
    AppearTest,
    Cart,
}

#[derive(Debug, Resource)]
pub struct Colors {
    primary_color: Color,
    primary_material: Handle<ColorMaterial>,
    secondary_color: Color,
    secondary_material: Handle<ColorMaterial>,
}

fn startup_system(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    let primary_color = Color::BLACK;
    let secondary_color = Color::WHITE;
    commands.insert_resource(Colors {
        primary_color,
        primary_material: materials.add(ColorMaterial::from(primary_color)),
        secondary_color,
        secondary_material: materials.add(ColorMaterial::from(secondary_color)),
    })
}

fn exit_game_system(mut commands: Commands, keys: Res<Input<KeyCode>>) {
    if keys.any_just_pressed(EXIT_KEYS) {
        commands.insert_resource(NextState(GameState::MainMenu));
    }
}

#[allow(clippy::too_many_arguments)]
fn set_colors_system(
    mut egui_context: ResMut<EguiContext>,
    windows: Res<Windows>,
    mut showed_last: Local<bool>,
    mut clear_color: ResMut<ClearColor>,
    mut clear_color_hex: Local<Option<String>>,
    mut colors: ResMut<Colors>,
    mut primary_hex: Local<Option<String>>,
    mut secondary_hex: Local<Option<String>>,
) {
    *showed_last = windows
        .primary()
        .cursor_position()
        .filter(|p| {
            p.y < match *showed_last {
                true => 320.0,
                false => 20.0,
            }
        })
        .is_some();
    egui::TopBottomPanel::bottom("colors_pannel").show_animated(
        egui_context.ctx_mut(),
        *showed_last,
        |ui| {
            ui.horizontal(|ui| {
                let mut color_edit = |color: &mut Color, hex: &mut Option<String>| {
                    let hex = hex.get_or_insert_with(|| {
                        let mut s = String::new();
                        to_hex_rgb(*color, &mut s);
                        s
                    });
                    let c = color.as_rgba_u32().to_le_bytes();
                    let mut c = [c[0], c[1], c[2]];
                    let mut changed = false;
                    if ui.color_edit_button_srgb(&mut c).changed {
                        changed = true;
                        *color = Color::rgb_u8(c[0], c[1], c[2]);
                        to_hex_rgb(*color, hex);
                    };
                    if ui
                        .add(egui::TextEdit::singleline(hex).desired_width(42.0))
                        .changed
                    {
                        changed = true;
                        if let Ok(new_color) = Color::hex(hex) {
                            *color = new_color;
                        }
                    }
                    changed
                };
                color_edit(&mut clear_color.0, &mut clear_color_hex);
                color_edit(&mut colors.primary_color, &mut primary_hex);
                color_edit(&mut colors.secondary_color, &mut secondary_hex);
            })
        },
    );
}

fn to_hex_rgb(color: Color, buf: &mut String) {
    use std::fmt::Write;

    buf.clear();
    buf.reserve(6);
    let [r, g, b, _] = color.as_rgba_u32().to_le_bytes();
    write!(buf, "{r:02x}{g:02x}{b:02x}").expect("String formatting failed!");
}
