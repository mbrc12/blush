mod widgets;
pub mod util;
use std::{path::Path, io};

use util::{color::{Color, NamedColor, hue_lerp, lightness_lerp}, vptree::VPTree};

use egui::{FontFamily, TextStyle, Ui};

use widgets::ShadeStrip;

pub struct Blush {
    db: VPTree<f64, NamedColor>,
    shown: bool,
    count: u32,
    font: egui::FontId,

    base_color: Color,
    shade_strip: ShadeStrip,
    shade_strip_2: ShadeStrip,
}

impl Blush {
    pub fn new(_cc: &eframe::CreationContext) -> Result<Self, io::Error> {
        let db = util::color::load_db(Path::new("res/colors.json"))?;
        let color = Color::from_hex("#ff355e");
        Ok(Blush { 
            db,
            shown: false,  
            count: 0,
            font: egui::FontId::new(30.0, FontFamily::Monospace),
            base_color: color,
            shade_strip: ShadeStrip::new(&color),
            shade_strip_2: ShadeStrip::new(&color),
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

            // ui.add(self.shade_strip.widget(&self.db, &mut self.base_color));
            ui.add(self.shade_strip.widget(&mut self.base_color, hue_lerp(0., 1.)));
            ui.add(self.shade_strip_2.widget(&mut self.base_color, lightness_lerp(0., 1.)));
        });

        resp.response.on_hover_text("Hovered");
    }
}
