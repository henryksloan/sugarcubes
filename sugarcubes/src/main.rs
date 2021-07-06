mod command;
mod states;
mod top_panel;
mod transitions;

use crate::{command::*, states::*, top_panel::*, transitions::*};

use sugarcubes_core::automata::{
    finite_automaton::{FiniteAutomaton, FiniteAutomatonTransition},
    SimulateAutomaton, Transition, EMPTY_STRING,
};

use macroquad::prelude::*;
use macroquad::ui::{hash, root_ui, widgets, Skin};
use std::collections::HashMap;

const DOUBLE_CLICK_DELAY: f64 = 0.25;

fn execute(
    command: Command,
    fa: &mut FiniteAutomaton,
    states: &mut States,
    undo_stack: &mut Vec<Command>,
    redo_stack: &mut Vec<Command>,
) {
    command.execute(fa, states);
    undo_stack.push(command);
    redo_stack.clear();
}

fn undo(
    fa: &mut FiniteAutomaton,
    states: &mut States,
    undo_stack: &mut Vec<Command>,
    redo_stack: &mut Vec<Command>,
) {
    if let Some(command) = undo_stack.pop() {
        command.undo(fa, states);
        redo_stack.push(command);
    }
}

fn redo(
    fa: &mut FiniteAutomaton,
    states: &mut States,
    undo_stack: &mut Vec<Command>,
    redo_stack: &mut Vec<Command>,
) {
    if let Some(command) = redo_stack.pop() {
        command.execute(fa, states);
        undo_stack.push(command);
    }
}

