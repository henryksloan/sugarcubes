use crate::automata::{State, Transition, TransitionSet};

use std::collections::HashMap;

/// A generic automaton holding a state graph
pub struct Automaton<T: Transition> {
    pub states: HashMap<u32, State>, // Identifies states by an ID
    pub transitions: TransitionSet<T>,
}
