mod widgets;
pub mod util;
use std::{path::Path, io};

use util::{color::{Color, NamedColor, hue_lerp, luminance_lerp, chroma_lerp}, vptree::VPTree};

use egui::{FontFamily, TextStyle, Ui, Layout, Align};

use widgets::{ShadeStrip, ThreeStrip};

pub struct Blush {
    // db: VPTree<f64, NamedColor>,
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
        _cc.egui_ctx.set_pixels_per_point(1.0f32);
        // let db = util::color::load_db(Path::new("res/colors.json"))?;
        let color = Color::from_hex("#ff355e");
        let color_2 = Color::from_hex("#dd5ac1");
        Ok(Blush { 
            // db,
            shown: false,  
            count: 0,
            font: egui::FontId::new(30.0, FontFamily::Monospace),
            base_color: color,
            base_color_2: color_2,
            three_strip: ThreeStrip::new(&color, 400.0, 200.0),
            three_strip_2: ThreeStrip::new(&color_2, 400.0, 200.0),
        })
    }

    fn apply_styles(&self, ui: &mut Ui) {
        ui.style_mut().text_styles.insert(TextStyle::Button, self.font.clone());
        ui.style_mut().text_styles.insert(TextStyle::Body, self.font.clone());
    }
}

impl eframe::App for Blush {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let _ = egui::TopBottomPanel::top("colors").show(ctx, |ui| {
            self.apply_styles(ui);

            self.count += 1;

            if ui.button("Hello").clicked() {
                self.shown = !self.shown;
            }
            if self.shown {
                ui.label(format!("I am visible!, {}", self.count));
            }

            // ui.add(self.ss.widget(&mut self.base_color, &[[1.0, 1.0], [1.0, 1.0]], &hue_lerp(0.0, 1.0)));
            // egui::ScrollArea::horizontal().show(ui, |ui| {
            //     // self.three_strip.place(ui, &mut self.base_color);
            //     // self.three_strip_2.place(ui, &mut self.base_color_2);
            //     log::info!("{:?}", ui.available_size());
            ui.horizontal(|ui| {
                for _ in 1..10 {
                    self.three_strip.place(ui, &mut self.base_color);
                }
            });
            // });
            
            // egui::Grid::new("colors").show(ui, |ui| {
            //     self.three_strip.place(ui, &mut self.base_color);
            //     self.three_strip.place(ui, &mut self.base_color);
            //     self.three_strip.place(ui, &mut self.base_color);
            //     self.three_strip.place(ui, &mut self.base_color);
            //     ui.end_row();

            //     self.three_strip.place(ui, &mut self.base_color);
            //     self.three_strip.place(ui, &mut self.base_color);

            //     ui.end_row();
            // });
        });

        // resp.response.on_hover_text("Hovered");
    }
}
