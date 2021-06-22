use crate::{command::*, states::*};

use sugarcubes_core::automata::{
    finite_automaton::{FiniteAutomaton, FiniteAutomatonConfiguration, FiniteAutomatonTransition},
    Configuration, SimulateAutomaton,
};

use macroquad::prelude::*;

const CONFIGURATION_HEIGHT: f32 = 60.;

pub enum Mode {
    Edit,
    Simulate,
}

pub struct TopPanel {
    pub height: f32,
    pub mode: Mode,
    pub contains_mouse: bool,
    pub open_context_menu: bool,
    pub context_menu_pos: Vec2,

    open_string_input_window: bool,
    string_input: String,
    pub new_configurations: Option<Vec<FiniteAutomatonConfiguration>>,

    pub should_step: bool,
    string_simulating: String,
}

impl TopPanel {
    pub fn new() -> Self {
        Self {
            height: 0.,
            mode: Mode::Edit,
            contains_mouse: false,
            open_context_menu: false,
            context_menu_pos: Vec2::ZERO,

            open_string_input_window: false,
            string_input: String::new(),
            new_configurations: None,

            should_step: false,
            string_simulating: String::new(),
        }
    }

    pub fn ui(
        &mut self,
        fa: &FiniteAutomaton,
        states: &States,
        configurations: &mut Vec<FiniteAutomatonConfiguration>,
        mouse_position: &Vec2,
        selected_state: &mut Option<u32>,
        selected_transition: &mut Option<FiniteAutomatonTransition>,
    ) -> Option<Command> {
        self.should_step = false;
        self.contains_mouse = false;
        self.new_configurations = None;

        let mut command = None;

        egui_macroquad::ui(|egui_ctx| {
            egui::TopPanel::top("top_panel").show(egui_ctx, |ui| {
                self.menu_bar(ui);

                if let Mode::Simulate = self.mode {
                    ui.separator();
                    self.simulation_toolbar(ui, fa, configurations);
                }

                self.contains_mouse = ui.ui_contains_pointer();
                self.height = ui.max_rect().height();
            });

            command = self.context_menu(
                egui_ctx,
                fa,
                states,
                mouse_position,
                selected_state,
                selected_transition,
            );

            if self.open_string_input_window {
                self.string_input_window(egui_ctx, fa);
            }
        });

        command
    }

    fn menu_bar(&mut self, ui: &mut egui::Ui) {
        egui::menu::bar(ui, |ui| {
            egui::menu::menu(ui, "File", |ui| {
                if ui.button("Open").clicked() {
                    // ...
                }
            });

            egui::menu::menu(ui, "Simulate", |ui| {
                if ui.button("Simulate String").clicked() {
                    self.open_string_input_window = true;
                }
            });
        });
    }

    fn simulation_toolbar(
        &mut self,
        ui: &mut egui::Ui,
        fa: &FiniteAutomaton,
        configurations: &mut Vec<FiniteAutomatonConfiguration>,
    ) {
        ui.horizontal(|ui| {
            if ui.button("X").clicked() {
                self.mode = Mode::Edit;
            }

            ui.vertical(|ui| {
                ui.add(
                    egui::Label::new(format!("Simulating \"{}\"", self.string_simulating))
                        .heading(),
                );

                ui.separator();

                ui.horizontal(|ui| {
                    ui.set_min_height(CONFIGURATION_HEIGHT);
                    for configuration in configurations {
                        let (fill, text_color, message) =
                            if configuration.remaining_string.is_empty() {
                                if fa.automaton.is_final(configuration.state()) {
                                    (
                                        Some(egui::Color32::from_rgb(122, 240, 98)),
                                        egui::Color32::BLACK,
                                        "accept",
                                    )
                                } else {
                                    (Some(egui::Color32::RED), egui::Color32::WHITE, "reject")
                                }
                            } else {
                                (
                                    None,
                                    egui::Color32::WHITE,
                                    configuration.remaining_string.as_str(),
                                )
                            };

                        if ui
                            .add_sized(
                                [75., CONFIGURATION_HEIGHT],
                                egui::Button::new(format!(
                                    "{}\n{}",
                                    configuration.state().to_string(),
                                    message,
                                ))
                                .fill(fill)
                                .text_color(text_color)
                                .text_style(egui::TextStyle::Heading),
                            )
                            .clicked()
                        {}
                    }
                });

                ui.separator();

                ui.horizontal(|ui| {
                    if ui.button("Step").clicked() {
                        self.should_step = true;
                    }

                    if ui.button("Reset").clicked() {
                        self.new_configurations =
                            Some(fa.initial_configurations(&self.string_simulating));
                    }
                });
            });
        });
    }

