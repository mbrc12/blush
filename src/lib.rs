mod widgets;
pub mod util;
pub mod gen;
mod state;

use std::{path::Path, io, time::Duration};

use state::{State, Chan};
use util::color::Color;

use egui::{FontFamily, TextStyle, Ui, Layout, Align, pos2, vec2};

use widgets::{ShadeStrip, ThreeStrip, ColorMap, ColorLabel};

pub struct Blush {
    // db: VPTree<f64, NamedColor>,
    font: egui::FontId,
    state: State,
    chan: Chan,
    color_picker: ThreeStrip,
    color_map: ColorMap,
}

#[derive(Debug, PartialEq)]
enum Enum {
    First,
    Second,
    Third,
}
impl Blush {

    const UPDATE_MAX_INTERVAL: Duration = Duration::from_millis(500); // atleast one refresh in this time

    pub fn new(cc: &eframe::CreationContext) -> Result<Self, io::Error> {
        cc.egui_ctx.set_pixels_per_point(1.0f32);
        // let db = util::color::load_db(Path::new("res/colors.json"))?;
        let mut fonts = egui::FontDefinitions::default();
        fonts.font_data.insert("uifont".into(), 
                               egui::FontData::from_static(include_bytes!("../res/Raleway-Regular.ttf")));
        fonts.families.get_mut(&FontFamily::Proportional).unwrap()
            .insert(0, "uifont".to_owned());

        cc.egui_ctx.set_fonts(fonts);

        Ok(Blush { 
            // db,
            font: egui::FontId::new(30.0, FontFamily::Name("uifont".into())),
            state: State::default(),
            chan: Chan::default(),
            color_picker: ThreeStrip::new(&Color::default()),
            color_map: ColorMap::new(),
        })
    }

    fn apply_styles(&self, ui: &mut Ui) {
        ui.style_mut().text_styles.insert(TextStyle::Button, self.font.clone());
        ui.style_mut().text_styles.insert(TextStyle::Body, self.font.clone());
    }
}

impl eframe::App for Blush {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        let _ = egui::TopBottomPanel::top("colors").show(ctx, |ui| {
            
            ctx.request_repaint_after(Blush::UPDATE_MAX_INTERVAL);
            self.apply_styles(ui);

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
           
            let max_size = frame.info().window_info.size;
    
            egui::TopBottomPanel::top("color_pickers").show(ctx, |ui| {
                ui.vertical_centered(|ui| {
                    let response = ui.add(self.color_map.construct(self.state.color_map(), &mut self.chan, 
                                                                   max_size.x,
                                                                   max_size.y/2.0));

                    if self.state.color_map().choose_color_mode() {
                        // println!("{:?}", self.state.color_choose_at());
                        egui::Window::new("color-picker")
                            .title_bar(false)
                            .auto_sized()
                            .open(&mut true)
                            .fixed_pos(self.state.color_map().choose_color_pos())
                            .show(ctx, |ui| {
                                ui.vertical(|ui| {
                                    ui.label("Choose color:");
                                    self.color_picker.place(ui, self.state.color_map().active_color(),
                                        &mut self.chan, 300.0, 300.0);
                                });
                            });

                    }
                    response
                })});

            egui::CentralPanel::default().show(ctx, |ui|{
                ui.vertical(|ui| {
                    // let colorpicker_id = ui.make_persistent_id("color-picker");
                    
                    // if self.state.color_choose_state {
                    //     ui.memory().open_popup(colorpicker_id);
                    // } else if ui.memory().is_popup_open(colorpicker_id){
                    //     ui.memory().close_popup();
                    // }


                    ui.add(ColorLabel::new().construct(Color::default(), "2".to_owned(), 100.0, 100.0));

                    // egui::popup::popup_below_widget(ui, colorpicker_id,
                    //                                 &response, |ui| {
                    //             ui.vertical(|ui| {
                    //                 ui.label("Choose color:");
                    //                 self.three_strip.place(ui, self.state.base_color(), 
                    //                                        &mut self.chan, 300.0, 300.0, !self.state.color_choose_state);
                    //             });
                    //     });
                    

                    let mut selected = Enum::Second;

                    egui::ComboBox::from_label("Select one!")
                        .selected_text(format!("{:?}", selected))
                        .show_ui(ui, |ui| {
                            ui.selectable_value(&mut selected, Enum::First, "First");
                            ui.selectable_value(&mut selected, Enum::Second, "Second");
                            ui.selectable_value(&mut selected, Enum::Third, "Third");
                        });
                })
            });

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
