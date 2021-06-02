use crate::automata::Transition;

/// A transition between two states in a finite automaton
#[derive(Default, PartialEq)]
pub struct FiniteAutomatonTransition {
    from: u32,
    to: u32,
    symbol: char,
}

impl Transition for FiniteAutomatonTransition {
    fn from(&self) -> u32 {
        self.from
    }

    fn to(&self) -> u32 {
        self.to
    }
}

impl FiniteAutomatonTransition {
    pub fn new(from: u32, to: u32, symbol: char) -> Self {
        Self { from, to, symbol }
    }

    pub fn symbol(&self) -> char {
        self.symbol
    }
}
