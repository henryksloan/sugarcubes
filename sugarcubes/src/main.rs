use sugarcubes_core::automata::{
    finite_automaton::{FiniteAutomaton, FiniteAutomatonTransition},
    Configuration, SimulateAutomaton, Transition, EMPTY_STRING,
};

use macroquad::prelude::*;
use std::collections::HashMap;

const INACTIVE_COLOR: Color = Color::new(0.90, 0.93, 0.52, 1.00);
const ACTIVE_COLOR: Color = Color::new(0.44, 0.45, 0.19, 1.00);
const SELECTED_COLOR: Color = Color::new(0.45, 0.58, 0.81, 1.00);

const DOUBLE_CLICK_DELAY: f64 = 0.25;

const STATE_RADIUS: f32 = 35.;
const INITIAL_ARROW_SIZE: f32 = 24.;
const ARROW_SIZE: f32 = 17.;
const TRANSITION_FONT_SIZE: f64 = 24.;
const STATE_FONT_SIZE: f64 = 30.;

// Draw an arrow with its tip at a given point,
// at a given angle relative to the horizontal,
// and with a given sidelength
fn draw_arrow(point: Vec2, angle: f32, size: f32, outlined: bool) {
    let a = (angle + std::f32::consts::FRAC_PI_6).cos() * size;
    let b = (angle + std::f32::consts::FRAC_PI_6).sin() * size;
    let c = (angle - std::f32::consts::FRAC_PI_6).cos() * size;
    let d = (angle - std::f32::consts::FRAC_PI_6).sin() * size;
    let v1 = point - vec2(a, b);
    let v2 = point - vec2(c, d);

    if outlined {
        draw_triangle(point, v1, v2, WHITE);
        draw_triangle_lines(point, v1, v2, 2., BLACK);
    } else {
        draw_triangle(point, v1, v2, BLACK);
    }
}

fn draw_transition(from: &Vec2, to: &Vec2, to_state: bool) {
    let angle = vec2(1., 0.).angle_between(*to - *from);
    let distance = from.distance(*to);
    let radius_over_distance = STATE_RADIUS / distance;
    let point_from = from.lerp(*to, radius_over_distance);
    let point_to = to.lerp(*from, if to_state { radius_over_distance } else { 0. });
    draw_line(
        point_from.x,
        point_from.y,
        point_to.x,
        point_to.y,
        2.,
        BLACK,
    );
    draw_arrow(point_to, angle, ARROW_SIZE, false);
}

fn draw_transition_with_text(
    from: &Vec2,
    to: &Vec2,
    to_state: bool,
    text: Vec<String>,
    gl: &mut QuadGl,
    font: &Font,
) {
    draw_transition(from, to, to_state);
    draw_transition_text(from, to, text, gl, font);
}

fn draw_transition_text(from: &Vec2, to: &Vec2, text: Vec<String>, gl: &mut QuadGl, font: &Font) {
    // Change coordinate systems to be centered on the middle of the transition,
    // and rotated parallel to the transition, then draw the text
    let angle = vec2(1., 0.).angle_between(*to - *from);
    let middle = from.lerp(*to, 0.5);
    let font_size = TRANSITION_FONT_SIZE * 5.;
    gl.push_model_matrix(glam::Mat4::from_translation(glam::vec3(
        middle.x, middle.y, 0.,
    )));
    let text_angle = angle
        + if angle > std::f32::consts::FRAC_PI_2 || angle < -std::f32::consts::FRAC_PI_2 {
            std::f32::consts::PI
        } else {
            0.
        };
    gl.push_model_matrix(glam::Mat4::from_rotation_z(text_angle));

    for (i, string) in text.iter().enumerate() {
        let text_size = measure_text(string, None, font_size as _, 0.2);
        draw_text_ex(
            string,
            -text_size.width / 2.,
            -(8. + 20. * i as f32),
            TextParams {
                font_size: font_size as _,
                font_scale: 0.2,
                font: *font,
                color: BLACK,
                ..Default::default()
            },
        );
    }

    // Reset the coordinate system
    gl.pop_model_matrix();
    gl.pop_model_matrix();
}

