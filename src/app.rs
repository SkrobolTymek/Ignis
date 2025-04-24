use eframe::egui;
use crate::{terminal::Terminal, theme::ThemeManager, commands::CommandHandler};

pub struct IgnisApp {
    terminal: Terminal,
    theme: ThemeManager,
    command_handler: CommandHandler,
}

impl IgnisApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let theme = ThemeManager::load_default();
        theme.apply(&cc.egui_ctx);
        
        Self {
            terminal: Terminal::new().expect("Failed to initialize terminal"),
            theme,
            command_handler: CommandHandler::new(),
        }
    }
}

impl eframe::App for IgnisApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.theme.apply(ctx);
        
        egui::CentralPanel::default().show(ctx, |ui| {
            self.terminal.show(ui, &mut self.command_handler);
        });
    }
}