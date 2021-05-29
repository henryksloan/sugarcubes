use crate::automata::{Automaton, SimulateAutomaton, Transition};

pub mod finite_automaton_configuration;
pub mod finite_automaton_transition;

pub use finite_automaton_configuration::FiniteAutomatonConfiguration;
pub use finite_automaton_transition::FiniteAutomatonTransition;

use super::Configuration;

/// A finite automaton defined by a state graph
pub struct FiniteAutomaton {
    automaton: Automaton<FiniteAutomatonTransition>,
}

impl SimulateAutomaton for FiniteAutomaton {
    type ConfigurationType = FiniteAutomatonConfiguration;

    fn step(&self, mut configuration: Self::ConfigurationType) -> Vec<Self::ConfigurationType> {
        let symbol = if let Some(symbol) = configuration.next_symbol() {
            symbol
        } else {
            return Vec::new();
        };

        let mut new_configurations = Vec::new();
        for &key in self.automaton.transitions.from(configuration.state()) {
            let transition = self.automaton.transitions.get(key).unwrap();
            if symbol == transition.symbol() {
                new_configurations.push(FiniteAutomatonConfiguration::new(
                    transition.to(),
                    configuration.remaining_string.clone(),
                ));
            }
        }
        new_configurations
    }
}
