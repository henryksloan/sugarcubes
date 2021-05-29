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
            return Vec::new();
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
}
