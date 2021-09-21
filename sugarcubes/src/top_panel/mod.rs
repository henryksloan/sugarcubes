mod input_window;

use input_window::InputWindow;

use crate::{command::*, states::*};

use sugarcubes_core::automata::{
    finite_automaton::{FiniteAutomaton, FiniteAutomatonConfiguration, FiniteAutomatonTransition},
    Configuration, SimulateAutomaton, EMPTY_STRING,
};

use macroquad::prelude::*;
use sapp_jsutils::JsObject;

use std::cell::RefCell;

const CONFIGURATION_HEIGHT: f32 = 60.;
pub const ACCEPT_COLOR: egui::Color32 = egui::Color32::from_rgb(122, 240, 98);
pub const REJECT_COLOR: egui::Color32 = egui::Color32::RED;

thread_local! { pub static TOP_PANEL: RefCell<TopPanel> = RefCell::new(TopPanel::new()); }

#[cfg(target_arch = "wasm32")]
extern "C" {
    fn choose_multiple_run_file();
}

#[cfg(not(target_arch = "wasm32"))]
unsafe fn choose_multiple_run_file() {}

#[no_mangle]
extern "C" fn read_multiple_run_inputs(content: JsObject) {
    let mut content_string = String::new();
    content.to_string(&mut content_string);
    TOP_PANEL.with(|panel| {
        if let Ok(mut panel) = panel.try_borrow_mut() {
            panel.load_multiple_run_inputs(content_string)
        }
    });
}

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

    fn menu_bar(
        &mut self,
        ui: &mut egui::Ui,
        can_undo: bool,
        can_redo: bool,
    ) -> Option<TopPanelCommand> {
        let mut command = None;

        egui::menu::bar(ui, |ui| {
            egui::menu::menu(ui, "File", |ui| {
                if ui.button("Open...").clicked() {
                    // ...
                }
            });

            egui::menu::menu(ui, "Edit", |ui| {
                let undo_button = egui::widgets::Button::new("Undo").enabled(can_undo);
                if ui.add(undo_button).clicked() {
                    command = Some(TopPanelCommand::Undo);
                }

                let redo_button = egui::widgets::Button::new("Redo").enabled(can_redo);
                if ui.add(redo_button).clicked() {
                    command = Some(TopPanelCommand::Redo);
                }
            });

            egui::menu::menu(ui, "Simulate", |ui| {
                if ui.button("Simulate String...").clicked() {
                    self.simulate_input_window.open = true;
                }

                if ui.button("Fast Run...").clicked() {
                    self.fast_run_input_window.open = true;
                }

                if ui.button("Multiple Run...").clicked() {
                    for pair in self.multiple_run_strings.iter_mut() {
                        pair.1 = None;
                    }
                    self.mode = Mode::MultipleRun;
                    self.multiple_run_selected_index = None;
                }
            });
        });

        command
    }

    fn simulation_toolbar(
        &mut self,
        ui: &mut egui::Ui,
        fa: &FiniteAutomaton,
        configurations: &mut Vec<FiniteAutomatonConfiguration>,
    ) -> Option<TopPanelCommand> {
        let mut command = None;

        ui.horizontal(|ui| {
            if ui.button("X").clicked() {
                self.mode = Mode::Edit;
            }

            ui.vertical(|ui| {
                ui.add(
                    egui::Label::new(format!("Simulating \"{}\"", self.string_simulating))
                        .heading(),
                );

                ui.separator();

                ui.horizontal(|ui| {
                    ui.set_min_height(CONFIGURATION_HEIGHT);
                    for configuration in configurations {
                        let config_exhausted = configuration.remaining_string.is_empty()
                            && !fa
                                .automaton
                                .transitions_from(configuration.state())
                                .into_iter()
                                .any(|&transition| transition.symbol() == EMPTY_STRING);

                        let (fill, text_color, message) = if config_exhausted {
                            if fa.automaton.is_final(configuration.state()) {
                                (
                                    Some(egui::Color32::from_rgb(122, 240, 98)),
                                    egui::Color32::BLACK,
                                    "accept",
                                )
                            } else {
                                (Some(egui::Color32::RED), egui::Color32::WHITE, "reject")
                            }
                        } else {
                            (
                                None,
                                egui::Color32::WHITE,
                                configuration.remaining_string.as_str(),
                            )
                        };

                        let mut button = egui::Button::new(format!(
                            "{}\n{}",
                            configuration.state().to_string(),
                            message,
                        ))
                        .text_color(text_color)
                        .text_style(egui::TextStyle::Heading);

                        if let Some(fill) = fill {
                            button = button.fill(fill);
                        }

                        if ui.add_sized([75., CONFIGURATION_HEIGHT], button).clicked() {}
                    }
                });

                ui.separator();

                ui.horizontal(|ui| {
                    if ui.button("Step").clicked() {
                        command = Some(TopPanelCommand::Step);
                    }

                    if ui.button("Reset").clicked() {
                        command = Some(TopPanelCommand::StartSimulation(
                            fa.initial_configurations(&self.string_simulating),
                        ))
                    }
                });
            });
        });

        command
    }

    fn left_panel(&mut self, egui_ctx: &egui::CtxRef, fa: &FiniteAutomaton) {
        egui::SidePanel::left("multiple_run")
            .resizable(false)
            .show(egui_ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.with_layout(egui::Layout::left_to_right(), |ui| {
                        ui.heading("Multiple Run");
                    });
                    ui.with_layout(egui::Layout::right_to_left(), |ui| {
                        if ui.button("X").clicked() {
                            self.mode = Mode::Edit;
                        }
                    });
                });

                ui.separator();

                egui::ScrollArea::auto_sized().show(ui, |ui| {
                    let selected_index = self.multiple_run_selected_index;
                    let mut add_new_line = false;
                    let mut new_selected_index = selected_index;
                    let num_strings = self.multiple_run_strings.len();
                    for (i, (text, status)) in self.multiple_run_strings.iter_mut().enumerate() {
                        ui.horizontal(|ui| {
                            let text_edit = ui.add(egui::TextEdit::singleline(text));

                            if text_edit.lost_focus() {
                                if ui.input().key_pressed(egui::Key::Enter) {
                                    if i == num_strings - 1 && !text.is_empty() {
                                        add_new_line = true;
                                        new_selected_index = Some(i + 1);
                                    } else {
                                        new_selected_index = Some((i + 1) % num_strings);
                                    }
                                } else {
                                    new_selected_index = None;
                                }
                            }

                            if let Some(selected_index) = selected_index {
                                // If this index is selected, and hasn't been clicked off of
                                if selected_index == i && new_selected_index.is_some() {
                                    text_edit.request_focus();
                                }
                            }

                            let label = match status {
                                None => "⛶",
                                Some(false) => "🗙",
                                Some(true) => "✔",
                            };
                            ui.add(egui::Label::new(label));
                        });
                    }

                    self.multiple_run_selected_index = new_selected_index;

                    // "Enter" was pressed on the last TextEdit, and it was empty
                    if add_new_line {
                        self.multiple_run_strings.push((String::new(), None));
                    }

                    ui.horizontal(|ui| {
                        if ui.button("Run").clicked() {
                            for (text, status) in self.multiple_run_strings.iter_mut() {
                                *status = Some(fa.check_input(text));
                            }

                            // If the last string is empty, discard the result,
                            // as it is most likely just an extra blank line, not a user's query
                            if let Some(mut last_string) = self.multiple_run_strings.last_mut() {
                                if last_string.0.is_empty() {
                                    last_string.1 = None;
                                }
                            }
                        }

                        if ui.button("Load from file").clicked() {
                            unsafe {
                                choose_multiple_run_file();
                            }
                        }

                        if ui.button("Clear").clicked() {
                            self.multiple_run_strings = vec![(String::new(), None)];
                        }
                    });
                });

                self.width = ui.max_rect().width();
            });
    }

    fn show_simulate_input_window(
        &mut self,
        egui_ctx: &egui::CtxRef,
        fa: &FiniteAutomaton,
    ) -> Option<TopPanelCommand> {
        let mut command = None;

        let mut new_configurations = None;
        let (hit_ok, contains_mouse) = self.simulate_input_window.show(egui_ctx);
        self.contains_mouse |= contains_mouse;

        if hit_ok {
            new_configurations = Some(fa.initial_configurations(&self.simulate_input_window.input));
            self.mode = Mode::Simulate;
            self.string_simulating = self.simulate_input_window.input.clone();
            self.simulate_input_window.open = false;
        }

        if let Some(new_configurations) = new_configurations {
            command = Some(TopPanelCommand::StartSimulation(new_configurations));
        }

        if !self.simulate_input_window.open {
            self.simulate_input_window.input.clear();
        }

        command
    }

    fn show_fast_run_input_window(&mut self, egui_ctx: &egui::CtxRef, fa: &FiniteAutomaton) {
        let (hit_ok, contains_mouse) = self.fast_run_input_window.show(egui_ctx);
        self.contains_mouse |= contains_mouse;

        if hit_ok {
            self.fast_run_input_window.open = false;
            self.fast_run_string = self.fast_run_input_window.input.clone();
            self.fast_run_result = Some(fa.check_input(&self.fast_run_input_window.input));
        }

        if !self.fast_run_input_window.open {
            self.fast_run_input_window.input.clear();
        }
    }

    fn show_fast_run_result_window(&mut self, egui_ctx: &egui::CtxRef, fast_run_result: bool) {
        let mut result_open = true;
        egui::Window::new("Fast Run Result")
            .open(&mut result_open)
            .resizable(false)
            .collapsible(false)
            .show(egui_ctx, |ui| {
                ui.horizontal(|ui| {
                    ui.label(format!("Result for string \"{}\": ", self.fast_run_string));
                    if fast_run_result {
                        ui.add(egui::widgets::Label::new("Accepted").text_color(ACCEPT_COLOR));
                    } else {
                        ui.add(egui::widgets::Label::new("Rejected").text_color(REJECT_COLOR));
                    }
                });
            });
        if !result_open {
            self.fast_run_result = None;
        }
    }

    fn context_menu(
        &mut self,
        egui_ctx: &egui::CtxRef,
        fa: &FiniteAutomaton,
        states: &mut States,
        mouse_position: &Vec2,
        selected_state: &mut Option<u32>,
        selected_transition: &mut Option<FiniteAutomatonTransition>,
    ) -> Option<Command> {
        let mut command = None;

        egui::Area::new("context_menu").show(egui_ctx, |ui| {
            let popup_id = ui.make_persistent_id("context_menu_id");
            if self.open_context_menu {
                ui.memory().open_popup(popup_id);
                self.open_context_menu = false;
            }
            let mut mouse_in_popup = false;
            if ui.memory().is_popup_open(popup_id) {
                let parent_clip_rect = ui.clip_rect();

                egui::Area::new(popup_id)
                    .order(egui::Order::Foreground)
                    .fixed_pos((self.context_menu_pos.x, self.context_menu_pos.y))
                    .show(ui.ctx(), |ui| {
                        ui.set_clip_rect(parent_clip_rect);
                        let frame = egui::Frame::popup(ui.style());
                        let frame_margin = frame.margin;
                        frame.show(ui, |ui| {
                            ui.with_layout(
                                egui::Layout::top_down_justified(egui::Align::LEFT),
                                |ui| {
                                    ui.set_width(100.0 - 2.0 * frame_margin.x);
                                    if let Some(selected) = *selected_state {
                                        let mut is_initial =
                                            fa.automaton.initial() == Some(selected);
                                        if ui.checkbox(&mut is_initial, "Initial").changed() {
                                            if is_initial {
                                                command = Some(Command::SetInitial(
                                                    selected,
                                                    fa.automaton.initial(),
                                                ));
                                            } else {
                                                command = Some(Command::RemoveInitial(selected));
                                            }
                                            *selected_state = None;
                                            ui.memory().close_popup();
                                        }

                                        let mut is_final = fa.automaton.is_final(selected);
                                        if ui.checkbox(&mut is_final, "Final").changed() {
                                            command = Some(Command::SetFinal(selected, is_final));
                                            *selected_state = None;
                                            ui.memory().close_popup();
                                        }

                                        ui.separator();

                                        if ui.button("Delete").clicked() {
                                            command = Some(Command::DeleteState(
                                                selected,
                                                *states.get_position(selected),
                                                fa.automaton
                                                    .transitions_with(selected)
                                                    .into_iter()
                                                    .cloned()
                                                    .collect(),
                                            ));
                                            *selected_state = None;
                                            ui.memory().close_popup();
                                        }
                                    } else if let Some(selected) = *selected_transition {
                                        if ui.button("Delete").clicked() {
                                            command = Some(Command::DeleteTransition(selected));
                                            *selected_transition = None;
                                            ui.memory().close_popup();
                                        }
                                    }

                                    mouse_in_popup = ui.ui_contains_pointer();
                                    self.contains_mouse |= mouse_in_popup;
                                },
                            );
                        });
                    });

                if ui.input().key_pressed(egui::Key::Escape) {
                    ui.memory().close_popup();
                    *selected_state = None;
                } else if is_mouse_button_pressed(MouseButton::Left) && !mouse_in_popup {
                    ui.memory().close_popup();

                    // Clear selected state if the cancelling click is not in the selected state
                    if let Some(selected) = *selected_state {
                        if !states.point_in_state(*mouse_position, selected) {
                            *selected_state = None;
                        }
                    }
                }

                self.contains_mouse |= ui.ui_contains_pointer();
            }
        });

        command
    }

    fn load_multiple_run_inputs(&mut self, content_string: String) {
        self.multiple_run_strings = content_string
            .lines()
            .map(|line| (line.to_string(), None))
            .collect();
        if self.multiple_run_strings.is_empty() {
            self.multiple_run_strings.push((String::new(), None));
        }
    }
}
