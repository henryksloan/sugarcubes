use super::{Mode, TopPanel, TopPanelCommand};
use sugarcubes_core::automata::{
    finite_automaton::{FiniteAutomaton, FiniteAutomatonConfiguration},
    Configuration, SimulateAutomaton, EMPTY_STRING,
};

const CONFIGURATION_HEIGHT: f32 = 60.;

impl TopPanel {
    pub(super) fn simulation_toolbar(
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
                    self.list_configurations(ui, fa, configurations);
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

    fn list_configurations(
        &mut self,
        ui: &mut egui::Ui,
        fa: &FiniteAutomaton,
        configurations: &mut Vec<FiniteAutomatonConfiguration>,
    ) {
        for configuration in configurations {
            let config_exhausted = configuration.remaining_string.is_empty()
                && !fa
                    .automaton
                    .transitions_from(configuration.state())
                    .into_iter()
                    .any(|&transition| transition.symbol() == EMPTY_STRING);

            let (fill, text_color, message) = if config_exhausted {
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
    }
}
