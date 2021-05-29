use crate::automata::Configuration;

/// A configuration in a finite automaton, including the remaining input
pub struct FiniteAutomatonConfiguration {
    state: u32,
    pub remaining_string: String,
}

impl Configuration for FiniteAutomatonConfiguration {
    fn state(&self) -> u32 {
        self.state
    }
}

impl FiniteAutomatonConfiguration {
    pub fn new(state: u32, remaining_string: String) -> Self {
        Self {
            state,
            remaining_string,
        }
    }

    pub fn next_symbol(&mut self) -> Option<char> {
        self.remaining_string.drain(0..).next()
    }
}