fn draw_self_transition(state_position: &Vec2) {
    let angle = std::f32::consts::FRAC_PI_2 - std::f32::consts::FRAC_PI_6;
    let point_from =
        *state_position + vec2(STATE_RADIUS * angle.cos(), -STATE_RADIUS * angle.sin());
    let point_to = *state_position + vec2(-STATE_RADIUS * angle.cos(), -STATE_RADIUS * angle.sin());
    let mut prev_point = point_from;
    let start = vec2(0., 120. * 0.25);
    for i in 0..=100 {
        let t = i as f32 * 0.01;
        let next_point = point_from.lerp(point_to, t) + vec2(0., 120. * (t - 0.5).powi(2)) - start;
        draw_line(
            prev_point.x,
            prev_point.y,
            next_point.x,
            next_point.y,
            2.,
            BLACK,
        );
        prev_point = next_point;
    }

    draw_arrow(
        point_from,
        std::f32::consts::FRAC_PI_2 - std::f32::consts::FRAC_PI_8,
        ARROW_SIZE,
        false,
    );
}

fn draw_self_transition_with_text(state_position: &Vec2, text: Vec<String>, font: &Font) {
    draw_self_transition(state_position);

    let font_size = TRANSITION_FONT_SIZE * 5.;
    for (i, string) in text.iter().enumerate() {
        let text_size = measure_text(string, None, font_size as _, 0.2);
        draw_text_ex(
            string,
            state_position.x - text_size.width / 2.,
            state_position.y - STATE_RADIUS - 32. - 20. * i as f32,
            TextParams {
                font_size: font_size as _,
                font_scale: 0.2,
                font: *font,
                color: BLACK,
                ..Default::default()
            },
        );
    }
}

