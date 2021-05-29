use crate::automata::{State, Transition, TransitionSet};

use std::collections::{HashMap, HashSet};

/// A generic automaton holding a state graph
#[derive(Default)]
pub struct Automaton<T: Transition> {
    states: HashMap<u32, State>, // Identifies states by an ID
    transitions: TransitionSet<T>,
    initial_state: Option<u32>,
    final_states: HashSet<u32>,
}

impl<T: Transition> Automaton<T> {
    /// Creates a new state with the smallest free ID, returning the ID
    pub fn add_new_state(&mut self) -> u32 {
        let used_ids: HashSet<u32> = self.states.keys().cloned().collect();
        let id = (0..).filter(|id| !used_ids.contains(id)).next().unwrap();
        self.states.insert(id, State {});
        self.transitions.register_state(id);
        id
    }

    pub fn states(&self) -> Vec<&u32> {
        self.states.keys().collect()
    }

    pub fn transitions_from(&self, from: u32) -> Vec<&T> {
        self.transitions.from(from)
    }

    pub fn transitions_to(&self, to: u32) -> Vec<&T> {
        self.transitions.to(to)
    }

    pub fn add_transition(&mut self, transition: T) {
        if self.states.contains_key(&transition.from())
            && self.states.contains_key(&transition.to())
        {
            self.transitions.add_transition(transition)
        }
    }

    pub fn has_initial(&self) -> bool {
        self.initial_state.is_some()
    }

    pub fn initial(&self) -> Option<u32> {
        self.initial_state
    }

    pub fn set_initial(&mut self, state: u32) {
        if self.states.contains_key(&state) {
            self.initial_state = Some(state);
        }
    }

    pub fn remove_initial(&mut self) {
        self.initial_state = None;
    }

    pub fn is_final(&self, state: u32) -> bool {
        self.final_states.contains(&state)
    }

    pub fn set_final(&mut self, state: u32, value: bool) {
        if value {
            self.final_states.insert(state);
        } else {
            self.final_states.remove(&state);
        }
    }
}
