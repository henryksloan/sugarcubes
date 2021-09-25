use crate::transitions::*;

use sugarcubes_core::automata::{
    finite_automaton::{FiniteAutomaton, FiniteAutomatonConfiguration},
    Configuration,
};

use macroquad::prelude::*;
use std::collections::HashMap;

pub const INACTIVE_COLOR: Color = Color::new(0.90, 0.93, 0.52, 1.00);
pub const ACTIVE_COLOR: Color = Color::new(0.44, 0.45, 0.19, 1.00);
pub const SELECTED_COLOR: Color = Color::new(0.45, 0.58, 0.81, 1.00);

pub const STATE_RADIUS: f32 = 35.;
pub const STATE_SIDES: u8 = 35;
pub const FINAL_STATE_CIRCLE_RATIO: f32 = 0.85;
pub const INITIAL_ARROW_SIZE: f32 = 24.;
pub const STATE_FONT_SIZE: f64 = 30.;

pub const DEFAULT_NAME_PREFIX: &str = "q";
// Macroquad currently draws text at weird Y coordinates,
// this is a sane adjustment value.
pub const STATE_TEXT_Y_ADJUSTMENT: f32 = 4.;

pub struct States {
    position_map: HashMap<u32, Vec2>,
    name_map: HashMap<u32, String>,
}

impl States {
    pub fn new() -> Self {
        Self {
            position_map: HashMap::new(),
            name_map: HashMap::new(),
        }
    }

    pub fn get_position(&mut self, state: u32) -> &Vec2 {
        self.position_map.get(&state).unwrap_or(&Vec2::ZERO)
    }

    pub fn insert_position(&mut self, state: u32, position: Vec2) {
        self.position_map.insert(state, position);
    }

    pub fn get_name(&mut self, state: u32) -> String {
        self.name_map
            .get(&state)
            .cloned()
            .unwrap_or_else(String::new)
    }

    pub fn insert_name(&mut self, state: u32, name: String) {
        self.name_map.insert(state, name);
    }

    pub fn default_name(state: u32) -> String {
        DEFAULT_NAME_PREFIX.to_owned() + &state.to_string()
    }

    pub fn add_state(&mut self, fa: &mut FiniteAutomaton, position: Vec2) -> u32 {
        let state = fa.automaton.add_new_state();
        self.position_map.insert(state, position);
        self.name_map.insert(state, Self::default_name(state));
        state
    }

    pub fn try_add_state_with_id(
        &mut self,
        fa: &mut FiniteAutomaton,
        position: Vec2,
        id: u32,
    ) -> bool {
        let succeeded = fa.automaton.try_add_state_with_id(id);
        if succeeded {
            self.position_map.insert(id, position);
            self.name_map
                .insert(id, DEFAULT_NAME_PREFIX.to_owned() + &id.to_string());
        }
        succeeded
    }

    pub fn remove_state(&mut self, fa: &mut FiniteAutomaton, state: u32) {
        fa.automaton.remove_state(state);
        self.position_map.remove(&state);
        self.name_map.remove(&state);
    }

    pub fn point_in_state(&self, point: Vec2, state: u32) -> bool {
        let position = *self.position_map.get(&state).unwrap();
        point.abs_diff_eq(position, STATE_RADIUS)
    }

    pub fn point_in_some_state(&self, point: Vec2, fa: &FiniteAutomaton) -> Option<u32> {
        // Iterate in reverse, so the highest-numbered state is selected first
        fa.automaton
            .states_iter()
            .cloned()
            .rev()
            .find(|&state| self.point_in_state(point, state))
    }

    pub fn draw_state(
        &mut self,
        state: u32,
        is_active: bool,
        is_initial: bool,
        is_final: bool,
        is_selected: bool,
        font: &Font,
    ) {
        let position = *self.get_position(state);
        let state_color = if is_selected {
            SELECTED_COLOR
        } else if is_active {
            ACTIVE_COLOR
        } else {
            INACTIVE_COLOR
        };
        draw_poly(
            position.x,
            position.y,
            STATE_SIDES,
            STATE_RADIUS,
            0.,
            state_color,
        );
        draw_poly_lines(
            position.x,
            position.y,
            STATE_SIDES,
            STATE_RADIUS + 0.5,
            0.,
            2.,
            BLACK,
        );

        if is_final {
            let r = STATE_RADIUS * FINAL_STATE_CIRCLE_RATIO;
            draw_poly_lines(position.x, position.y, STATE_SIDES, r, 0., 2., BLACK);
        }

        if is_initial {
            draw_arrow(
                vec2(position.x - STATE_RADIUS, position.y),
                0.,
                INITIAL_ARROW_SIZE,
                true,
            );
        }

        let text = self.get_name(state);
        let text_size = measure_text(&text, Some(*font), STATE_FONT_SIZE as _, 1.0);
        draw_text_ex(
            &text,
            position.x - text_size.width / 2.,
            position.y - text_size.height / 2. + STATE_RADIUS / 2. + STATE_TEXT_Y_ADJUSTMENT,
            TextParams {
                font_size: STATE_FONT_SIZE as _,
                font: *font,
                color: BLACK,
                ..Default::default()
            },
        );
    }

    pub fn draw_states(
        &mut self,
        fa: &FiniteAutomaton,
        is_simulating: bool,
        configurations: &[FiniteAutomatonConfiguration],
        selected_state: Option<u32>,
        font: &Font,
    ) {
        // Draw states in order of increasing ID, so higher ID states are drawn on top
        for &state in fa.automaton.states_iter() {
            let is_active = is_simulating
                && configurations
                    .iter()
                    .any(|configuration| configuration.state() == state);
            self.draw_state(
                state,
                is_active,
                fa.automaton.initial() == Some(state),
                fa.automaton.is_final(state),
                selected_state == Some(state),
                font,
            )
        }
    }
}
