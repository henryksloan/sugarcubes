use crate::states::*;

use sugarcubes_core::automata::finite_automaton::{FiniteAutomaton, FiniteAutomatonTransition};

// Defines all undo-able commands in edit mode
pub enum Command {
    SetInitial(u32, Option<u32>),
    RemoveInitial(u32),
    SetFinal(u32, bool),

    DeleteState(u32),
    DeleteTransition(FiniteAutomatonTransition),
}

impl Command {
    pub fn execute(&self, fa: &mut FiniteAutomaton, states: &mut States) {
        match *self {
            Self::SetInitial(state, _) => fa.automaton.set_initial(state),
            Self::RemoveInitial(_) => fa.automaton.remove_initial(),
            Self::SetFinal(state, value) => fa.automaton.set_final(state, value),

            Self::DeleteState(state) => states.remove_state(fa, state),
            Self::DeleteTransition(transition) => fa.automaton.remove_transition(transition),
        }
    }

    pub fn undo(&self, fa: &mut FiniteAutomaton, states: &mut States) {
        match *self {
            Self::SetInitial(_, old_initial) => {
                if let Some(state) = old_initial {
                    fa.automaton.set_initial(state);
                } else {
                    fa.automaton.remove_initial();
                }
            }
            Self::RemoveInitial(state) => fa.automaton.set_initial(state),
            Self::SetFinal(state, value) => fa.automaton.set_final(state, !value),
            _ => {}
        }
    }
}
