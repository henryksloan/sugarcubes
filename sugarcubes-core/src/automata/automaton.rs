use crate::automata::{State, Transition, TransitionSet};

use std::collections::{btree_map, BTreeMap, HashSet};

/// A generic automaton holding a state graph
#[derive(Default)]
pub struct Automaton<T: Transition> {
    states: BTreeMap<u32, State>, // Identifies states by an ID
    transitions: TransitionSet<T>,
    initial_state: Option<u32>,
    final_states: HashSet<u32>,
}

impl<T: Transition> Automaton<T> {
    /// Create a new state with the smallest free ID, returning the ID
    pub fn add_new_state(&mut self) -> u32 {
        let id = self.get_next_state_id();
        self.states.insert(id, State {});
        self.transitions.register_state(id);
        id
    }

    /// Add a state with a given ID, failing if the ID is taken,
    /// and returning whether it succeeded
    pub fn try_add_state_with_id(&mut self, id: u32) -> bool {
        if let btree_map::Entry::Vacant(entry) = self.states.entry(id) {
            entry.insert(State {});
            self.transitions.register_state(id);
            true
        } else {
            false
        }
    }

    pub fn remove_state(&mut self, id: u32) {
        self.states.remove(&id);
        self.transitions.unregister_state(id);

        // Clean up any invalid data
        if self.initial_state == Some(id) {
            self.initial_state = None;
        }
        self.final_states.remove(&id);
    }

    /// Generate an ID by finding the first unused ordinal
    pub fn get_next_state_id(&mut self) -> u32 {
        let used_ids: HashSet<u32> = self.states.keys().cloned().collect();
        (0..).find(|id| !used_ids.contains(id)).unwrap()
    }

    pub fn states(&self) -> Vec<&u32> {
        self.states.keys().collect()
    }

    /// Returns a sorted iterator of the states of the automaton
    pub fn states_iter(&self) -> btree_map::Keys<u32, State> {
        self.states.keys()
    }

    pub fn transitions_from(&self, from: u32) -> Vec<&T> {
        self.transitions.from(from)
    }

    pub fn transitions_to(&self, to: u32) -> Vec<&T> {
        self.transitions.to(to)
    }

    pub fn transitions_with(&self, state: u32) -> Vec<&T> {
        self.transitions.with(state)
    }

    pub fn states_have_loop(&self, state0: u32, state1: u32) -> bool {
        self.transitions.states_have_loop(state0, state1)
    }

    pub fn add_transition(&mut self, transition: T) {
        if self.states.contains_key(&transition.from())
            && self.states.contains_key(&transition.to())
        {
            self.transitions.add_transition(transition);
        }
    }

    pub fn remove_transition(&mut self, transition: T) {
        if self.states.contains_key(&transition.from())
            && self.states.contains_key(&transition.to())
        {
            self.transitions.remove_transition(transition);
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
