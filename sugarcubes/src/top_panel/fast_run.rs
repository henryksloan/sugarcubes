use super::{TopPanel, ACCEPT_COLOR, REJECT_COLOR};
use sugarcubes_core::automata::{finite_automaton::FiniteAutomaton, SimulateAutomaton};

impl TopPanel {
    pub(super) fn show_fast_run_input_window(
        &mut self,
        egui_ctx: &egui::CtxRef,
        fa: &FiniteAutomaton,
    ) {
        let (hit_ok, contains_mouse) = self.fast_run_input_window.show(egui_ctx);
        self.contains_mouse |= contains_mouse;

        if hit_ok {
            self.fast_run_input_window.open = false;
            self.fast_run_string = self.fast_run_input_window.input.clone();
            self.fast_run_result = Some(fa.check_input(&self.fast_run_input_window.input));
        }

        if !self.fast_run_input_window.open {
            self.fast_run_input_window.input.clear();
        }
    }

    pub(super) fn show_fast_run_result_window(
        &mut self,
        egui_ctx: &egui::CtxRef,
        fast_run_result: bool,
    ) {
        let mut result_open = true;
        egui::Window::new("Fast Run Result")
            .open(&mut result_open)
            .resizable(false)
            .collapsible(false)
            .show(egui_ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label(format!("Result for string \"{}\": ", self.fast_run_string));
                    if fast_run_result {
                        ui.add(egui::widgets::Label::new("Accepted").text_color(ACCEPT_COLOR));
                    } else {
                        ui.add(egui::widgets::Label::new("Rejected").text_color(REJECT_COLOR));
                    }
                });
            });
        if !result_open {
            self.fast_run_result = None;
        }
    }
}
