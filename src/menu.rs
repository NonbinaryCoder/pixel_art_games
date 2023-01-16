use bevy::prelude::*;
use bevy_egui::{
    egui::{self, RichText, TextStyle},
    EguiContext, EguiPlugin,
};
use iyes_loopless::prelude::*;

use crate::{
    art::Art,
    game::GameType,
    ordering::{CurrentOrdering, OrderingType, Orderings},
    GameState,
};

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(EguiPlugin)
            .add_system(show_menu_system.run_in_state(GameState::MainMenu));
    }
}

#[allow(clippy::too_many_arguments)]
pub fn show_menu_system(
    mut commands: Commands,
    mut egui_context: ResMut<EguiContext>,
    windows: Res<Windows>,
    mut ordering: Local<OrderingType>,
    mut is_reversed: Local<bool>,
    mut is_by_color: Local<bool>,
    mut game: Local<GameType>,
    mut orderings: ResMut<Orderings>,
    art: Res<Art>,
) {
    let window_width = windows.get_primary().map(Window::width).unwrap_or(200.0);
    let set_style = |ui: &mut egui::Ui| {
        let style = ui.style_mut();
        style.text_styles.get_mut(&TextStyle::Heading).unwrap().size = 80.0;
        style.text_styles.get_mut(&TextStyle::Button).unwrap().size = 30.0;
    };

    let mut set_ordering = *ordering;
    egui::SidePanel::left("ordering")
        .min_width(window_width / 2.0 - 15.0)
        .resizable(false)
        .show(egui_context.ctx_mut(), |ui| {
            set_style(ui);

            ui.vertical_centered(|ui| {
                ui.heading("Ordering");

                let mut show_option = |new_ordering, label: &str| {
                    if ui
                        .selectable_label(*ordering == new_ordering, label)
                        .clicked()
                    {
                        set_ordering = new_ordering;
                    };
                };
                show_option(OrderingType::Default, "Default");
                show_option(OrderingType::SideToSide, "Side-to-Side");
                show_option(OrderingType::Spiral, "Spiral");

                ui.add_space(30.0);

                if ui
                    .button(if *is_reversed {
                        "Reversed"
                    } else {
                        "Not Reversed"
                    })
                    .clicked()
                {
                    *is_reversed = !*is_reversed
                }

                if ui
                    .button(if *is_by_color {
                        "By Color"
                    } else {
                        "One Shape"
                    })
                    .clicked()
                {
                    *is_by_color = !*is_by_color
                }
            });

            egui::TopBottomPanel::bottom("ordering_bottom")
                .resizable(false)
                .show_separator_line(false)
                .show_inside(ui, |ui| {
                    ui.vertical_centered(|ui| {
                        if ui.button(RichText::new("Generate!").size(40.0)).clicked() {
                            // TODO
                        }
                    });
                });
        });
    *ordering = set_ordering;

    let mut set_game = *game;
    egui::SidePanel::right("game")
        .min_width(window_width / 2.0 - 15.0)
        .resizable(false)
        .show_separator_line(false)
        .show(egui_context.ctx_mut(), |ui| {
            set_style(ui);

            ui.vertical_centered(|ui| {
                ui.heading("Game");

                let mut show_option = |new_game, label: &str| {
                    if ui.selectable_label(*game == new_game, label).clicked() {
                        set_game = new_game;
                    };
                };
                show_option(GameType::AppearTest, "Appear Test");
                show_option(GameType::Cart, "Cart");
            });

            egui::TopBottomPanel::bottom("game_bottom")
                .resizable(false)
                .show_separator_line(false)
                .show_inside(ui, |ui| {
                    ui.vertical_centered(|ui| {
                        if ui.button(RichText::new("Play!").size(40.0)).clicked() {
                            commands.insert_resource(CurrentOrdering::init(
                                &mut orderings,
                                *ordering,
                                &art,
                            ));
                            commands.insert_resource(NextState(GameState::Play(*game)));
                        }
                    });
                });
        });
    *game = set_game;
}
