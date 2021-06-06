mod states;
mod transitions;

use crate::{states::*, transitions::*};

use sugarcubes_core::automata::{
    finite_automaton::{FiniteAutomaton, FiniteAutomatonTransition},
    Configuration, SimulateAutomaton, Transition, EMPTY_STRING,
};

use macroquad::prelude::*;
use macroquad::ui::{hash, root_ui, widgets, Skin};
use std::collections::HashMap;

const DOUBLE_CLICK_DELAY: f64 = 0.25;

#[macroquad::main("Sugarcubes")]
async fn main() {
    let transition_input_size = vec2(150., 36.);

    let editbox_skin = {
        let editbox_style = root_ui()
            .style_builder()
            .font_size(25)
            .margin(RectOffset::new(0., 0., 5., 0.))
            .font(include_bytes!("../../assets/OpenSans-Regular.ttf"))
            .build();
        Skin {
            editbox_style,
            ..root_ui().default_skin()
        }
    };

    let mut fa = FiniteAutomaton::default();
    let mut states = States::new();
    let s0 = states.add_state(&mut fa, vec2(200., 300.));
    let s1 = states.add_state(&mut fa, vec2(400., 200.));
    let s2 = states.add_state(&mut fa, vec2(400., 400.));
    let s3 = states.add_state(&mut fa, vec2(600., 400.));

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
        .add_transition(FiniteAutomatonTransition::new(s2, s3, 'b'));
    fa.automaton
        .add_transition(FiniteAutomatonTransition::new(s3, s2, 'c'));

    let mut configurations = fa.initial_configurations("xabc");

    let gl = unsafe { get_internal_gl().quad_gl };

    let font = load_ttf_font("./assets/OpenSans-Regular.ttf").await;

    // The offset of the click relative to the center of the selected state,
    // so that the mouse "grabs" the state at the point of the initial click
    let mut state_drag_offset = Vec2::ZERO;
    let mut selected_state: Option<u32> = None;
    let mut dragging_selected = false;

    // If the user is drawing a new transition starting on a state, its ID is in here
    let mut creating_transition_from: Option<u32> = None;

    // If the user is editing a transition, this holds its (position, text, state_from, state_to)
    let mut editing_transition: Option<(Vec2, String, u32, u32)> = None;

    let mut last_click_time = 0.;
    let mut mouse_over_egui = false;
    let mut open_context_menu = false;
    let mut context_menu_pos = Vec2::ZERO;

    loop {
        clear_background(WHITE);

        // Process keys, mouse etc.
        let mouse_position: Vec2 = mouse_position().into();
        if !mouse_over_egui && is_mouse_button_pressed(MouseButton::Left) {
            let new_click_time = get_time();

            // Check for double click
            if last_click_time > 0. && new_click_time - last_click_time <= DOUBLE_CLICK_DELAY {
                creating_transition_from = None;

                if let Some(state) = states.point_in_some_state(mouse_position, &fa) {
                    creating_transition_from = Some(state);
                } else {
                    selected_state = Some(states.add_state(&mut fa, mouse_position));
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

        if !mouse_over_egui && is_mouse_button_pressed(MouseButton::Right) {
            open_context_menu = true;
            context_menu_pos = mouse_position;
            if let Some(state) = states.point_in_some_state(mouse_position, &fa) {
                selected_state = Some(state);
                dragging_selected = false;
            } else {
                selected_state = None;
            }
        }

        let mut should_step = false;
        egui_macroquad::ui(|egui_ctx| {
            egui::TopPanel::top("Sugarcubes").show(egui_ctx, |ui| {
                egui::menu::bar(ui, |ui| {
                    egui::menu::menu(ui, "File", |ui| {
                        if ui.button("Open").clicked() {
                            // ...
                        }
                    });
                });

                ui.separator();

                ui.label("Simulation Toolbar");
                if ui.button("Step").clicked() {
                    should_step = true;
                }

                mouse_over_egui = ui.ui_contains_pointer();
            });

            egui::Area::new("my_area").show(egui_ctx, |ui| {
                let popup_id = ui.make_persistent_id("context_menu_id");
                if open_context_menu {
                    ui.memory().open_popup(popup_id);
                    open_context_menu = false;
                }
                let mut mouse_in_popup = false;
                if ui.memory().is_popup_open(popup_id) {
                    let parent_clip_rect = ui.clip_rect();

                    egui::Area::new(popup_id)
                        .order(egui::Order::Foreground)
                        .fixed_pos((context_menu_pos.x, context_menu_pos.y))
                        .show(ui.ctx(), |ui| {
                            ui.set_clip_rect(parent_clip_rect);
                            let frame = egui::Frame::popup(ui.style());
                            let frame_margin = frame.margin;
                            frame.show(ui, |ui| {
                                ui.with_layout(
                                    egui::Layout::top_down_justified(egui::Align::LEFT),
                                    |ui| {
                                        ui.set_width(100.0 - 2.0 * frame_margin.x);
                                        if let Some(selected) = selected_state {
                                            let mut is_initial =
                                                fa.automaton.initial() == Some(selected);
                                            if ui.checkbox(&mut is_initial, "Initial").changed() {
                                                if is_initial {
                                                    fa.automaton.set_initial(selected);
                                                } else {
                                                    fa.automaton.remove_initial();
                                                }
                                                selected_state = None;
                                                ui.memory().close_popup();
                                            }

                                            let mut is_final = fa.automaton.is_final(selected);
                                            if ui.checkbox(&mut is_final, "Final").changed() {
                                                fa.automaton.set_final(selected, is_final);
                                                selected_state = None;
                                                ui.memory().close_popup();
                                            }

                                            ui.separator();

                                            if ui.button("Delete").clicked() {
                                                if let Some(selected) = selected_state {
                                                    states.remove_state(&mut fa, selected);
                                                    // TODO: This won't be necessary once editing and simulation modes are separated
                                                    configurations
                                                        .retain(|conf| conf.state() != selected);
                                                }
                                                selected_state = None;
                                                ui.memory().close_popup();
                                            }
                                        }

                                        mouse_in_popup = ui.ui_contains_pointer();
                                        mouse_over_egui |= mouse_in_popup;
                                    },
                                );
                            });
                        });

                    if ui.input().key_pressed(egui::Key::Escape) {
                        ui.memory().close_popup();
                    } else if is_mouse_button_pressed(MouseButton::Left) && !mouse_in_popup {
                        ui.memory().close_popup();

                        // Clear selected state if the cancelling click is not in the selected state
                        if let Some(selected) = selected_state {
                            if !states.point_in_state(mouse_position, selected) {
                                selected_state = None;
                            }
                        }
                    }

                    mouse_over_egui |= ui.ui_contains_pointer();
                }
            });
        });

        if should_step {
            configurations = fa.step_all(configurations);
        }

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
                if *state == other_state {
                    draw_self_transition_with_text(&position, symbols, &font);
                } else if fa.automaton.states_have_loop(*state, other_state) {
                    let other_position = states.get_position(other_state);
                    draw_curved_transition_with_text(&position, other_position, symbols, gl, &font)
                } else {
                    let other_position = states.get_position(other_state);
                    draw_transition_with_text(&position, other_position, true, symbols, gl, &font)
                }
            }

            states.draw_states(&fa, &configurations, selected_state, &font);
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
            root_ui().push_skin(&editbox_skin);
            widgets::Window::new(
                hash!("win", editing_transition.2, editing_transition.3),
                editing_transition.0,
                transition_input_size,
            )
            .titlebar(false)
            .ui(&mut *root_ui(), |ui| {
                ui.input_text(
                    hash!(editing_transition.2, editing_transition.3),
                    "",
                    &mut editing_transition.1,
                );
            });
            root_ui().pop_skin();
        }

        if let Some(tuple) = editing_transition.clone() {
            if is_key_pressed(KeyCode::Enter)
                || (is_mouse_button_pressed(MouseButton::Left)
                    && !root_ui().is_mouse_over(mouse_position))
            {
                fa.automaton.add_transition(FiniteAutomatonTransition::new(
                    tuple.2,
                    tuple.3,
                    tuple.1.chars().next().unwrap_or(EMPTY_STRING),
                ));
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
