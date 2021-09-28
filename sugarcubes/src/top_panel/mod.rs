mod context_menu;
mod fast_run;
mod input_window;
mod left_panel;
mod menu_bar;
mod set_name;
mod simulate_input_window;
mod simulation_toolbar;

use input_window::InputWindow;

use crate::{command::*, states::*};

use sugarcubes_core::automata::finite_automaton::{
    FiniteAutomaton, FiniteAutomatonConfiguration, FiniteAutomatonTransition,
};

use macroquad::prelude::*;

use std::cell::RefCell;

pub const ACCEPT_COLOR: egui::Color32 = egui::Color32::from_rgb(122, 240, 98);
pub const REJECT_COLOR: egui::Color32 = egui::Color32::RED;

thread_local! { pub static TOP_PANEL: RefCell<TopPanel> = RefCell::new(TopPanel::new()); }

#[derive(Copy, Clone)]
pub enum Mode {
    Edit,
    Simulate,
    MultipleRun,
}

pub enum TopPanelCommand {
    Command(Command),
    Undo,
    Redo,
    Step,
    StartSimulation(Vec<FiniteAutomatonConfiguration>),
}

pub struct TopPanel {
    pub width: f32,
    pub height: f32,
    pub mode: Mode,
    pub contains_mouse: bool,
    pub open_context_menu: bool,
    pub context_menu_pos: Vec2,

    simulate_input_window: InputWindow,
    string_simulating: String,

    fast_run_input_window: InputWindow,
    fast_run_string: String,
    fast_run_result: Option<bool>,

    multiple_run_strings: Vec<(String, Option<bool>)>,
    multiple_run_selected_index: Option<usize>,

    set_name_input_window: InputWindow,
    set_name_state_id: Option<u32>,
}

impl TopPanel {
    pub fn new() -> Self {
        Self {
            width: 0.,
            height: 0.,
            mode: Mode::Edit,
            contains_mouse: false,
            open_context_menu: false,
            context_menu_pos: Vec2::ZERO,

            simulate_input_window: InputWindow::new("simulate"),
            string_simulating: String::new(),

            fast_run_input_window: InputWindow::new("fast_run"),
            fast_run_string: String::new(),
            fast_run_result: None,

            multiple_run_strings: vec![(String::new(), None)],
            multiple_run_selected_index: None,

            set_name_input_window: InputWindow::new("set_name"),
            set_name_state_id: None,
        }
    }

    pub fn ui(
        &mut self,
        fa: &FiniteAutomaton,
        states: &mut States,
        configurations: &mut Vec<FiniteAutomatonConfiguration>,
        mouse_position: &Vec2,
        selected_state: &mut Option<u32>,
        selected_transition: &mut Option<FiniteAutomatonTransition>,
        can_undo: bool,
        can_redo: bool,
    ) -> Option<TopPanelCommand> {
        self.contains_mouse = false;

        let mut command = None;

        egui_macroquad::ui(|egui_ctx| {
            let mut fonts = egui::FontDefinitions::default();
            fonts.family_and_size.insert(
                egui::TextStyle::Button,
                (egui::FontFamily::Proportional, 18.),
            );
            fonts
                .family_and_size
                .insert(egui::TextStyle::Body, (egui::FontFamily::Proportional, 19.));
            egui_ctx.set_fonts(fonts);

            let top_panel_command =
                self.top_panel(egui_ctx, fa, configurations, can_undo, can_redo);
            if let Some(top_panel_command) = top_panel_command {
                command = Some(top_panel_command);
            }

            if let Mode::MultipleRun = self.mode {
                self.left_panel(egui_ctx, fa);
            } else {
                self.width = 0.;
            }

            let context_menu_command = self.context_menu(
                egui_ctx,
                fa,
                states,
                mouse_position,
                selected_state,
                selected_transition,
            );

            if let Some(context_menu_command) = context_menu_command {
                command = Some(TopPanelCommand::Command(context_menu_command));
            }

            if self.simulate_input_window.open {
                let simulate_input_command = self.show_simulate_input_window(egui_ctx, fa);
                if let Some(simulate_input_command) = simulate_input_command {
                    command = Some(simulate_input_command);
                }
            }

            if self.fast_run_input_window.open {
                self.show_fast_run_input_window(egui_ctx, fa);
            }

            if let Some(fast_run_result) = self.fast_run_result {
                self.show_fast_run_result_window(egui_ctx, fast_run_result);
            }

            if self.set_name_input_window.open {
                let set_name_input_command = self.show_set_name_input_window(egui_ctx, states);
                if let Some(set_name_input_command) = set_name_input_command {
                    command = Some(set_name_input_command);
                }
            }
        });

        command
    }

    fn top_panel(
        &mut self,
        egui_ctx: &egui::CtxRef,
        fa: &FiniteAutomaton,
        configurations: &mut Vec<FiniteAutomatonConfiguration>,
        can_undo: bool,
        can_redo: bool,
    ) -> Option<TopPanelCommand> {
        let mut command = None;

        egui::TopBottomPanel::top("top_panel").show(egui_ctx, |ui| {
            let menu_bar_command = self.menu_bar(ui, can_undo, can_redo);
            if let Some(menu_bar_command) = menu_bar_command {
                command = Some(menu_bar_command);
            }

            if let Mode::Simulate = self.mode {
                ui.separator();

                let simulation_toolbar_command = self.simulation_toolbar(ui, fa, configurations);
                if let Some(simulation_toolbar_command) = simulation_toolbar_command {
                    command = Some(simulation_toolbar_command);
                }
            }

            self.contains_mouse = ui.ui_contains_pointer();
            self.height = ui.max_rect().height();
        });

        command
    }
}
