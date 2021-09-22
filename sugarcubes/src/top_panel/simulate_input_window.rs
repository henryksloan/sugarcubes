use super::{Mode, TopPanel, TopPanelCommand};
use sugarcubes_core::automata::{finite_automaton::FiniteAutomaton, SimulateAutomaton};

impl TopPanel {
    pub(super) fn show_simulate_input_window(
        &mut self,
        egui_ctx: &egui::CtxRef,
        fa: &FiniteAutomaton,
    ) -> Option<TopPanelCommand> {
        let mut command = None;

        let mut new_configurations = None;
        let (hit_ok, contains_mouse) = self.simulate_input_window.show(egui_ctx);
        self.contains_mouse |= contains_mouse;

        if hit_ok {
            new_configurations = Some(fa.initial_configurations(&self.simulate_input_window.input));
            self.mode = Mode::Simulate;
            self.string_simulating = self.simulate_input_window.input.clone();
            self.simulate_input_window.open = false;
        }

        if let Some(new_configurations) = new_configurations {
            command = Some(TopPanelCommand::StartSimulation(new_configurations));
        }

        if !self.simulate_input_window.open {
            self.simulate_input_window.input.clear();
        }

        command
    }
}
