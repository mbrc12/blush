use egui::{Color32, Rounding, Ui, vec2, Sense, Widget, pos2, Stroke, Rect, FontId, Layout};
use crate::{util::color::{Color, ColorDB, shades, Lerp}, state::{Chan, Message}};

#[derive(Clone, Copy)]
pub struct RoundingLegend {
    nw: f32,
    sw: f32,
    ne: f32, 
    se: f32
}

impl RoundingLegend {
    pub const fn new(nw: i32, sw: i32, ne: i32, se: i32) -> RoundingLegend {
        RoundingLegend{
            nw: nw as f32,
            sw: sw as f32,
            ne: ne as f32,
            se: se as f32
        }
    }

    pub fn to_rounding(self, radius: f32) -> Rounding {
        Rounding {
            nw: self.nw * radius,
            sw: self.sw * radius,
            ne: self.ne * radius,
            se: self.se * radius
        }
    }
}

pub struct ShadeStrip {
    last_color: Color,
    last_index: usize,
    shades: Vec<(Color, Color32)>,

    new_color: Color,
    max_shade_count: usize,
    lerp: Lerp,
    rounding_legend: RoundingLegend,
    show_hex: bool,
}

impl ShadeStrip {
    const ASPECT_RATIO: f32 = 6.0;
    const ROUND_RADIUS_FRAC: f32 = 0.02;
    const DOT_RADIUS_FRAC: f32 = 0.01;
    const SELECT_WIDTH: f32 = 2.0;
    const OUTLINE_WIDTH: f32 = 2.0;

    pub fn new(color_ref: &Color, show_hex: bool, 
               lerp: Lerp, rounding_legend: RoundingLegend) -> Self {
        let color = *color_ref;
        ShadeStrip { 
            last_color: color,
            last_index: usize::MAX,
            shades: vec![],

            new_color: color,
            max_shade_count: 10,
            lerp,
            rounding_legend,
            show_hex
        }
    }

    pub fn construct<'a>(&'a mut self, base_color: Color, chan: &'a mut Chan, 
                         max_width: f32, max_height: f32, disabled: bool) -> impl Widget + 'a {
        move |ui: &mut Ui| -> egui::Response {

            let width = max_width.min(max_height * ShadeStrip::ASPECT_RATIO);
            let height = max_width / ShadeStrip::ASPECT_RATIO;

            let radius = width * ShadeStrip::ROUND_RADIUS_FRAC;
            let rounding = self.rounding_legend.to_rounding(radius);

            let (id, rect) = ui.allocate_space(vec2(width, height));
            let response = ui.interact(rect, id, Sense::union(Sense::hover(), Sense::click()));
            let painter = ui.painter();

            self.new_color = base_color;
            let mut normal_draw = false;

            if !disabled && response.hovered() {

                let (shades, index) = if self.last_color == base_color && self.last_index != usize::MAX {
                    (&self.shades, self.last_index)
                } else {
                    let (shades, last_index) = shades(base_color, self.max_shade_count, &self.lerp);
                    self.last_index = last_index;
                    self.shades = shades.iter().map(|s| (*s, s.to_color32())).collect();
                    (&self.shades, self.last_index)
                };

                let each_width = rect.width()/(shades.len() as f32);

                for (i, (shade, shade_color32)) in shades.iter().enumerate() {
                    let shade_rect_x = rect.left() + each_width * (i as f32);
                    let shade_rect = Rect::from_min_size(pos2(shade_rect_x, rect.top()), vec2(each_width, rect.height()));

                    let rounding = if i == 0 {
                        Rounding{ne: 0f32, se: 0f32, ..rounding}
                    } else if i + 1 == shades.len() {
                        Rounding{nw: 0f32, sw: 0f32, ..rounding}
                    } else {
                        Rounding::default()
                    };

                    painter.rect_filled(shade_rect, rounding, *shade_color32);

                    let dot_radius = width * ShadeStrip::DOT_RADIUS_FRAC;

                    let is_inside = response.ctx.pointer_interact_pos()
                        .map_or(false, |pos| shade_rect.contains(pos));
                    if is_inside {
                        self.new_color = *shade; 
                        painter.circle_stroke(shade_rect.center(), dot_radius, 
                                              Stroke{width: Self::SELECT_WIDTH,
                                                     color: shade.borw()
                                              });
                    }

                    if i == index {
                        painter.circle_filled(shade_rect.center(), dot_radius , shade.borw());
                    }
                }
            } 

            if !disabled && response.clicked() { // Color got selected
                chan.push(Message::ChangeColor {to: self.new_color});
            }

            if !response.hovered() || disabled {
                normal_draw = true;
            }

            if normal_draw {
                let draw_color = if disabled {
                    Color {chroma: 0.05, ..base_color}
                } else {
                    base_color
                };

                painter.rect_filled(rect, rounding, draw_color.to_color32());

                if self.show_hex {
                    let font = FontId::monospace(rect.height() * 0.75f32);
                    let galley = 
                        painter.layout(base_color.to_hex(), font, base_color.borw(), rect.width());
                    let position = rect.center() - galley.rect.size()/2f32;
                    painter.galley(position, galley);
                }
            }
            
            painter.rect_stroke(rect, rounding, Stroke{width: ShadeStrip::OUTLINE_WIDTH, color: Color32::BLACK});

            response
        }
    }
}
