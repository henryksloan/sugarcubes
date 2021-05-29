use sugarcubes_core::automata::{
    finite_automaton::{FiniteAutomaton, FiniteAutomatonTransition},
    Transition, EMPTY_STRING,
};

use macroquad::prelude::*;
use std::collections::HashMap;

// Draw an arrow with its tip at a given point,
// at a given angle relative to the horizontal,
// and with a given sidelength
fn draw_arrow(point: (f32, f32), angle: f32, size: f32, outlined: bool) {
    let a = (angle + std::f32::consts::FRAC_PI_6).cos() * size;
    let b = (angle + std::f32::consts::FRAC_PI_6).sin() * size;
    let c = (angle - std::f32::consts::FRAC_PI_6).cos() * size;
    let d = (angle - std::f32::consts::FRAC_PI_6).sin() * size;
    let v1 = Vec2::new(point.0, point.1);
    let v2 = Vec2::new(point.0 - a, point.1 - b);
    let v3 = Vec2::new(point.0 - c, point.1 - d);

    if outlined {
        draw_triangle(v1, v2, v3, WHITE);
        draw_triangle_lines(v1, v2, v3, 2., BLACK);
    } else {
        draw_triangle(v1, v2, v3, BLACK);
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

    let mut position_map = HashMap::new();
    position_map.insert(state0, (200., 300.));
    position_map.insert(state1, (400., 200.));
    position_map.insert(state2, (400., 400.));

    let radius = 35.;
    let initial_arrow_size = 24.;
    let arrow_size = 17.;
    let state_color = Color::from_rgba(232, 237, 133, 255);

    loop {
        clear_background(WHITE);

        // Process keys, mouse etc.

        egui_macroquad::ui(|egui_ctx| {
            egui::Window::new("egui ❤ macroquad").show(egui_ctx, |ui| {
                ui.label("Test");
            });
        });

        // Draw things before egui
        for state in fa.automaton.states() {
            let position = position_map.get(state).unwrap_or(&(0., 0.));

            for transition in fa.automaton.transitions_from(*state) {
                let other_position = position_map.get(&transition.to()).unwrap_or(&(0., 0.));
                let angle = ((other_position.1 as f32 - position.1)
                    / (other_position.0 - position.0))
                    .atan();
                let x_off = angle.cos() * radius;
                let y_off = angle.sin() * radius;
                let point_from = (position.0 + x_off, position.1 + y_off);
                let point_to = (other_position.0 - x_off, other_position.1 - y_off);
                draw_line(
                    point_from.0,
                    point_from.1,
                    point_to.0,
                    point_to.1,
                    2.,
                    BLACK,
                );

                draw_arrow(point_to, angle, arrow_size, false);
            }

            draw_circle(position.0, position.1, radius, state_color);
            draw_circle_lines(position.0, position.1, radius + 0.5, 2., BLACK);

            if fa.automaton.initial() == Some(*state) {
                draw_arrow(
                    (position.0 - radius, position.1),
                    0.,
                    initial_arrow_size,
                    true,
                );
            }

            let text = &state.to_string();
            let font_size = 30.;
            let text_size = measure_text(text, None, font_size as _, 1.0);
            draw_text(
                &state.to_string(),
                position.0 - text_size.width / 2.,
                position.1 - text_size.height / 2. + radius / 2.,
                font_size,
                BLACK,
            );
        }

        egui_macroquad::draw();

        // Draw things after egui

        next_frame().await;
    }
}
