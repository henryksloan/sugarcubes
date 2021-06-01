use sugarcubes_core::automata::{
    finite_automaton::{FiniteAutomaton, FiniteAutomatonTransition},
    Configuration, SimulateAutomaton, Transition, EMPTY_STRING,
};

use macroquad::prelude::*;
use std::collections::HashMap;

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

const INACTIVE_COLOR: Color = Color::new(0.90, 0.93, 0.52, 1.00);
const ACTIVE_COLOR: Color = Color::new(0.44, 0.45, 0.19, 1.00);
const SELECTED_COLOR: Color = Color::new(0.45, 0.58, 0.81, 1.00);

const DOUBLE_CLICK_DELAY: f64 = 0.25;

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

    let mut configurations = fa.initial_configurations("xabc");

    let mut position_map = HashMap::new();
    position_map.insert(state0, vec2(200., 300.));
    position_map.insert(state1, vec2(400., 200.));
    position_map.insert(state2, vec2(400., 400.));

    let radius = 35.;
    let initial_arrow_size = 24.;
    let arrow_size = 17.;

    let gl = unsafe { get_internal_gl().quad_gl };

    let font = load_ttf_font("./assets/OpenSans-Regular.ttf").await;

    let mut selected_state: Option<u32> = None;
    // The offset of the click relative to the center of the selected state,
    // so that the mouse "grabs" the state at the point of the initial click
    let mut state_drag_offset = Vec2::ZERO;

    let mut last_click_time = 0.;

    loop {
        clear_background(WHITE);

        // Process keys, mouse etc.
        let mouse_position: Vec2 = mouse_position().into();
        if is_mouse_button_pressed(MouseButton::Left) {
            let new_click_time = get_time();
            // Check for double click
            if last_click_time > 0. && new_click_time - last_click_time <= DOUBLE_CLICK_DELAY {
                let mut state_double_clicked_on = None;
                // Iterate in reverse, so the highest-numbered state is selected first
                for state in fa.automaton.states_iter().rev() {
                    let position = *position_map.get(state).unwrap();
                    if mouse_position.abs_diff_eq(position, radius) {
                        state_double_clicked_on = Some(*state);
                        break;
                    }
                }

                if state_double_clicked_on.is_none() {
                    let new_state = fa.automaton.add_new_state();
                    position_map.insert(new_state, mouse_position);
                    selected_state = Some(new_state);
                    state_drag_offset = Vec2::ZERO;
                }
            } else {
                for state in fa.automaton.states_iter().rev() {
                    let position = *position_map.get(state).unwrap();
                    if mouse_position.abs_diff_eq(position, radius) {
                        selected_state = Some(*state);
                        state_drag_offset = position - mouse_position;
                        break;
                    }
                }
            }
            last_click_time = new_click_time;
        } else if !is_mouse_button_down(MouseButton::Left) {
            selected_state = None;
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

            for transition in fa.automaton.transitions_from(*state) {
                let other_position = position_map.get(&transition.to()).unwrap_or(&Vec2::ZERO);
                let angle = vec2(1., 0.).angle_between(*other_position - *position);
                let distance = position.distance(*other_position);
                let radius_over_distance = radius / distance;
                let point_from = position.lerp(*other_position, radius_over_distance);
                let point_to = other_position.lerp(*position, radius_over_distance);
                draw_line(
                    point_from.x,
                    point_from.y,
                    point_to.x,
                    point_to.y,
                    2.,
                    BLACK,
                );

                draw_arrow(point_to, angle, arrow_size, false);

                // Change coordinate systems to be centered on the middle of the transition,
                // and rotated parallel to the transition, then draw the text
                let middle = position.lerp(*other_position, 0.5);
                let font_size = 120.;
                let symbol_str = &transition.symbol().to_string();
                let text_size = measure_text(symbol_str, None, font_size as _, 0.2);
                gl.push_model_matrix(glam::Mat4::from_translation(glam::vec3(
                    middle.x, middle.y, 0.,
                )));
                let text_angle = angle
                    + if angle > std::f32::consts::FRAC_PI_2 || angle < -std::f32::consts::FRAC_PI_2
                    {
                        std::f32::consts::PI
                    } else {
                        0.
                    };
                gl.push_model_matrix(glam::Mat4::from_rotation_z(text_angle));

                draw_text_ex(
                    symbol_str,
                    -text_size.width / 2.,
                    -8.,
                    TextParams {
                        font_size: font_size as _,
                        font_scale: 0.2,
                        font,
                        color: BLACK,
                        ..Default::default()
                    },
                );

                // Reset the coordinate system
                gl.pop_model_matrix();
                gl.pop_model_matrix();
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
            draw_circle(position.x, position.y, radius, state_color);
            draw_circle_lines(position.x, position.y, radius + 0.5, 2., BLACK);

            if fa.automaton.initial() == Some(*state) {
                draw_arrow(
                    vec2(position.x - radius, position.y),
                    0.,
                    initial_arrow_size,
                    true,
                );
            }

            let text = &state.to_string();
            let font_size = 30.;
            let text_size = measure_text(text, None, font_size as _, 1.0);
            draw_text_ex(
                &state.to_string(),
                position.x - text_size.width / 2.,
                position.y - text_size.height / 2. + radius / 2.,
                TextParams {
                    font_size: font_size as _,
                    font,
                    color: BLACK,
                    ..Default::default()
                },
            );
        }

        egui_macroquad::draw();

        // Draw things after egui

        next_frame().await;
    }
}
