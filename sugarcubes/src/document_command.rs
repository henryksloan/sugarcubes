use crate::states::*;

use sugarcubes_core::automata::{
    finite_automaton::{FiniteAutomaton, FiniteAutomatonTransition},
    Transition,
};

use macroquad::prelude::*;
use xmltree::{Element, XMLNode};

#[cfg(target_arch = "wasm32")]
use sapp_jsutils::JsObject;

use std::io::BufWriter;

#[cfg(target_arch = "wasm32")]
extern "C" {
    fn save_jff_file(content: JsObject);
}

// The sizing scale going from JFLAP to Sugarcubes
// e.g. 2.0 means (100.0, 100.0) in JFLAP is (200.0, 200.0) in Sugarcubes
const SCALE_FACTOR: f32 = 2.0;

// Defines actions that read or write the entire document state
pub enum DocumentCommand {
    NewFile,
    OpenJFF(String),
    SaveJFF,
}

impl DocumentCommand {
    pub fn execute(&self, fa: &mut FiniteAutomaton, states: &mut States) {
        match &*self {
            Self::NewFile => {
                // TODO: Alert the user if they have unsaved changes
                // possibly by packing all save-invalidating commands (e.g. moving states,
                // deleting) into a dispatcher
                *fa = FiniteAutomaton::default();
                *states = States::new();
            }
            Self::OpenJFF(content_string) => {
                // TODO: Report errors to user
                let _ = self.open_jff(content_string, fa, states);
            }
            Self::SaveJFF => {
                // TODO: Report errors to user
                let _ = self.save_jff(fa, states);
            }
        }
    }

    fn open_jff(
        &self,
        content_string: &str,
        fa: &mut FiniteAutomaton,
        states: &mut States,
    ) -> Option<()> {
        let mut new_fa = FiniteAutomaton::default();
        let mut new_states = States::new();

        let element = Element::parse(content_string.as_bytes()).ok()?;
        // TODO: Error if type is wrong. Eventually, treat different types differently.
        let _model_type = element.get_child("type")?.get_text()?;
        let automaton = element.get_child("automaton")?;

        // First, read and add all the states
        for child in &automaton.children {
            if let XMLNode::Element(element) = child {
                if element.name.as_str() == "state" {
                    let id: u32 = element.attributes.get("id")?.parse().ok()?;
                    let x: f32 = element.get_child("x")?.get_text()?.parse().ok()?;
                    let y: f32 = element.get_child("y")?.get_text()?.parse().ok()?;
                    let is_initial = element.get_child("initial").is_some();
                    let is_final = element.get_child("final").is_some();
                    new_states.try_add_state_with_id(
                        &mut new_fa,
                        vec2(x * SCALE_FACTOR, y * SCALE_FACTOR),
                        id,
                    );

                    if is_initial {
                        new_fa.automaton.set_initial(id);
                    }

                    if is_final {
                        new_fa.automaton.set_final(id, true);
                    }
                }
            }
        }

        // After reading all the states, take a second pass to read and add all the transitions
        for child in &automaton.children {
            if let XMLNode::Element(element) = child {
                if element.name.as_str() == "transition" {
                    let from: u32 = element.get_child("from")?.get_text()?.parse().ok()?;
                    let to: u32 = element.get_child("to")?.get_text()?.parse().ok()?;
                    let read = element.get_child("read")?.get_text()?.chars().next()?;
                    new_fa
                        .automaton
                        .add_transition(FiniteAutomatonTransition::new(from, to, read));
                }
            }
        }

        *fa = new_fa;
        *states = new_states;

        Some(())
    }

    fn save_jff(&self, fa: &FiniteAutomaton, states: &mut States) -> Option<()> {
        let mut structure = Element::new("structure");

        let mut model_type = Element::new("type");
        model_type.children.push(XMLNode::Text("fa".to_string()));
        structure.children.push(XMLNode::Element(model_type));

        let mut automaton = Element::new("automaton");

        automaton
            .children
            .push(XMLNode::Comment("The list of states.".to_string()));
        for id in fa.automaton.states_iter() {
            let mut state = Element::new("state");

            let id_string = id.to_string();
            state.attributes.insert("id".to_owned(), id_string);
            state
                .attributes
                .insert("name".to_owned(), states.get_name(*id));

            let (x, y) = (*states.get_position(*id)).into();
            let mut x_element = Element::new("x");
            x_element
                .children
                .push(XMLNode::Text((x / SCALE_FACTOR).to_string()));
            state.children.push(XMLNode::Element(x_element));
            let mut y_element = Element::new("y");
            y_element
                .children
                .push(XMLNode::Text((y / SCALE_FACTOR).to_string()));
            state.children.push(XMLNode::Element(y_element));

            if fa.automaton.initial() == Some(*id) {
                state
                    .children
                    .push(XMLNode::Element(Element::new("initial")));
            }

            if fa.automaton.is_final(*id) {
                state.children.push(XMLNode::Element(Element::new("final")));
            }

            automaton.children.push(XMLNode::Element(state));
        }

        automaton
            .children
            .push(XMLNode::Comment("The list of transitions.".to_string()));
        for transition in fa.automaton.transitions() {
            let mut transition_element = Element::new("transition");

            let mut from = Element::new("from");
            from.children
                .push(XMLNode::Text(transition.from().to_string()));
            transition_element.children.push(XMLNode::Element(from));

            let mut to = Element::new("to");
            to.children.push(XMLNode::Text(transition.to().to_string()));
            transition_element.children.push(XMLNode::Element(to));

            let mut read = Element::new("read");
            read.children
                .push(XMLNode::Text(transition.symbol().to_string()));
            transition_element.children.push(XMLNode::Element(read));

            automaton
                .children
                .push(XMLNode::Element(transition_element));
        }

        structure.children.push(XMLNode::Element(automaton));

        unsafe {
            let mut content = String::new();
            let buffer = BufWriter::new(content.as_mut_vec());
            // TODO: Consider formatting the file better, e.g. with newlines
            structure.write(buffer).ok()?;
            #[cfg(target_arch = "wasm32")]
            save_jff_file(JsObject::string(&content));
        }

        Some(())
    }
}
