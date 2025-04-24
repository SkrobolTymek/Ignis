use eframe::egui;
use crate::{commands::CommandHandler, completer::IgnisCompleter, pty::Pty};

pub struct Terminal {
    input: String,
    history: Vec<String>,
    cursor_pos: usize,
    completer: IgnisCompleter,
    show_completions: bool,
    #[cfg(unix)] pty: Option<Pty>,  // Only include on Unix
}

impl Terminal {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            input: String::new(),
            history: vec!["Welcome to Ignis Terminal".to_string()],
            cursor_pos: 0,
            completer: IgnisCompleter::new(),
            show_completions: false,
            #[cfg(unix)] pty: match Pty::new() {
                Ok(pty) => Some(pty),
                Err(e) => {
                    eprintln!("Failed to initialize PTY: {}", e);
                    None
                }
            },
        })
    }

    pub fn show(&mut self, ui: &mut egui::Ui, command_handler: &mut CommandHandler) {
        // Display command history
        egui::ScrollArea::vertical().show(ui, |ui| {
            for line in &self.history {
                ui.label(line);
            }
        });

        // Command input
        ui.horizontal(|ui| {
            ui.label("> ");
            let response = ui.add(
                egui::TextEdit::singleline(&mut self.input)
                    .desired_width(f32::INFINITY)
                    .hint_text("Enter command..."),
            );

            if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                self.execute_command(command_handler);
            }

            if response.has_focus() && ui.input(|i| i.key_pressed(egui::Key::Tab)) {
                self.show_completions = true;
            }
        });

        // Show autocomplete suggestions
        if self.show_completions && !self.input.is_empty() {
            let suggestions = self.completer.complete_command(&self.input);
            if !suggestions.is_empty() {
                egui::Window::new("Suggestions")
                    .auto_sized()
                    .show(ui.ctx(), |ui| {
                        for suggestion in suggestions {
                            if ui.button(&suggestion).clicked() {
                                self.input = suggestion;
                                self.show_completions = false;
                            }
                        }
                    });
            }
        }
    }

    fn execute_command(&mut self, command_handler: &mut CommandHandler) {
        let command = self.input.trim().to_string();
        if !command.is_empty() {
            self.history.push(format!("> {}", command));
            
            let output = command_handler.execute(&command);
            if !output.is_empty() {
                self.history.push(output);
            }
            
            self.input.clear();
            self.show_completions = false;
        }
    }
}