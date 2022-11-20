mod widgets;
pub mod util;
use std::{path::Path, io};

use util::{color::{Color, NamedColor, hue_lerp, luminance_lerp, chroma_lerp}, vptree::VPTree};

use egui::{FontFamily, TextStyle, Ui, Layout, Align};

use widgets::{ShadeStrip, ThreeStrip};

pub struct Blush {
    db: VPTree<f64, NamedColor>,
    shown: bool,
    count: u32,
    font: egui::FontId,

    base_color: Color,
    base_color_2: Color,
    three_strip: ThreeStrip,
    three_strip_2: ThreeStrip,
}

impl Blush {
    pub fn new(_cc: &eframe::CreationContext) -> Result<Self, io::Error> {
        let db = util::color::load_db(Path::new("res/colors.json"))?;
        let color = Color::from_hex("#ff355e");
        let color_2 = Color::from_hex("#dd5ac1");
        Ok(Blush { 
            db,
            shown: false,  
            count: 0,
            font: egui::FontId::new(30.0, FontFamily::Monospace),
            base_color: color,
            base_color_2: color_2,
            three_strip: ThreeStrip::new(&color),
            three_strip_2: ThreeStrip::new(&color_2),
        })
    }

    fn apply_styles(&self, ui: &mut Ui) {
        ui.style_mut().text_styles.insert(TextStyle::Button, self.font.clone());
        ui.style_mut().text_styles.insert(TextStyle::Body, self.font.clone());
    }
}

impl eframe::App for Blush {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let resp = egui::TopBottomPanel::top("colors").show(ctx, |ui| {
            self.apply_styles(ui);

            self.count += 1;

            if ui.button("Hello").clicked() {
                self.shown = !self.shown;
            }
            if self.shown {
                ui.label(format!("I am visible!, {}", self.count));
            }
            
            ui.horizontal(|ui| {
                self.three_strip.place(ui, &mut self.base_color);
                self.three_strip_2.place(ui, &mut self.base_color_2);
            });
        });

        // resp.response.on_hover_text("Hovered");
    }
}
