use eframe::{NativeOptions, Theme};

fn main() {
    let native_opts = NativeOptions{default_theme: Theme::Light, ..eframe::NativeOptions::default()};
    eframe::run_native("blush", native_opts, Box::new(|cc| Box::new(blush::Blush::new(cc).unwrap())));
}
