use egui::CursorPair;

pub struct InputWindow {
    id: egui::Id,
    pub open: bool,
    pub input: String,
    // If set to on, move cursor to the end of the input on next call to show()
    pub end_of_line: bool,
}

impl InputWindow {
    pub fn new(id: &str) -> Self {
        Self {
            id: egui::Id::new(id),
            open: false,
            input: String::new(),
            end_of_line: false,
        }
    }

    pub fn show(&mut self, egui_ctx: &egui::CtxRef) -> (bool, bool) {
        let mut hit_ok = false;
        let mut contains_mouse = false;

        let mut window_open = true;
        let response = egui::Window::new("Input string")
            .id(self.id)
            .open(&mut window_open)
            .resizable(false)
            .collapsible(false)
            .title_bar(true)
            .show(egui_ctx, |ui| {
                // TODO: Checking for lost_focus AND enter pressed no longer works; fix that
                //     || (text_edit.lost_focus() && ui.input().key_pressed(egui::Key::Enter))
                let text_edit_id = self.id.with("input");
                let text_edit =
                    ui.add(egui::TextEdit::singleline(&mut self.input).id(text_edit_id));
                text_edit.request_focus();

                if self.end_of_line {
                    self.end_of_line = false;

                    #[derive(Clone, Debug, Default)]
                    struct State {
                        cursorp: Option<CursorPair>,
                    }

                    let mut text_edit_state = ui
                        .memory()
                        .id_data
                        .get_or_default::<State>(text_edit_id)
                        .clone();
                    let text_style = ui
                        .style()
                        .override_text_style
                        .unwrap_or_else(|| ui.style().body_text_style);
                    let galley = ui
                        .fonts()
                        .layout_single_line(text_style, self.input.clone());
                    if let Some(cursorp) = &mut text_edit_state.cursorp {
                        (*cursorp).primary = galley.cursor_end_of_row(&cursorp.primary);
                    }
                }

                ui.horizontal(|ui| {
                    if ui.button("Ok").clicked() || ui.input().key_pressed(egui::Key::Enter) {
                        hit_ok = true;
                    }

                    if ui.button("Cancel").clicked() || ui.input().key_pressed(egui::Key::Escape) {
                        self.open = false;
                    }
                });
            });
        if !window_open {
            self.open = false;
        }

        if let Some(inner_response) = response {
            contains_mouse |= inner_response.response.hovered();
        }

        (hit_ok, contains_mouse)
    }
}
