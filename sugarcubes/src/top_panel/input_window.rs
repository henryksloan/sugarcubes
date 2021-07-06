pub struct InputWindow {
    pub open: bool,
    pub input: String,
}

impl InputWindow {
    pub fn new() -> Self {
        Self {
            open: false,
            input: String::new(),
        }
    }

    pub fn show(&mut self, egui_ctx: &egui::CtxRef) -> (bool, bool) {
        let mut hit_ok = false;
        let mut contains_mouse = false;

        let mut window_open = true;
        let response = egui::Window::new("Input string")
            .open(&mut window_open)
            .resizable(false)
            .collapsible(false)
            .title_bar(true)
            .show(egui_ctx, |ui| {
                // TODO: Checking for lost_focus AND enter pressed no longer works; fix that
                //     || (text_edit.lost_focus() && ui.input().key_pressed(egui::Key::Enter))
                let text_edit = ui.add(egui::TextEdit::singleline(&mut self.input));
                text_edit.request_focus();

                ui.horizontal(|ui| {
                    if ui.button("Ok").clicked() || ui.input().key_pressed(egui::Key::Enter) {
                        hit_ok = true;
                    }

                    if ui.button("Cancel").clicked() {
                        self.open = false;
                    }
                });
            });
        if !window_open {
            self.open = false;
        }

        if let Some(response) = response {
            contains_mouse |= response.hovered();
        }

        (hit_ok, contains_mouse)
    }
}
