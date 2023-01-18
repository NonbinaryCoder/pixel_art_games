use bevy::prelude::*;
use bevy_egui::{
    egui::{self, RichText, TextStyle},
    EguiContext, EguiPlugin,
};
use iyes_loopless::prelude::*;

use crate::{
    art::{Art, ArtName},
    camera::AreaTrackingProjection,
    game::GameType,
    ordering::{CurrentOrdering, OrderingType, Orderings},
    GameState,
};

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SystemLabel)]
        struct Label;

        app.add_plugin(EguiPlugin)
            .add_system(
                show_menu_system
                    .run_in_state(GameState::MainMenu)
                    .after(Label),
            )
            .add_system(awaiting_image_system.label(Label));
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
    mut projection_query: Query<&mut AreaTrackingProjection>,
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
        .max_width(window_width / 2.0)
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
        .min_width(window_width / 2.0)
        .max_width(window_width / 2.0)
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

                            projection_query.single_mut().tracked_area = Rect {
                                min: Vec2::new(-1.0, -1.0),
                                max: art.size().as_vec2(),
                            };

                            commands.insert_resource(NextState(GameState::Play(*game)));
                        }
                    });
                });
        });
    *game = set_game;
}

fn awaiting_image_system(
    mut commands: Commands,
    state: Res<CurrentState<GameState>>,
    mut egui_context: ResMut<EguiContext>,
    art_name: Option<Res<ArtName>>,
    mut file_events: EventReader<FileDragAndDrop>,
) {
    if let Some(art_name) = art_name {
        art_name.show(egui_context.ctx_mut())
    }

    if state.0 == GameState::AwaitingImage {
        egui::CentralPanel::default().show(egui_context.ctx_mut(), |ui| {
            ui.centered_and_justified(|ui| {
                ui.heading(RichText::new("Drag image here to begin").size(100.0));
            });
        });
    }

    for file_event in file_events.iter() {
        if let FileDragAndDrop::DroppedFile { path_buf, .. } = file_event {
            match Art::load_from_path(path_buf) {
                Ok(art) => {
                    commands.insert_resource(art);
                    commands.insert_resource(ArtName(path_buf.file_name().map_or_else(
                        || "{unknown}".to_owned(),
                        |name| name.to_string_lossy().to_string(),
                    )));
                    commands.insert_resource(NextState(GameState::MainMenu));
                }
                Err(err) => {
                    commands.insert_resource(ArtName(err));
                }
            }
        }
    }
}
