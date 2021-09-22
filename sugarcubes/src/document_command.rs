use crate::states::*;

use sugarcubes_core::automata::finite_automaton::{FiniteAutomaton, FiniteAutomatonTransition};

use macroquad::prelude::*;

// Defines actions that read or write the entire document state
pub enum DocumentCommand {
    OpenJFF(String),
    SaveJFF(String),
}

impl DocumentCommand {
    pub fn execute(&self, fa: &mut FiniteAutomaton, states: &mut States) {
        match &*self {
            Self::OpenJFF(content_string) => {
                // TODO: Report errors to user
                let _ = self.open_jff(content_string, fa, states);
            }
            Self::SaveJFF(_content_string) => {}
        }
    }

    fn open_jff(
        &self,
        content_string: &str,
        fa: &mut FiniteAutomaton,
        states: &mut States,
    ) -> Option<()> {
        // TODO: Make this operation atomic
        *fa = FiniteAutomaton::default();
        *states = States::new();

        let element = xmltree::Element::parse(content_string.as_bytes()).ok()?;
        // TODO: Error if type is wrong. Eventually, treat different types differently.
        let _model_type = element.get_child("type")?.get_text()?;
        let automaton = element.get_child("automaton")?;

        // First, read and add all the states
        for child in &automaton.children {
            if let xmltree::XMLNode::Element(element) = child {
                if element.name.as_str() == "state" {
                    let id: u32 = element.attributes.get("id")?.parse().ok()?;
                    let x: f32 = element.get_child("x")?.get_text()?.parse().ok()?;
                    let y: f32 = element.get_child("y")?.get_text()?.parse().ok()?;
                    let is_initial = element.get_child("initial").is_some();
                    let is_final = element.get_child("final").is_some();
                    // TODO: Tune these multipliers, and move them to constants
                    states.try_add_state_with_id(fa, vec2(x * 2.0, y * 2.0), id);

                    if is_initial {
                        fa.automaton.set_initial(id);
                    }

                    if is_final {
                        fa.automaton.set_final(id, true);
                    }
                }
            }
        }

        // After reading all the states, take a second pass to read and add all the transitions
        for child in &automaton.children {
            if let xmltree::XMLNode::Element(element) = child {
                if element.name.as_str() == "transition" {
                    let from: u32 = element.get_child("from")?.get_text()?.parse().ok()?;
                    let to: u32 = element.get_child("to")?.get_text()?.parse().ok()?;
                    let read = element.get_child("read")?.get_text()?.chars().next()?;
                    fa.automaton
                        .add_transition(FiniteAutomatonTransition::new(from, to, read));
                }
            }
        }

        Some(())
    }
}
