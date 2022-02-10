// Turn off terminal on Windows OS
#![windows_subsystem = "windows"]
#[cfg(not(target_arch = "wasm32"))]
fn main() {
    eframe::run_native(
        Box::new(ma_labs::LabsApp::default()),
        eframe::NativeOptions::default(),
    );
}
