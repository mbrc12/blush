mod widgets;
pub mod util;
mod state;

use std::{path::Path, io, f32::INFINITY};

use state::{State, Chan};
use util::{color::{Color}};

use egui::{FontFamily, TextStyle, Ui, Layout, Align};

use widgets::{ShadeStrip, ThreeStrip, ColorMap};

pub struct Blush {
    // db: VPTree<f64, NamedColor>,
    font: egui::FontId,
    state: State,
    chan: Chan,
    three_strip: ThreeStrip,
    color_map: ColorMap,
}

impl Blush {
    pub fn new(_cc: &eframe::CreationContext) -> Result<Self, io::Error> {
        _cc.egui_ctx.set_pixels_per_point(1.0f32);
        // let db = util::color::load_db(Path::new("res/colors.json"))?;
        let color = Color::from_hex("#ff355e");
        let color_2 = Color::from_hex("#dd5ac1");
        Ok(Blush { 
            // db,
            font: egui::FontId::new(30.0, FontFamily::Monospace),
            state: State::default(),
            chan: Chan::default(),
            three_strip: ThreeStrip::new(&color),
            color_map: ColorMap::new(),
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

            // self.count += 1;

            // if ui.button("Hello").clicked() {
            //     self.shown = !self.shown;
            // }
            // if self.shown {
            //     ui.label(format!("I am visible!, {}", self.count));
            // }
    
            // let num = 5;
            // let max_width = ui.max_rect().width();
            // let each_width = max_width / (num as f32) - ui.spacing().item_spacing.x * (1.0 - 1.0/(num as f32));

            // // ui.add(self.ss.widget(&mut self.base_color, &[[1.0, 1.0], [1.0, 1.0]], &hue_lerp(0.0, 1.0)));
            // // egui::ScrollArea::horizontal().show(ui, |ui| {
            // //     // self.three_strip.place(ui, &mut self.base_color);
            // //     // self.three_strip_2.place(ui, &mut self.base_color_2);
            // //     log::info!("{:?}", ui.available_size());
            // ui.horizontal(|ui| {
            //     for _ in 1..=num {
            //         self.three_strip.place(ui, self.state.base_color(), &mut self.chan, each_width, INFINITY);
            //     }
            // });
            
            ui.add(self.color_map.construct(self.state.color_map(), &mut self.chan, 500.0, 500.0));

            self.state.process_chan(&mut self.chan)
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