#[macroquad::main("Sugarcubes")]
async fn main() {
    let mut fa = FiniteAutomaton::default();
    let state0 = fa.automaton.add_new_state();
    fa.automaton.set_initial(state0);
    let state1 = fa.automaton.add_new_state();
    let state2 = fa.automaton.add_new_state();
    fa.automaton.set_final(state2, true);

    fa.automaton
        .add_transition(FiniteAutomatonTransition::new(state0, state1, 'x'));
    fa.automaton
        .add_transition(FiniteAutomatonTransition::new(state0, state2, EMPTY_STRING));
    fa.automaton
        .add_transition(FiniteAutomatonTransition::new(state1, state1, 'a'));

    let mut configurations = fa.initial_configurations("xabc");

    let mut position_map = HashMap::new();
    position_map.insert(state0, vec2(200., 300.));
    position_map.insert(state1, vec2(400., 200.));
    position_map.insert(state2, vec2(400., 400.));

    let gl = unsafe { get_internal_gl().quad_gl };

    let font = load_ttf_font("./assets/OpenSans-Regular.ttf").await;

    let mut selected_state: Option<u32> = None;
    // The offset of the click relative to the center of the selected state,
    // so that the mouse "grabs" the state at the point of the initial click
    let mut state_drag_offset = Vec2::ZERO;

    // If the user is drawing a new transition starting on a state, its ID is in here
    let mut creating_transition_from: Option<u32> = None;

    let mut last_click_time = 0.;

    loop {
        clear_background(WHITE);

        // Process keys, mouse etc.
        // TODO: Ignore clicks on egui elements
        let mouse_position: Vec2 = mouse_position().into();
        if is_mouse_button_pressed(MouseButton::Left) {
            let new_click_time = get_time();
            // Check for double click
            if last_click_time > 0. && new_click_time - last_click_time <= DOUBLE_CLICK_DELAY {
                creating_transition_from = None;

                // Iterate in reverse, so the highest-numbered state is selected first
                for state in fa.automaton.states_iter().rev() {
                    let position = *position_map.get(state).unwrap();
                    if mouse_position.abs_diff_eq(position, STATE_RADIUS) {
                        creating_transition_from = Some(*state);
                        break;
                    }
                }

                if creating_transition_from.is_none() {
                    let new_state = fa.automaton.add_new_state();
                    position_map.insert(new_state, mouse_position);
                    selected_state = Some(new_state);
                    state_drag_offset = Vec2::ZERO;
                }
            } else {
                for state in fa.automaton.states_iter().rev() {
                    let position = *position_map.get(state).unwrap();
                    if mouse_position.abs_diff_eq(position, STATE_RADIUS) {
                        selected_state = Some(*state);
                        state_drag_offset = position - mouse_position;
                        break;
                    }
                }
            }
            last_click_time = new_click_time;
        }

        if is_mouse_button_released(MouseButton::Left) {
            selected_state = None;

            // If the user releases over a state while creating a transition,
            // connect the two states
            if let Some(from) = creating_transition_from {
                for &other in fa.automaton.states_iter().rev() {
                    let position = *position_map.get(&other).unwrap();
                    if mouse_position.abs_diff_eq(position, STATE_RADIUS) {
                        fa.automaton.add_transition(FiniteAutomatonTransition::new(
                            from,
                            other,
                            EMPTY_STRING, // TODO: Add a symbol (at an angle?)
                        ));
                        break;
                    }
                }
            }

            creating_transition_from = None;
        }

        if is_mouse_button_pressed(MouseButton::Right) {
            // TODO: Probably a context menu
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
            });
        });

        if should_step {
            configurations = fa.step_all(configurations);
        }

        // Draw things before egui
        for state in fa.automaton.states() {
            if selected_state == Some(*state) {
                position_map.insert(*state, mouse_position + state_drag_offset);
            }
        }

        // Draw states in order of increasing ID, so higher ID states are drawn on top
        for state in fa.automaton.states_iter() {
            let position = position_map.get(state).unwrap_or(&Vec2::ZERO);

            // TODO: Two states with transitions to each other need curved transitions

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
                    draw_self_transition_with_text(position, symbols, &font);
                } else {
                    let other_position = position_map.get(&other_state).unwrap_or(&Vec2::ZERO);
                    draw_transition_with_text(position, other_position, true, symbols, gl, &font)
                }
            }

            let state_color = if selected_state == Some(*state) {
                SELECTED_COLOR
            } else if configurations
                .iter()
                .any(|configuration| configuration.state() == *state)
            {
                ACTIVE_COLOR
            } else {
                INACTIVE_COLOR
            };
            draw_circle(position.x, position.y, STATE_RADIUS, state_color);
            draw_circle_lines(position.x, position.y, STATE_RADIUS + 0.5, 2., BLACK);

            if fa.automaton.initial() == Some(*state) {
                draw_arrow(
                    vec2(position.x - STATE_RADIUS, position.y),
                    0.,
                    INITIAL_ARROW_SIZE,
                    true,
                );
            }

            let text = &state.to_string();
            let text_size = measure_text(text, None, STATE_FONT_SIZE as _, 1.0);
            draw_text_ex(
                &state.to_string(),
                position.x - text_size.width / 2.,
                position.y - text_size.height / 2. + STATE_RADIUS / 2.,
                TextParams {
                    font_size: STATE_FONT_SIZE as _,
                    font,
                    color: BLACK,
                    ..Default::default()
                },
            );
        }

        if let Some(from) = creating_transition_from {
            let position = position_map.get(&from).unwrap_or(&Vec2::ZERO);
            if mouse_position.abs_diff_eq(*position, STATE_RADIUS) {
                draw_self_transition(position);
            } else {
                draw_transition(position, &mouse_position, false);
            }
        }

        egui_macroquad::draw();

        // Draw things after egui

        next_frame().await;
    }
}
