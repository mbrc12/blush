use egui::{Widget, Sense, vec2, Rounding, Stroke};

use crate::{state::IndexRepr, util::{color::Color, RoundedRect}};

pub struct ColorLabel {}

impl ColorLabel {
    
    const ASPECT_RATIO: f32 = 2.0;
    const ROUND_RADIUS_FRAC: f32 = 0.5;
    const STROKE_WIDTH: f32 = 4.0;

    pub fn new() -> Self {
        ColorLabel {}
    }

    pub fn construct(&'_ mut self, color: Color, label: IndexRepr,
                         max_width: f32, max_height: f32) -> impl Widget + '_ {
        move |ui: &mut egui::Ui| -> egui::Response {
            let width = max_width.min(max_height * ColorLabel::ASPECT_RATIO);
            let height = width / ColorLabel::ASPECT_RATIO;

            let (id, rect) = ui.allocate_space(vec2(width, height));
            let response = ui.interact(rect, id, Sense::union(Sense::hover(), Sense::click()));
            let painter = ui.painter();
    
            let radius = width * ColorLabel::ROUND_RADIUS_FRAC;
            let rounding = Rounding::same(radius);

            painter.rect_filled(rect, rounding, color.to_color32());
            painter.rect_stroke(rect, rounding, Stroke{color: color.accent(), width: ColorLabel::STROKE_WIDTH});

            let rr: RoundedRect = (rect, rounding).into();
            rr.label_inset(painter, label, color.accent_color(), None);

            response
        }
    }
}
