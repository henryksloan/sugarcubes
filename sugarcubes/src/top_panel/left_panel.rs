use super::{Mode, TopPanel, TOP_PANEL};
use crate::{FiniteAutomaton, SimulateAutomaton};

use sapp_jsutils::JsObject;

#[cfg(target_arch = "wasm32")]
extern "C" {
    fn choose_multiple_run_file();
}

#[cfg(not(target_arch = "wasm32"))]
unsafe fn choose_multiple_run_file() {}

#[no_mangle]
extern "C" fn read_multiple_run_inputs(content: JsObject) {
    let mut content_string = String::new();
    content.to_string(&mut content_string);
    TOP_PANEL.with(|panel| {
        if let Ok(mut panel) = panel.try_borrow_mut() {
            panel.load_multiple_run_inputs(content_string)
        }
    });
}

impl TopPanel {
    pub(super) fn left_panel(&mut self, egui_ctx: &egui::CtxRef, fa: &FiniteAutomaton) {
        egui::SidePanel::left("multiple_run")
            .resizable(false)
            .show(egui_ctx, |ui| {
                // Title bar and close button
                ui.horizontal(|ui| {
                    ui.with_layout(egui::Layout::left_to_right(), |ui| {
                        ui.heading("Multiple Run");
                    });
                    ui.with_layout(egui::Layout::right_to_left(), |ui| {
                        if ui.button("X").clicked() {
                            self.mode = Mode::Edit;
                        }
                    });
                });

                ui.separator();

                // Input list and buttons
                egui::ScrollArea::auto_sized().show(ui, |ui| {
                    let (new_selected_index, add_new_line) = self.input_list(ui);
                    self.multiple_run_selected_index = new_selected_index;

                    // "Enter" was pressed on the last TextEdit, and it was empty
                    if add_new_line {
                        self.multiple_run_strings.push((String::new(), None));
                    }

                    ui.horizontal(|ui| {
                        if ui.button("Run").clicked() {
                            for (text, status) in self.multiple_run_strings.iter_mut() {
                                *status = Some(fa.check_input(text));
                            }

                            // If the last string is empty, discard the result,
                            // as it is most likely just an extra blank line, not a user's query
                            if let Some(mut last_string) = self.multiple_run_strings.last_mut() {
                                if last_string.0.is_empty() {
                                    last_string.1 = None;
                                }
                            }
                        }

                        if ui.button("Load from file").clicked() {
                            unsafe {
                                choose_multiple_run_file();
                            }
                        }

                        if ui.button("Clear").clicked() {
                            self.multiple_run_strings = vec![(String::new(), None)];
                        }
                    });
                });

                self.width = ui.max_rect().width();
            });
    }

    // Display all of the string boxes and current results, returning
    // the focused index, if any, and whether a new line should be added
    fn input_list(&mut self, ui: &mut egui::Ui) -> (Option<usize>, bool) {
        let selected_index = self.multiple_run_selected_index;
        let mut add_new_line = false;
        let mut new_selected_index = selected_index;
        let num_strings = self.multiple_run_strings.len();
        for (i, (text, status)) in self.multiple_run_strings.iter_mut().enumerate() {
            ui.horizontal(|ui| {
                let text_edit = ui.add(egui::TextEdit::singleline(text));

                if text_edit.lost_focus() {
                    if ui.input().key_pressed(egui::Key::Enter) {
                        if i == num_strings - 1 && !text.is_empty() {
                            add_new_line = true;
                            new_selected_index = Some(i + 1);
                        } else {
                            new_selected_index = Some((i + 1) % num_strings);
                        }
                    } else {
                        new_selected_index = None;
                    }
                }

                if let Some(selected_index) = selected_index {
                    // If this index is selected, and hasn't been clicked off of
                    if selected_index == i && new_selected_index.is_some() {
                        text_edit.request_focus();
                    }
                }

                let label = match status {
                    None => "⛶",
                    Some(false) => "🗙",
                    Some(true) => "✔",
                };
                ui.add(egui::Label::new(label));
            });
        }
        (new_selected_index, add_new_line)
    }
}
