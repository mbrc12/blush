use egui::{Widget, Sense, vec2, Rounding};

use crate::{state::IndexRepr, util::color::Color};

pub struct ColorLabel {}

impl ColorLabel {
    
    const ASPECT_RATIO: f32 = 1.0;
    const ROUND_RADIUS_FRAC: f32 = 0.5;

    pub fn new() -> Self {
        ColorLabel {}
    }

    pub fn construct<'a>(&'a mut self, color: Color, repr: IndexRepr,
                         max_width: f32, max_height: f32) -> impl Widget + 'a {
        move |ui: &mut egui::Ui| -> egui::Response {
            let width = max_width.min(max_height * ColorLabel::ASPECT_RATIO);
            let height = width / ColorLabel::ASPECT_RATIO;

            let (id, rect) = ui.allocate_space(vec2(width, height));
            let response = ui.interact(rect, id, Sense::union(Sense::hover(), Sense::click()));
            let painter = ui.painter();
    
            let radius = width * ColorLabel::ROUND_RADIUS_FRAC;
            let rounding = Rounding::same(radius);

            painter.rect_filled(rect, rounding, color.accent());

            response
        }
    }
}
