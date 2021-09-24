use super::{Command, TopPanel};
use crate::states::States;
use sugarcubes_core::automata::finite_automaton::{FiniteAutomaton, FiniteAutomatonTransition};

use macroquad::prelude::*;

impl TopPanel {
    pub(super) fn context_menu(
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
            if ui.memory().is_popup_open(popup_id) {
                let (context_menu_command, mouse_in_popup) = self.show_context_menu(
                    ui,
                    popup_id,
                    fa,
                    states,
                    selected_state,
                    selected_transition,
                );

                if let Some(context_menu_command) = context_menu_command {
                    command = Some(context_menu_command);
                }

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

    // Returns an optional command,
    // and whether the mouse is in the content menu popup
    fn show_context_menu(
        &mut self,
        ui: &mut egui::Ui,
        popup_id: egui::Id,
        fa: &FiniteAutomaton,
        states: &mut States,
        selected_state: &mut Option<u32>,
        selected_transition: &mut Option<FiniteAutomatonTransition>,
    ) -> (Option<Command>, bool) {
        let mut command = None;
        let mut mouse_in_popup = false;
        let parent_clip_rect = ui.clip_rect();

        egui::Area::new(popup_id)
            .order(egui::Order::Foreground)
            .fixed_pos((self.context_menu_pos.x, self.context_menu_pos.y))
            .show(ui.ctx(), |ui| {
                ui.set_clip_rect(parent_clip_rect);
                let frame = egui::Frame::popup(ui.style());
                let frame_margin = frame.margin;
                frame.show(ui, |ui| {
                    ui.with_layout(egui::Layout::top_down_justified(egui::Align::LEFT), |ui| {
                        ui.set_width(100.0 - 2.0 * frame_margin.x);
                        if let Some(selected) = *selected_state {
                            let mut is_initial = fa.automaton.initial() == Some(selected);
                            if ui.checkbox(&mut is_initial, "Initial").changed() {
                                if is_initial {
                                    command =
                                        Some(Command::SetInitial(selected, fa.automaton.initial()));
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

                            if ui.button("Set Name").clicked() {
                                self.set_name_input_window.open = true;
                                self.set_name_input_window.input = states.get_name(selected);
                                self.set_name_state_id = Some(selected);
                                *selected_state = None;
                                ui.memory().close_popup();
                            }

                            ui.separator();

                            if ui.button("Delete").clicked() {
                                command = Some(Command::DeleteState(
                                    selected,
                                    *states.get_position(selected),
                                    fa.automaton
                                        .transitions_with(selected)
                                        .into_iter()
                                        .cloned()
                                        .collect(),
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
                    });
                });
            });
        (command, mouse_in_popup)
    }
}
