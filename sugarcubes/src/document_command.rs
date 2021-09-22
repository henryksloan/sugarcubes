use crate::states::*;

use sugarcubes_core::automata::finite_automaton::{FiniteAutomaton, FiniteAutomatonTransition};

use macroquad::prelude::*;
use sapp_jsutils::JsObject;

// TODO: Remove
extern "C" {
    fn console_log(object: JsObject);
}

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
        let model_type = element.get_child("type")?.get_text()?;
        let automaton = element.get_child("automaton")?;

        for child in &automaton.children {
            if let xmltree::XMLNode::Element(element) = child {
                match element.name.as_str() {
                    "state" => {
                        let id: u32 = element.attributes.get("id")?.parse().ok()?;
                        let x: f32 = element.get_child("x")?.get_text()?.parse().ok()?;
                        let y: f32 = element.get_child("y")?.get_text()?.parse().ok()?;
                        let is_initial = element.get_child("initial").is_some();
                        let is_final = element.get_child("final").is_some();
                        unsafe {
                            console_log(JsObject::string(
                                &format!(
                                    "state {} {} ({}, {}) initial:{} final:{}",
                                    id,
                                    element.attributes.get("name")?,
                                    x,
                                    y,
                                    is_initial,
                                    is_final,
                                )
                                .to_string(),
                            ));
                            // TODO: Tune these multipliers, and move them to constants
                            states.try_add_state_with_id(fa, vec2(x * 2.0, y * 2.0), id);
                        }
                    }
                    "transition" => {
                        let from: u32 = element.get_child("from")?.get_text()?.parse().ok()?;
                        let to: u32 = element.get_child("to")?.get_text()?.parse().ok()?;
                        let read = element.get_child("read")?.get_text()?.chars().next()?;
                        unsafe {
                            console_log(JsObject::string(
                                &format!("transition {} -> {} symbol: {}", from, to, read)
                                    .to_string(),
                            ));
                        }
                        fa.automaton
                            .add_transition(FiniteAutomatonTransition::new(to, from, read));
                    }
                    _ => {}
                }
            }
        }

        unsafe {
            console_log(JsObject::string(
                &format!("type: {:#?}", model_type).to_string(),
            ));
            // console_log(JsObject::string(&format!("{:#?}", element).to_string()));
        }
        Some(())
    }
}
