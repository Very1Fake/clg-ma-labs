#[cfg(not(target_arch = "wasm32"))]
fn main() {
    eframe::run_native(
        Box::new(lab_6::App::default()),
        eframe::NativeOptions::default(),
    );
}
