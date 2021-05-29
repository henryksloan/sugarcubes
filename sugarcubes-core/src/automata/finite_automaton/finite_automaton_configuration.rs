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

    /// Returns the next symbol and the string remaining after it, or None if there is no next symbol
    pub fn next_symbol(&mut self) -> Option<(char, String)> {
        let mut chars = self.remaining_string.chars();
        if let Some(next) = chars.next() {
            Some((next, chars.collect()))
        } else {
            None
        }
    }
}
