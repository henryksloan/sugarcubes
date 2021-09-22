use crate::states::*;

use sugarcubes_core::automata::finite_automaton::FiniteAutomaton;

// Defines actions that read or write the entire document state
pub enum DocumentCommand {
    OpenJFF(String),
    SaveJFF(String),
}

impl DocumentCommand {
    pub fn execute(&self, fa: &mut FiniteAutomaton, states: &mut States) {
        match &*self {
            Self::OpenJFF(content_string) => {
                *fa = FiniteAutomaton::default();
                *states = States::new();
            }
            Self::SaveJFF(content_string) => {}
        }
    }
}
