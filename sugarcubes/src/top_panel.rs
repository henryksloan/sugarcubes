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

pub enum TopPanelCommand {
    Command(Command),
    Undo,
    Redo,
    Step,
    StartSimulation(Vec<FiniteAutomatonConfiguration>),
}

pub struct TopPanel {
    pub height: f32,
    pub mode: Mode,
    pub contains_mouse: bool,
    pub open_context_menu: bool,
    pub context_menu_pos: Vec2,

    open_string_input_window: bool,
    string_input: String,
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
            string_simulating: String::new(),
        }
    }

    pub fn ui(
        &mut self,
        fa: &FiniteAutomaton,
        states: &mut States,
        configurations: &mut Vec<FiniteAutomatonConfiguration>,
        mouse_position: &Vec2,
        selected_state: &mut Option<u32>,
        selected_transition: &mut Option<FiniteAutomatonTransition>,
        can_undo: bool,
        can_redo: bool,
    ) -> Option<TopPanelCommand> {
        self.contains_mouse = false;

        let mut command = None;

        egui_macroquad::ui(|egui_ctx| {
            let mut fonts = egui::FontDefinitions::default();
            fonts.family_and_size.insert(
                egui::TextStyle::Button,
                (egui::FontFamily::Proportional, 18.),
            );
            fonts
                .family_and_size
                .insert(egui::TextStyle::Body, (egui::FontFamily::Proportional, 19.));
            egui_ctx.set_fonts(fonts);

            egui::TopBottomPanel::top("top_panel").show(egui_ctx, |ui| {
                let menu_bar_command = self.menu_bar(ui, can_undo, can_redo);

                if let Some(menu_bar_command) = menu_bar_command {
                    command = Some(menu_bar_command);
                }

                if let Mode::Simulate = self.mode {
                    ui.separator();
                    let simulation_toolbar_command =
                        self.simulation_toolbar(ui, fa, configurations);

                    if let Some(simulation_toolbar_command) = simulation_toolbar_command {
                        command = Some(simulation_toolbar_command);
                    }
                }

                self.contains_mouse = ui.ui_contains_pointer();
                self.height = ui.max_rect().height();
            });

            let context_menu_command = self.context_menu(
                egui_ctx,
                fa,
                states,
                mouse_position,
                selected_state,
                selected_transition,
            );

            if let Some(context_menu_command) = context_menu_command {
                command = Some(TopPanelCommand::Command(context_menu_command));
            }

            if self.open_string_input_window {
                let new_configurations = self.string_input_window(egui_ctx, fa);
                if let Some(new_configurations) = new_configurations {
                    command = Some(TopPanelCommand::StartSimulation(new_configurations));
                }
            }
        });

        command
    }

    fn menu_bar(
        &mut self,
        ui: &mut egui::Ui,
        can_undo: bool,
        can_redo: bool,
    ) -> Option<TopPanelCommand> {
        let mut command = None;

        egui::menu::bar(ui, |ui| {
            egui::menu::menu(ui, "File", |ui| {
                if ui.button("Open").clicked() {
                    // ...
                }
            });

            egui::menu::menu(ui, "Edit", |ui| {
                if ui
                    .add(egui::widgets::Button::new("Undo").enabled(can_undo))
                    .clicked()
                {
                    command = Some(TopPanelCommand::Undo);
                }

                if ui
                    .add(egui::widgets::Button::new("Redo").enabled(can_redo))
                    .clicked()
                {
                    command = Some(TopPanelCommand::Redo);
                }
            });

            egui::menu::menu(ui, "Simulate", |ui| {
                if ui.button("Simulate String").clicked() {
                    self.open_string_input_window = true;
                }
            });
        });

        command
    }

    fn simulation_toolbar(
        &mut self,
        ui: &mut egui::Ui,
        fa: &FiniteAutomaton,
        configurations: &mut Vec<FiniteAutomatonConfiguration>,
    ) -> Option<TopPanelCommand> {
        let mut command = None;

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

                        let mut button = egui::Button::new(format!(
                            "{}\n{}",
                            configuration.state().to_string(),
                            message,
                        ))
                        .text_color(text_color)
                        .text_style(egui::TextStyle::Heading);

                        if let Some(fill) = fill {
                            button = button.fill(fill);
                        }

                        if ui.add_sized([75., CONFIGURATION_HEIGHT], button).clicked() {}
                    }
                });

                ui.separator();

                ui.horizontal(|ui| {
                    if ui.button("Step").clicked() {
                        command = Some(TopPanelCommand::Step);
                    }

                    if ui.button("Reset").clicked() {
                        command = Some(TopPanelCommand::StartSimulation(
                            fa.initial_configurations(&self.string_simulating),
                        ))
                    }
                });
            });
        });

        command
    }

    fn context_menu(
        &mut self,
        egui_ctx: &egui::CtxRef,
        fa: &FiniteAutomaton,
        states: &mut States,
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
                                                command = Some(Command::SetInitial(
                                                    selected,
                                                    fa.automaton.initial(),
                                                ));
                                            } else {
                                                command = Some(Command::RemoveInitial(selected));
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
                                            let transitions = {
                                                let transitions_from = fa
                                                    .automaton
                                                    .transitions_from(selected)
                                                    .into_iter()
                                                    .cloned();
                                                let transitions_to = fa
                                                    .automaton
                                                    .transitions_to(selected)
                                                    .into_iter()
                                                    .cloned();
                                                transitions_from.chain(transitions_to).collect()
                                            };
                                            command = Some(Command::DeleteState(
                                                selected,
                                                states.get_position(selected).clone(),
                                                transitions,
                                            ));
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

    fn string_input_window(
        &mut self,
        egui_ctx: &egui::CtxRef,
        fa: &FiniteAutomaton,
    ) -> Option<Vec<FiniteAutomatonConfiguration>> {
        let mut new_configurations = None;

        let mut window_open = true;
        let response = egui::Window::new("Input string")
            .open(&mut window_open)
            .resizable(false)
            .collapsible(false)
            .title_bar(true)
            .show(egui_ctx, |ui| {
                // TODO: Checking for lost_focus AND enter pressed no longer works; fix that
                //     || (text_edit.lost_focus() && ui.input().key_pressed(egui::Key::Enter))
                let text_edit = ui.add(egui::TextEdit::singleline(&mut self.string_input));
                text_edit.request_focus();

                ui.horizontal(|ui| {
                    if ui.button("Ok").clicked() || ui.input().key_pressed(egui::Key::Enter) {
                        new_configurations = Some(fa.initial_configurations(&self.string_input));
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

        new_configurations
    }
}
