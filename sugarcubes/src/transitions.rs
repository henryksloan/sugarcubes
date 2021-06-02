use crate::states::STATE_RADIUS;

use macroquad::prelude::*;

pub const ARROW_SIZE: f32 = 17.;
pub const TRANSITION_FONT_SIZE: f64 = 24.;

// The number of steps used to approximate curves
const TRANSITION_POINTS: usize = 40;
const SELF_TRANSITION_POINTS: usize = 20;

// Draw an arrow with its tip at a given point,
// at a given angle relative to the horizontal,
// and with a given sidelength
pub fn draw_arrow(point: Vec2, angle: f32, size: f32, outlined: bool) {
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

pub fn draw_transition(from: &Vec2, to: &Vec2, to_state: bool) {
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

pub fn draw_transition_with_text(
    from: &Vec2,
    to: &Vec2,
    to_state: bool,
    text: Vec<String>,
    gl: &mut QuadGl,
    font: &Font,
) {
    draw_transition(from, to, to_state);
    draw_transition_text(from, to, false, false, text, gl, font);
}

pub fn draw_transition_text(
    from: &Vec2,
    to: &Vec2,
    curved: bool,
    down: bool,
    text: Vec<String>,
    gl: &mut QuadGl,
    font: &Font,
) {
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

    let direction = if down { 1. } else { -1. };
    let initial_y_offset = if curved { 32. } else { 8. } + (direction + 1.) * 4.;
    for (i, string) in text.iter().enumerate() {
        let text_size = measure_text(string, None, font_size as _, 0.2);
        draw_text_ex(
            string,
            -text_size.width / 2.,
            direction * (initial_y_offset + 23. * i as f32),
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

pub fn draw_curved_transition(from: &Vec2, to: &Vec2) {
    let angle = vec2(1., 0.).angle_between(*to - *from);
    let angle_on_state = 0.12;
    let angle_from = angle - angle_on_state;
    let angle_to = angle + angle_on_state;
    let point_from = *from
        + vec2(
            STATE_RADIUS * angle_from.cos(),
            STATE_RADIUS * angle_from.sin(),
        );
    let point_to = *to - vec2(STATE_RADIUS * angle_to.cos(), STATE_RADIUS * angle_to.sin());
    let mut prev_point = point_from;
    let start = vec2(
        (angle + std::f32::consts::FRAC_PI_2).cos(),
        (angle + std::f32::consts::FRAC_PI_2).sin(),
    ) * (80. * 0.25);
    let mut final_angle = 0.;
    for i in 0..=TRANSITION_POINTS {
        let t = i as f32 / (TRANSITION_POINTS as f32);
        let y = 80. * (t - 0.5).powi(2);
        let next_point = point_from.lerp(point_to, t)
            + vec2(
                y * (angle + std::f32::consts::FRAC_PI_2).cos(),
                y * (angle + std::f32::consts::FRAC_PI_2).sin(),
            )
            - start;
        draw_line(
            prev_point.x,
            prev_point.y,
            next_point.x,
            next_point.y,
            2.,
            BLACK,
        );

        if i == TRANSITION_POINTS {
            final_angle = vec2(1., 0.).angle_between(next_point - prev_point);
        }

        prev_point = next_point;
    }
    draw_arrow(point_to, final_angle, ARROW_SIZE, false);
}

pub fn draw_curved_transition_with_text(
    from: &Vec2,
    to: &Vec2,
    text: Vec<String>,
    gl: &mut QuadGl,
    font: &Font,
) {
    let down = (vec2(1., 0.).angle_between(*to - *from)).abs() > std::f32::consts::FRAC_PI_2;
    draw_curved_transition(from, to);
    draw_transition_text(from, to, true, down, text, gl, font);
}

pub fn draw_self_transition(state_position: &Vec2) {
    let angle = std::f32::consts::FRAC_PI_2 - std::f32::consts::FRAC_PI_6;
    let point_from =
        *state_position + vec2(STATE_RADIUS * angle.cos(), -STATE_RADIUS * angle.sin());
    let point_to = *state_position + vec2(-STATE_RADIUS * angle.cos(), -STATE_RADIUS * angle.sin());
    let mut prev_point = point_from;
    let start = vec2(0., 120. * 0.25);
    for i in 0..=SELF_TRANSITION_POINTS {
        let t = i as f32 / (SELF_TRANSITION_POINTS as f32);
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

pub fn draw_self_transition_with_text(state_position: &Vec2, text: Vec<String>, font: &Font) {
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
