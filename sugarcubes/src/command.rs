use crate::states::*;

use sugarcubes_core::automata::{
    finite_automaton::{FiniteAutomaton, FiniteAutomatonConfiguration, FiniteAutomatonTransition},
    Configuration,
};

// Defines all undo-able commands in edit mode
pub enum Command {
    SetInitial(u32),
    RemoveInitial,
    SetFinal(u32, bool),

    DeleteState(u32),
    DeleteTransition(FiniteAutomatonTransition),
}

impl Command {
    pub fn execute(
        &self,
        fa: &mut FiniteAutomaton,
        states: &mut States,
        configurations: &mut Vec<FiniteAutomatonConfiguration>,
    ) {
        match *self {
            Self::SetInitial(state) => fa.automaton.set_initial(state),
            Self::RemoveInitial => fa.automaton.remove_initial(),
            Self::SetFinal(state, value) => fa.automaton.set_final(state, value),

            Self::DeleteState(state) => {
                states.remove_state(fa, state);
                // TODO: This won't be necessary once editing and simulation modes are separated
                configurations.retain(|conf| conf.state() != state);
            }
            Self::DeleteTransition(transition) => fa.automaton.remove_transition(transition),
        }
    }
}
