use crate::{command::Command, states::States};

use sugarcubes_core::automata::finite_automaton::FiniteAutomaton;

pub struct TopPanelCommandHandler {
    undo_stack: Vec<Command>,
    redo_stack: Vec<Command>,
}

impl TopPanelCommandHandler {
    pub fn new() -> Self {
        Self {
            undo_stack: Vec::new(),
            redo_stack: Vec::new(),
        }
    }

    pub fn execute(&mut self, command: Command, fa: &mut FiniteAutomaton, states: &mut States) {
        command.execute(fa, states);
        self.undo_stack.push(command);
        self.redo_stack.clear();
    }

    pub fn undo(&mut self, fa: &mut FiniteAutomaton, states: &mut States) {
        if let Some(command) = self.undo_stack.pop() {
            command.undo(fa, states);
            self.redo_stack.push(command);
        }
    }

    pub fn redo(&mut self, fa: &mut FiniteAutomaton, states: &mut States) {
        if let Some(command) = self.redo_stack.pop() {
            command.execute(fa, states);
            self.undo_stack.push(command);
        }
    }

    pub fn can_undo(&self) -> bool {
        !self.undo_stack.is_empty()
    }

    pub fn can_redo(&self) -> bool {
        !self.redo_stack.is_empty()
    }
}
