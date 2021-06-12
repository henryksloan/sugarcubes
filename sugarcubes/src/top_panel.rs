use crate::states::*;

use sugarcubes_core::automata::{
    finite_automaton::{FiniteAutomaton, FiniteAutomatonConfiguration, FiniteAutomatonTransition},
    Configuration,
};

use macroquad::prelude::*;

pub enum Command {
    SetInitial(u32),
    RemoveInitial,
    SetFinal(u32, bool),

    DeleteState(u32),
    DeleteTransition(FiniteAutomatonTransition),
}

impl Command {
    pub fn execute(
        &self,
        fa: &mut FiniteAutomaton,
        states: &mut States,
        configurations: &mut Vec<FiniteAutomatonConfiguration>,
    ) {
        match *self {
            Self::SetInitial(state) => fa.automaton.set_initial(state),
            Self::RemoveInitial => fa.automaton.remove_initial(),
            Self::SetFinal(state, value) => fa.automaton.set_final(state, value),

            Self::DeleteState(state) => {
                states.remove_state(fa, state);
                // TODO: This won't be necessary once editing and simulation modes are separated
                configurations.retain(|conf| conf.state() != state);
            }
            Self::DeleteTransition(transition) => fa.automaton.remove_transition(transition),
        }
    }
}

pub struct TopPanel {
    pub should_step: bool,
    pub contains_mouse: bool,
    pub open_context_menu: bool,
    pub context_menu_pos: Vec2,
}

impl TopPanel {
    pub fn new() -> Self {
        Self {
            should_step: false,
            contains_mouse: false,
            open_context_menu: false,
            context_menu_pos: Vec2::ZERO,
        }
    }

    pub fn ui(
        &mut self,
        fa: &FiniteAutomaton,
        states: &States,
        configurations: &Vec<FiniteAutomatonConfiguration>,
        mouse_position: &Vec2,
        selected_state: &mut Option<u32>,
        selected_transition: &mut Option<FiniteAutomatonTransition>,
    ) -> Option<Command> {
        self.should_step = false;
        self.contains_mouse = false;

        let mut command = None;

        egui_macroquad::ui(|egui_ctx| {
            egui::TopPanel::top("Sugarcubes").show(egui_ctx, |ui| {
                egui::menu::bar(ui, |ui| {
                    egui::menu::menu(ui, "File", |ui| {
                        if ui.button("Open").clicked() {
                            // ...
                        }
                    });
                });

                ui.separator();

                // TODO: First of all, refactor all egui stuff into a new file/module folder
                // Then, the toolbar shown should be based on a Match of some global state (possibly in egui `memory`?)
                ui.label("Simulation Toolbar");
                ui.horizontal(|ui| {
                    if ui.button("Step").clicked() {
                        self.should_step = true;
                    }

                    for configuration in configurations {
                        if ui
                            .add_sized(
                                [75., 50.],
                                egui::Button::new(configuration.state().to_string()),
                            )
                            .clicked()
                        {}
                    }
                });

                self.contains_mouse = ui.ui_contains_pointer();
            });

            egui::Area::new("my_area").show(egui_ctx, |ui| {
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
                                                command =
                                                    Some(Command::SetFinal(selected, is_final));
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
        });

        command
    }
}