#[macroquad::main("Sugarcubes")]
async fn main() {
    let mut top_panel = TopPanel::new();

    let transition_input_size = vec2(150., 36.);

    let editbox_skin = {
        let editbox_style = root_ui()
            .style_builder()
            .font_size(25)
            .margin(RectOffset::new(0., 0., 5., 0.))
            .font(include_bytes!("../../assets/OpenSans-Regular.ttf"))
            .unwrap()
            .build();
        Skin {
            editbox_style,
            ..root_ui().default_skin()
        }
    };

    let mut fa = FiniteAutomaton::default();
    let mut states = States::new();
    let s0 = states.add_state(&mut fa, vec2(200., 270.));
    let s1 = states.add_state(&mut fa, vec2(400., 170.));
    let s2 = states.add_state(&mut fa, vec2(400., 370.));
    let s3 = states.add_state(&mut fa, vec2(600., 370.));

    fa.automaton.set_initial(s0);
    fa.automaton.set_final(s2, true);

    fa.automaton
        .add_transition(FiniteAutomatonTransition::new(s0, s1, 'x'));
    fa.automaton
        .add_transition(FiniteAutomatonTransition::new(s0, s2, EMPTY_STRING));
    fa.automaton
        .add_transition(FiniteAutomatonTransition::new(s1, s1, 'a'));
    fa.automaton
        .add_transition(FiniteAutomatonTransition::new(s1, s2, 'a'));
    fa.automaton
        .add_transition(FiniteAutomatonTransition::new(s1, s2, 'b'));
    fa.automaton
        .add_transition(FiniteAutomatonTransition::new(s2, s3, 'b'));
    fa.automaton
        .add_transition(FiniteAutomatonTransition::new(s3, s2, 'c'));
    fa.automaton
        .add_transition(FiniteAutomatonTransition::new(s3, s2, 'f'));
    fa.automaton
        .add_transition(FiniteAutomatonTransition::new(s3, s3, 'd'));
    fa.automaton
        .add_transition(FiniteAutomatonTransition::new(s3, s3, 'x'));

    let mut configurations = Vec::new();

    let gl = unsafe { get_internal_gl().quad_gl };

    let font = load_ttf_font("./assets/OpenSans-Regular.ttf")
        .await
        .unwrap();

    // The offset of the click relative to the center of the selected state,
    // so that the mouse "grabs" the state at the point of the initial click
    let mut state_drag_offset = Vec2::ZERO;
    let mut selected_state: Option<u32> = None;
    let mut dragging_selected = false;

    let mut selected_transition: Option<FiniteAutomatonTransition> = None;

    // If the user is drawing a new transition starting on a state, its ID is in here
    let mut creating_transition_from: Option<u32> = None;

    // If the user is editing a transition, this holds its (position, text, state_from, state_to)
    let mut editing_transition: Option<(Vec2, String, u32, u32)> = None;

    let mut undo_stack: Vec<Command> = Vec::new();
    let mut redo_stack: Vec<Command> = Vec::new();

    let mut last_click_time = 0.;

    loop {
        clear_background(WHITE);

        // Process keys, mouse etc.
        let screen_mouse_position = Vec2::from(mouse_position());
        let mouse_position: Vec2 = screen_mouse_position - vec2(0., top_panel.height);

        if let Mode::Edit = top_panel.mode {
            if !top_panel.contains_mouse && is_mouse_button_pressed(MouseButton::Left) {
                let new_click_time = get_time();

                // Check for double click
                if last_click_time > 0. && new_click_time - last_click_time <= DOUBLE_CLICK_DELAY {
                    creating_transition_from = None;

                    if let Some(state) = states.point_in_some_state(mouse_position, &fa) {
                        creating_transition_from = Some(state);
                    } else {
                        let id = fa.automaton.get_next_state_id();
                        execute(
                            Command::CreateState(id, mouse_position),
                            &mut fa,
                            &mut states,
                            &mut undo_stack,
                            &mut redo_stack,
                        );
                        selected_state = Some(id);
                        state_drag_offset = Vec2::ZERO;
                        dragging_selected = true;
                    }
                } else {
                    if let Some(state) = states.point_in_some_state(mouse_position, &fa) {
                        selected_state = Some(state);
                        state_drag_offset = *states.get_position(state) - mouse_position;
                        dragging_selected = true;
                    }
                }

                last_click_time = new_click_time;
            }

            if is_mouse_button_released(MouseButton::Left) {
                if dragging_selected {
                    selected_state = None;
                }

                // If the user releases over a state while creating a transition,
                // connect the two states
                if let Some(from) = creating_transition_from {
                    if let Some(to) = states.point_in_some_state(mouse_position, &fa) {
                        let middle = {
                            let position_from = *states.get_position(from);
                            let position_to = *states.get_position(to);
                            position_from.lerp(position_to, 0.5)
                        } - transition_input_size / 2.;
                        editing_transition = Some((middle, "".to_string(), from, to));
                    }
                }

                creating_transition_from = None;
            }

            if !top_panel.contains_mouse && is_mouse_button_pressed(MouseButton::Right) {
                top_panel.open_context_menu = true;
                top_panel.context_menu_pos = mouse_position + vec2(0., top_panel.height);
                if let Some(state) = states.point_in_some_state(mouse_position, &fa) {
                    selected_state = Some(state);
                    dragging_selected = false;
                    selected_transition = None;
                } else {
                    selected_state = None;
                    selected_transition = None;
                }
            }
        }

        let command_opt = top_panel.ui(
            &fa,
            &mut states,
            &mut configurations,
            &mouse_position,
            &mut selected_state,
            &mut selected_transition,
            !undo_stack.is_empty(),
            !redo_stack.is_empty(),
        );

        if let Some(command) = command_opt {
            match command {
                TopPanelCommand::Command(command) => execute(
                    command,
                    &mut fa,
                    &mut states,
                    &mut undo_stack,
                    &mut redo_stack,
                ),
                TopPanelCommand::Undo => {
                    undo(&mut fa, &mut states, &mut undo_stack, &mut redo_stack)
                }
                TopPanelCommand::Redo => {
                    redo(&mut fa, &mut states, &mut undo_stack, &mut redo_stack)
                }
                TopPanelCommand::Step => configurations = fa.step_all(configurations),
                TopPanelCommand::StartSimulation(new_configurations) => {
                    configurations = new_configurations.to_vec()
                }
            }
        }

        set_camera(&Camera2D::from_display_rect(Rect::new(
            0.,
            -top_panel.height,
            screen_width(),
            screen_height(),
        )));

        // Draw things before egui
        if let Some(selected) = selected_state {
            if dragging_selected {
                states.insert_position(selected, mouse_position + state_drag_offset);
            }
        }

        // Draw states in order of increasing ID, so higher ID states are drawn on top
        for state in fa.automaton.states_iter() {
            let position = *states.get_position(*state);

            // Group transition symbols by the state the transition leads to,
            // so multiple transitions to the same state will display as stacked symbols
            let symbols_by_other_state = fa.automaton.transitions_from(*state).into_iter().fold(
                HashMap::new(),
                |mut map: HashMap<u32, Vec<String>>, transition| {
                    map.entry(transition.to())
                        .or_default()
                        .push(transition.symbol().to_string());
                    map
                },
            );

            for (other_state, symbols) in symbols_by_other_state {
                let (rects, angle) = if *state == other_state {
                    draw_self_transition_with_text(&position, &symbols, &font)
                } else if fa.automaton.states_have_loop(*state, other_state) {
                    let other_position = states.get_position(other_state);
                    draw_curved_transition_with_text(&position, other_position, &symbols, gl, &font)
                } else {
                    let other_position = states.get_position(other_state);
                    draw_transition_with_text(&position, other_position, true, &symbols, gl, &font)
                };
                for (i, rect) in rects.iter().enumerate() {
                    // TODO: Add some padding to the rect for easier clicking
                    if is_mouse_button_pressed(MouseButton::Right)
                        && Rect::new(0., 0., rect.w, rect.h).contains(
                            Mat3::from_rotation_z(-angle)
                                .transform_vector2(mouse_position - rect.point()),
                        )
                        && states.point_in_some_state(mouse_position, &fa).is_none()
                    {
                        selected_transition = Some(FiniteAutomatonTransition::new(
                            *state,
                            other_state,
                            symbols[i].chars().next().unwrap_or(EMPTY_STRING).clone(),
                        ));
                        selected_state = None;
                    }
                }
            }

            let is_simulating = matches!(top_panel.mode, Mode::Simulate);
            states.draw_states(&fa, is_simulating, &configurations, selected_state, &font);
        }

        if let Some(from) = creating_transition_from {
            let position = states.get_position(from);
            if mouse_position.abs_diff_eq(*position, STATE_RADIUS) {
                draw_self_transition(position);
            } else {
                draw_transition(position, &mouse_position, false);
            }
        }

        if let Some(editing_transition) = &mut editing_transition {
            // Workaround for macroquad UI camera bug
            set_default_camera();
            root_ui().push_skin(&editbox_skin);
            widgets::Window::new(
                hash!("win", editing_transition.2, editing_transition.3),
                editing_transition.0 + vec2(0., top_panel.height),
                transition_input_size,
            )
            .titlebar(false)
            .ui(&mut *root_ui(), |ui| {
                let id = hash!(editing_transition.2, editing_transition.3);
                ui.input_text(id, "", &mut editing_transition.1);
                ui.set_input_focus(id);
            });
            root_ui().pop_skin();
        }

        if let Some(tuple) = editing_transition.clone() {
            if is_key_pressed(KeyCode::Enter)
                || (is_mouse_button_pressed(MouseButton::Left)
                    && !root_ui().is_mouse_over(screen_mouse_position))
            {
                let transition = FiniteAutomatonTransition::new(
                    tuple.2,
                    tuple.3,
                    tuple.1.chars().next().unwrap_or(EMPTY_STRING),
                );
                execute(
                    Command::CreateTransition(transition),
                    &mut fa,
                    &mut states,
                    &mut undo_stack,
                    &mut redo_stack,
                );
                editing_transition = None;
            } else if is_key_pressed(KeyCode::Escape) {
                editing_transition = None;
            }
        }

        egui_macroquad::draw();

        // Draw things after egui

        next_frame().await;
    }
}
