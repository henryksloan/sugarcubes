use crate::automata::Transition;

use slotmap::{DefaultKey, SlotMap};
use std::collections::{HashMap, HashSet};

/// A collection of transitions with auxiliary maps
pub struct TransitionSet<T: Transition> {
    transitions: SlotMap<DefaultKey, T>,
    transitions_from: HashMap<u32, HashSet<DefaultKey>>, // Transitions coming from a given state
    transitions_to: HashMap<u32, HashSet<DefaultKey>>,   // Transitions going into a given state
}

impl<T: Transition> TransitionSet<T> {
    pub fn add_transition(&mut self, transition: T) {
        let (from, to) = (transition.from(), transition.to());
        let key = self.transitions.insert(transition);
        self.transitions_from.entry(from).or_default().insert(key);
        self.transitions_to.entry(to).or_default().insert(key);
    }

    pub fn from(&self, from: u32) -> &HashSet<DefaultKey> {
        self.transitions_from
            .get(&from)
            .expect("no transition_from for state")
    }

    pub fn to(&self, to: u32) -> &HashSet<DefaultKey> {
        self.transitions_to
            .get(&to)
            .expect("no transition_to for state")
    }

    pub fn get(&self, key: DefaultKey) -> Option<&T> {
        self.transitions.get(key)
    }
}
