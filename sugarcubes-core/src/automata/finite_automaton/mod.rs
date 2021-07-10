use crate::automata::{Automaton, Configuration, SimulateAutomaton, Transition, EMPTY_STRING};

pub mod finite_automaton_configuration;
pub mod finite_automaton_transition;

pub use finite_automaton_configuration::FiniteAutomatonConfiguration;
pub use finite_automaton_transition::FiniteAutomatonTransition;

/// A finite automaton defined by a state graph
#[derive(Default)]
pub struct FiniteAutomaton {
    pub automaton: Automaton<FiniteAutomatonTransition>,
}

impl SimulateAutomaton for FiniteAutomaton {
    type ConfigurationType = FiniteAutomatonConfiguration;

    fn initial_configurations(&self, input: &str) -> Vec<Self::ConfigurationType> {
        if let Some(initial) = self.automaton.initial() {
            vec![FiniteAutomatonConfiguration::new(
                initial,
                input.to_string(),
            )]
        } else {
            Vec::new()
        }
    }

    fn step(&self, mut configuration: Self::ConfigurationType) -> Vec<Self::ConfigurationType> {
        let (symbol, remaining) = if let Some((symbol, remaining)) = configuration.next_symbol() {
            (symbol, remaining)
        } else {
            let mut new_configurations = Vec::new();
            for transition in self.automaton.transitions_from(configuration.state()) {
                let transition_symbol = transition.symbol();
                if transition_symbol == EMPTY_STRING {
                    new_configurations.push(FiniteAutomatonConfiguration::new(
                        transition.to(),
                        configuration.remaining_string.clone(),
                    ));
                }
            }
            return new_configurations;
        };

        let mut new_configurations = Vec::new();
        for transition in self.automaton.transitions_from(configuration.state()) {
            let transition_symbol = transition.symbol();
            if transition_symbol == EMPTY_STRING {
                new_configurations.push(FiniteAutomatonConfiguration::new(
                    transition.to(),
                    configuration.remaining_string.clone(),
                ));
            } else if transition_symbol == symbol {
                new_configurations.push(FiniteAutomatonConfiguration::new(
                    transition.to(),
                    remaining.clone(),
                ));
            }
        }
        new_configurations
    }

    fn step_all(
        &self,
        configurations: Vec<Self::ConfigurationType>,
    ) -> Vec<Self::ConfigurationType> {
        configurations
            .into_iter()
            .flat_map(|configuration| self.step(configuration))
            .collect()
    }

    fn check_input(&self, input: &str) -> bool {
        let mut configurations = self.initial_configurations(input);

        // TODO: Implement warnings rather than a hard limit
        // This could be implemented with a different struct
        // that holds configurations and controls progression.
        // Probably include "continue?" warnings WITH "don't show this again",
        // but take steps to avoid unresponsiveness.
        const MAX_ITERS: usize = 1000;
        for _ in 0..MAX_ITERS {
            if configurations.is_empty() {
                return false;
            }

            if configurations.iter().any(|conf| {
                conf.remaining_string.is_empty() && self.automaton.is_final(conf.state())
            }) {
                return true;
            }

            configurations = self.step_all(configurations);
        }

        return false;
    }
}
