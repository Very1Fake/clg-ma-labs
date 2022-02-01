use eframe::{
    egui::CentralPanel,
    epi::{App as EApp, NativeOptions},
    run_native,
};

struct App;

impl EApp for App {
    fn update(&mut self, ctx: &eframe::egui::CtxRef, _frame: &eframe::epi::Frame) {
        CentralPanel::default().show(ctx, |ui| ui.label("Hello World"));
    }

    fn name(&self) -> &str {
        "MA: Lab 6"
    }
}

impl App {
    fn new() -> Self {
        App {}
    }
}

fn main() {
    run_native(Box::new(App::new()), NativeOptions::default());
}
