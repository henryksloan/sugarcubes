use super::{Mode, TopPanel, TopPanelCommand};

impl TopPanel {
    pub(super) fn menu_bar(
        &mut self,
        ui: &mut egui::Ui,
        can_undo: bool,
        can_redo: bool,
    ) -> Option<TopPanelCommand> {
        let mut command = None;

        egui::menu::bar(ui, |ui| {
            self.file_menu(ui);

            let edit_menu_command = self.edit_menu(ui, can_undo, can_redo);
            if let Some(edit_menu_command) = edit_menu_command {
                command = Some(edit_menu_command);
            }

            self.simulate_menu(ui);
        });

        command
    }

    fn file_menu(&mut self, ui: &mut egui::Ui) {
        egui::menu::menu(ui, "File", |ui| {
            if ui.button("Open...").clicked() {
                // ...
            }
        });
    }

    fn edit_menu(
        &mut self,
        ui: &mut egui::Ui,
        can_undo: bool,
        can_redo: bool,
    ) -> Option<TopPanelCommand> {
        let mut command = None;
        egui::menu::menu(ui, "Edit", |ui| {
            let undo_button = egui::widgets::Button::new("Undo").enabled(can_undo);
            if ui.add(undo_button).clicked() {
                command = Some(TopPanelCommand::Undo);
            }

            let redo_button = egui::widgets::Button::new("Redo").enabled(can_redo);
            if ui.add(redo_button).clicked() {
                command = Some(TopPanelCommand::Redo);
            }
        });
        command
    }

    fn simulate_menu(&mut self, ui: &mut egui::Ui) {
        egui::menu::menu(ui, "Simulate", |ui| {
            if ui.button("Simulate String...").clicked() {
                self.simulate_input_window.open = true;
            }

            if ui.button("Fast Run...").clicked() {
                self.fast_run_input_window.open = true;
            }

            if ui.button("Multiple Run...").clicked() {
                for pair in self.multiple_run_strings.iter_mut() {
                    pair.1 = None;
                }
                self.mode = Mode::MultipleRun;
                self.multiple_run_selected_index = None;
            }
        });
    }
}
