use crate::states::*;

use sugarcubes_core::automata::finite_automaton::{FiniteAutomaton, FiniteAutomatonTransition};

use macroquad::prelude::Vec2;

// Defines all undo-able commands in edit mode
pub enum Command {
    // The new initial state, and optionally the old one
    SetInitial(u32, Option<u32>),
    // The initial state to be made non-initial
    RemoveInitial(u32),
    // The state to be updated, and the new value of is_final
    SetFinal(u32, bool),

    // The state's ID and its old and new names
    SetStateName(u32, String, String),

    // The state's ID and position
    CreateState(u32, Vec2),
    // The transition to be added
    CreateTransition(FiniteAutomatonTransition),

    // The state, its position, its name, and transitions involving it
    DeleteState(u32, Vec2, String, Vec<FiniteAutomatonTransition>),
    // The transition to be deleted
    DeleteTransition(FiniteAutomatonTransition),
}

impl Command {
    pub fn execute(&self, fa: &mut FiniteAutomaton, states: &mut States) {
        match self {
            Self::SetInitial(state, _) => fa.automaton.set_initial(*state),
            Self::RemoveInitial(_) => fa.automaton.remove_initial(),
            Self::SetFinal(state, value) => fa.automaton.set_final(*state, *value),

            Self::SetStateName(state, _, new_name) => {
                states.insert_name(*state, new_name.to_string())
            }

            Self::CreateState(state, pos) => {
                states.try_add_state_with_id(fa, *pos, *state);
            }
            Self::CreateTransition(transition) => fa.automaton.add_transition(*transition),

            Self::DeleteState(state, _, _, _) => states.remove_state(fa, *state),
            Self::DeleteTransition(transition) => fa.automaton.remove_transition(*transition),
        }
    }

    pub fn undo(&self, fa: &mut FiniteAutomaton, states: &mut States) {
        match self {
            Self::SetInitial(_, old_initial) => {
                if let Some(state) = *old_initial {
                    fa.automaton.set_initial(state);
                } else {
                    fa.automaton.remove_initial();
                }
            }
            Self::RemoveInitial(state) => fa.automaton.set_initial(*state),
            Self::SetFinal(state, value) => fa.automaton.set_final(*state, !value),

            Self::SetStateName(state, old_name, _) => {
                states.insert_name(*state, old_name.to_string())
            }

            Self::CreateState(state, _) => states.remove_state(fa, *state),
            Self::CreateTransition(transition) => fa.automaton.remove_transition(*transition),

            Self::DeleteState(id, position, name, transitions) => {
                // TODO: Remember name in the command so it can be undone
                let succeeded = states.try_add_state_with_id(fa, *position, *id);
                if succeeded {
                    states.insert_name(*id, name.to_string());
                    for &transition in transitions {
                        fa.automaton.add_transition(transition)
                    }
                }
            }
            Self::DeleteTransition(transition) => fa.automaton.add_transition(*transition),
        }
    }
}