    fn context_menu(
        &mut self,
        egui_ctx: &egui::CtxRef,
        fa: &FiniteAutomaton,
        states: &States,
        mouse_position: &Vec2,
        selected_state: &mut Option<u32>,
        selected_transition: &mut Option<FiniteAutomatonTransition>,
    ) -> Option<Command> {
        let mut command = None;

        egui::Area::new("context_menu").show(egui_ctx, |ui| {
            let popup_id = ui.make_persistent_id("context_menu_id");
            if self.open_context_menu {
                ui.memory().open_popup(popup_id);
                self.open_context_menu = false;
            }
            let mut mouse_in_popup = false;
            if ui.memory().is_popup_open(popup_id) {
                let parent_clip_rect = ui.clip_rect();

                egui::Area::new(popup_id)
                    .order(egui::Order::Foreground)
                    .fixed_pos((self.context_menu_pos.x, self.context_menu_pos.y))
                    .show(ui.ctx(), |ui| {
                        ui.set_clip_rect(parent_clip_rect);
                        let frame = egui::Frame::popup(ui.style());
                        let frame_margin = frame.margin;
                        frame.show(ui, |ui| {
                            ui.with_layout(
                                egui::Layout::top_down_justified(egui::Align::LEFT),
                                |ui| {
                                    ui.set_width(100.0 - 2.0 * frame_margin.x);
                                    if let Some(selected) = *selected_state {
                                        let mut is_initial =
                                            fa.automaton.initial() == Some(selected);
                                        if ui.checkbox(&mut is_initial, "Initial").changed() {
                                            if is_initial {
                                                command = Some(Command::SetInitial(selected));
                                            } else {
                                                command = Some(Command::RemoveInitial);
                                            }
                                            *selected_state = None;
                                            ui.memory().close_popup();
                                        }

                                        let mut is_final = fa.automaton.is_final(selected);
                                        if ui.checkbox(&mut is_final, "Final").changed() {
                                            command = Some(Command::SetFinal(selected, is_final));
                                            *selected_state = None;
                                            ui.memory().close_popup();
                                        }

                                        ui.separator();

                                        if ui.button("Delete").clicked() {
                                            command = Some(Command::DeleteState(selected));
                                            *selected_state = None;
                                            ui.memory().close_popup();
                                        }
                                    } else if let Some(selected) = *selected_transition {
                                        if ui.button("Delete").clicked() {
                                            command = Some(Command::DeleteTransition(selected));
                                            *selected_transition = None;
                                            ui.memory().close_popup();
                                        }
                                    }

                                    mouse_in_popup = ui.ui_contains_pointer();
                                    self.contains_mouse |= mouse_in_popup;
                                },
                            );
                        });
                    });

                if ui.input().key_pressed(egui::Key::Escape) {
                    ui.memory().close_popup();
                    *selected_state = None;
                } else if is_mouse_button_pressed(MouseButton::Left) && !mouse_in_popup {
                    ui.memory().close_popup();

                    // Clear selected state if the cancelling click is not in the selected state
                    if let Some(selected) = *selected_state {
                        if !states.point_in_state(*mouse_position, selected) {
                            *selected_state = None;
                        }
                    }
                }

                self.contains_mouse |= ui.ui_contains_pointer();
            }
        });

        command
    }

    fn string_input_window(&mut self, egui_ctx: &egui::CtxRef, fa: &FiniteAutomaton) {
        let mut window_open = true;
        let response = egui::Window::new("Input string")
            .open(&mut window_open)
            .resizable(false)
            .collapsible(false)
            .title_bar(true)
            .show(egui_ctx, |ui| {
                let text_edit = ui.add(egui::TextEdit::singleline(&mut self.string_input));

                ui.horizontal(|ui| {
                    if ui.button("Ok").clicked()
                        || (text_edit.lost_focus() && ui.input().key_pressed(egui::Key::Enter))
                    {
                        self.new_configurations =
                            Some(fa.initial_configurations(&self.string_input));
                        self.mode = Mode::Simulate;
                        self.string_simulating = self.string_input.clone();
                        self.open_string_input_window = false;
                    }

                    if ui.button("Cancel").clicked() {
                        self.open_string_input_window = false;
                    }
                });
            });
        if !window_open {
            self.open_string_input_window = false;
        }

        if !self.open_string_input_window {
            self.string_input.clear();
        }

        if let Some(response) = response {
            self.contains_mouse |= response.hovered();
        }
    }
}
