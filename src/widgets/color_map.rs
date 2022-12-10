use std::f32::INFINITY;

use eframe::glow::LUMINANCE;
use egui::{Widget, Ui, Rounding, Sense, vec2, Rect, pos2, Color32, Pos2};

use crate::{state::{Chan, Message, MapData, Location}, util::color::Color};

struct RoundedRect {
    rect: Rect, 
    rounding: Rounding
}

impl RoundedRect {
    fn split_ver_flat(&self) -> (RoundedRect, RoundedRect) {
        let RoundedRect { rect, rounding } = self;
        let each_size = vec2(rect.width(), rect.height()/2.0);
        let rect_1 = Rect::from_min_size(rect.left_top(), each_size);
        let top_2 = rect.top() + rect.height()/2.0;
        let rect_2 = Rect::from_min_size(pos2(rect.left(), top_2), each_size);

        let rnd_1 = Rounding{sw: 0.0, se: 0.0, ..*rounding};
        let rnd_2 = Rounding{nw: 0.0, ne: 0.0, ..*rounding};

        ((rect_1, rnd_1).into(), (rect_2, rnd_2).into())
    }
}

impl From<(Rect, Rounding)> for RoundedRect {
    fn from(rr: (Rect, Rounding)) -> Self {
        RoundedRect { rect: rr.0, rounding: rr.1 }
    }
}

pub struct ColorMap {}

fn make_rounding(radius: f32) -> Rounding {
    Rounding::same(radius)
}

const MOUSE_FAR_AWAY: Pos2 = pos2(INFINITY, INFINITY);

impl ColorMap {

    const ROUND_RADIUS_FRAC: f32 = 0.05;

    pub fn new() -> Self {
        ColorMap {  }
    }

    pub fn construct<'a>(&'a mut self, map: &'a MapData, chan: &'a mut Chan, 
                         max_width: f32, max_height: f32) -> impl Widget + 'a {
        move |ui: &mut Ui| -> egui::Response {
            let (rows, cols) = map.size();
            let aspect_ratio = cols as f32 / rows as f32;
            let rect_width = max_width.min(max_height * aspect_ratio);
            let rect_height = max_width / aspect_ratio;

            let (id, rect) = ui.allocate_space(vec2(rect_width, rect_height));
            let response = ui.interact(rect, id, Sense::union(Sense::hover(), Sense::click()));
            let painter = ui.painter();

            let each_width = rect_width / cols as f32; 
            let each_height = rect_height / rows as f32;

            let radius = each_width * ColorMap::ROUND_RADIUS_FRAC;
            let rounding = make_rounding(radius);

            let mouse = response.ctx.pointer_interact_pos().unwrap_or(MOUSE_FAR_AWAY);
            let click = response.clicked();

            for r in 0..rows {
                for c in 0..cols {
                    let cell_color = map.color_at((r, c));
                    let cell_left = rect.left() + each_width * c as f32;
                    let cell_top = rect.top() + each_height * r as f32;
                    let cell_rect = Rect::from_min_size(pos2(cell_left, cell_top), vec2(each_width, each_height));

                    if let Some(cc) = cell_color {
                        valid_cell(painter, (cell_rect, rounding).into(), cc, (r, c), chan, mouse, click);
                    } else {
                        invalid_cell(painter, (cell_rect, rounding).into(), (r, c), chan, mouse, click); 
                    }
                }
            }

            response
        }
    }

}


fn button_color(color: Color) -> Color {
    let mut lum = color.luminance;
    if lum > 0.5 {
        lum -= 0.1;
    } else {
        lum += 0.1;
    }
    Color{luminance: lum, ..color}
}

fn valid_cell(painter: &egui::Painter, rr: RoundedRect, color: Color, 
              loc: Location, chan: &mut Chan, mouse: Pos2, click: bool) {
    let RoundedRect{ rect, rounding } = rr;
    painter.rect_filled(rect, rounding, color.to_color32());

    let button_color = button_color(color);

    let (top_rr, bot_rr) = rr.split_ver_flat();
    if top_rr.rect.contains(mouse) {
        painter.rect_filled(top_rr.rect, top_rr.rounding, button_color.to_color32());
        if click {
            chan.push(Message::UpdateColor { loc });
        }
    } else if bot_rr.rect.contains(mouse) {
        painter.rect_filled(bot_rr.rect, bot_rr.rounding, button_color.to_color32());
        if click {
            chan.push(Message::DeleteColor { loc });
        }
    }
}

const INVALID_COLOR_DARK: Color32 = Color32::from_rgb(55, 55, 55);
const INVALID_COLOR_LIGHT: Color32 = Color32::from_rgb(85, 85, 85);

fn invalid_cell(painter: &egui::Painter, rr: RoundedRect, 
                loc: Location, chan: &mut Chan, mouse: Pos2, click: bool) {
    let RoundedRect{ rect, rounding } = rr;

    if rect.contains(mouse) {
        painter.rect_filled(rect, rounding, INVALID_COLOR_LIGHT);
        if click {
            chan.push(Message::AddColor { loc });
        }
    } else {
        painter.rect_filled(rect, rounding, INVALID_COLOR_DARK);
    }
}
