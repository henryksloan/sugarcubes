use super::TopPanel;
use crate::States;

impl TopPanel {
    pub(super) fn show_set_name_input_window(
        &mut self,
        egui_ctx: &egui::CtxRef,
        states: &mut States,
    ) {
        let (hit_ok, contains_mouse) = self.set_name_input_window.show(egui_ctx);
        self.contains_mouse |= contains_mouse;

        if hit_ok {
            self.set_name_input_window.open = false;
            if let Some(set_name_state_id) = self.set_name_state_id {
                let new_name = if self.set_name_input_window.input.is_empty() {
                    States::default_name(set_name_state_id)
                } else {
                    self.set_name_input_window.input.clone()
                };
                states.insert_name(set_name_state_id, new_name);
            }
        }

        if !self.set_name_input_window.open {
            self.set_name_input_window.input.clear();
            self.set_name_state_id = None;
        }
    }
}
