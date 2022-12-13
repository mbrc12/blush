use std::f32::INFINITY;

use egui::{Widget, Ui, Rounding, Sense, vec2, Rect, pos2, Color32, Pos2};

use crate::{state::{Chan, Message, MapData, Location}, util::{color::Color, RoundedRect}};

pub struct ColorMap {}

fn make_rounding(radius: f32) -> Rounding {
    Rounding::same(radius)
}

const MOUSE_FAR_AWAY: Pos2 = pos2(INFINITY, INFINITY);

impl ColorMap {

    const ROUND_RADIUS_FRAC: f32 = 0.05;

    pub fn new() -> Self {
        ColorMap { }
    }

    pub fn construct<'a>(&'a mut self, map: &'a MapData, chan: &'a mut Chan, 
                         max_width: f32, max_height: f32) -> impl Widget + 'a {
        move |ui: &mut Ui| -> egui::Response {
            let (rows, cols) = map.size();
            let aspect_ratio = cols as f32 / rows as f32;
            let rect_width = max_width.min(max_height * aspect_ratio);
            let rect_height = rect_width / aspect_ratio;
            
            // dbg!(max_width, max_height, rect_width, rect_height, aspect_ratio, rows, cols);

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
                        valid_cell(painter, (cell_rect, rounding).into(), cc, (r, c), 
                                   map, chan, mouse, click);
                    } else {
                        invalid_cell(painter, (cell_rect, rounding).into(), (r, c), 
                                     map, chan, mouse, click); 
                    }
                }
            }

            response
        }
    }

}


fn select_color(color: Color) -> Color {
    let mut lum = color.luminance;
    if lum > 0.5 {
        lum -= 0.1;
    } else {
        lum += 0.1;
    }
    Color{luminance: lum, ..color}
}

fn valid_cell(painter: &egui::Painter, rr: RoundedRect, color: Color, loc: Location, 
              map: &MapData, chan: &mut Chan, mouse: Pos2, click: bool) {
    let RoundedRect{ rect, rounding } = rr;
    painter.rect_filled(rect, rounding, color.to_color32());

    let button_color = color.accent_color();

    let (top_rr, bot_rr) = rr.split_ver_flat();
    if top_rr.rect.contains(mouse) {
        painter.rect_filled(top_rr.rect, top_rr.rounding, button_color.to_color32());
        if click {
            chan.push(Message::UpdateColor { loc, pos: mouse });
        }
    } else if bot_rr.rect.contains(mouse) {
        painter.rect_filled(bot_rr.rect, bot_rr.rounding, button_color.to_color32());
        if click {
            chan.push(Message::DeleteColor { loc });
        }
    } else {
        rr.label_inset(painter, map.index_at(loc).unwrap(), button_color, None);
    }
}

const INVALID_COLOR_DARK: Color = Color{hue: 0.0, luminance: 0.3, chroma: 0.0};
const INVALID_COLOR_LIGHT: Color = Color{hue: 0.0, luminance: 0.4, chroma: 0.0};
const TESSELATE_LEVEL: usize = 4;

fn invalid_cell(painter: &egui::Painter, rr: RoundedRect, loc: Location, 
                map: &MapData, chan: &mut Chan, mouse: Pos2, click: bool) {
    let RoundedRect{ rect, rounding } = rr;
    if rect.contains(mouse) {
        tesselate(painter, rr, TESSELATE_LEVEL, 
                  [INVALID_COLOR_LIGHT, INVALID_COLOR_DARK]);
        // painter.rect_filled(rect, rounding, INVALID_COLOR_LIGHT);
        if click {
            chan.push(Message::AddColor { loc, pos: mouse });
        }
    } else {
        tesselate(painter, rr, TESSELATE_LEVEL * 2usize, 
                  [select_color(INVALID_COLOR_DARK), select_color(INVALID_COLOR_LIGHT)]);
    }
}

// tesselate a domain with C colors
fn tesselate<const C: usize>(painter: &egui::Painter, domain: RoundedRect, n: usize, cols: [Color; C]) {
    let rrs = domain.split(n, n);
    for (index, rr) in rrs.into_iter().enumerate() {
        painter.rect_filled(rr.rect, rr.rounding, cols[(index + index / n) % C].to_color32());
    }
}
