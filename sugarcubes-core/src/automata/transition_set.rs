use crate::automata::Transition;

use slotmap::{DefaultKey, SlotMap};
use std::collections::{HashMap, HashSet};

/// A collection of transitions with auxiliary maps
#[derive(Default)]
pub struct TransitionSet<T: Transition> {
    // TODO: Transitions from a given state to another should stay in the order in which they were inserted
    transitions: SlotMap<DefaultKey, T>,
    transitions_from: HashMap<u32, HashSet<DefaultKey>>, // Transitions coming from a given state
    transitions_to: HashMap<u32, HashSet<DefaultKey>>,   // Transitions going into a given state
}

impl<T: Transition> TransitionSet<T> {
    pub fn add_transition(&mut self, transition: T) {
        if self.has_transition(&transition) {
            return;
        }

        let (from, to) = (transition.from(), transition.to());
        let key = self.transitions.insert(transition);
        self.transitions_from.entry(from).or_default().insert(key);
        self.transitions_to.entry(to).or_default().insert(key);
    }

    pub fn register_state(&mut self, state: u32) {
        self.transitions_from.entry(state).or_default();
        self.transitions_to.entry(state).or_default();
    }

    /// Returns whether two states both have transitions to the other
    pub fn states_have_loop(&self, state0: u32, state1: u32) -> bool {
        self.from(state0)
            .into_iter()
            .any(|transition| transition.to() == state1)
            && self
                .to(state0)
                .into_iter()
                .any(|transition| transition.from() == state1)
    }

    pub fn from(&self, from: u32) -> Vec<&T> {
        self.keys_from(from)
            .iter()
            .map(|&key| self.transitions.get(key).unwrap())
            .collect()
    }

    pub fn to(&self, to: u32) -> Vec<&T> {
        self.keys_to(to)
            .iter()
            .map(|&key| self.transitions.get(key).unwrap())
            .collect()
    }

    fn keys_from(&self, from: u32) -> &HashSet<DefaultKey> {
        self.transitions_from
            .get(&from)
            .expect("no transitions_from for state")
    }

    fn keys_to(&self, to: u32) -> &HashSet<DefaultKey> {
        self.transitions_to
            .get(&to)
            .expect("no transitions_to for state")
    }

    fn has_transition(&self, transition: &T) -> bool {
        if let Some(set_from) = self.transitions_from.get(&transition.from()) {
            set_from
                .into_iter()
                .any(|key| *self.transitions.get(*key).unwrap() == *transition)
        } else {
            false
        }
    }
}
