use egui::{Color32, Rounding, Ui, vec2, Sense, Widget, pos2, Stroke, Rect, FontId, Layout};
use crate::util::color::{Color, ColorDB, shades, Lerp};

pub struct ShadeStrip {
    new_color: Color,
    max_shade_count: usize,
    just_changed: bool,
    show_hex: bool,
}

impl ShadeStrip {
    const MIN_WIDTH: f32 = 400f32;
    const MIN_HEIGHT: f32 = 50f32;
    const ROUND_RADIUS: f32 = 5f32;
    const DOT_RADIUS: f32 = 5f32;
    const SELECT_WIDTH: f32 = 2f32;

    pub fn new(color_ref: &Color, show_hex: bool) -> Self {
        let color = *color_ref;
        ShadeStrip { 
            new_color: color,
            max_shade_count: 10,
            just_changed: false,
            show_hex
        }
    }

    pub fn widget<'a>(&'a mut self, base_color: &'a mut Color, legend: &'a [[f32; 2]; 2], lerp: &'a Lerp) -> impl Widget + 'a {
        move |ui: &mut Ui| -> egui::Response {
            let radius = ShadeStrip::ROUND_RADIUS;
            let rounding = Rounding{
                nw: legend[0][0] * radius,
                sw: legend[0][1] * radius,
                ne: legend[1][0] * radius,
                se: legend[1][1] * radius
            };

            let (id, rect) = ui.allocate_space(vec2(ShadeStrip::MIN_WIDTH, ShadeStrip::MIN_HEIGHT));
            let response = ui.interact(rect, id, Sense::union(Sense::hover(), Sense::click()));
            let painter = ui.painter();

            self.new_color = *base_color;
            let mut normal_draw = false;

            if response.hovered() && !self.just_changed {

                let (shades, index) = shades(*base_color, self.max_shade_count, lerp);
                let each_width = rect.width()/(shades.len() as f32);

                for (i, shade) in shades.iter().enumerate() {
                    let shade_color32 = shade.to_color32();
                    let shade_rect_x = rect.left() + each_width * (i as f32);
                    let shade_rect = Rect::from_min_size(pos2(shade_rect_x, rect.top()), vec2(each_width, rect.height()));

                    let rounding = if i == 0 {
                        Rounding{ne: 0f32, se: 0f32, ..rounding}
                    } else if i + 1 == shades.len() {
                        Rounding{nw: 0f32, sw: 0f32, ..rounding}
                    } else {
                        Rounding::default()
                    };

                    painter.rect_filled(shade_rect, rounding, shade_color32);

                    let is_inside = response.ctx.pointer_interact_pos()
                        .map_or(false, |pos| shade_rect.contains(pos));
                    if is_inside {
                        self.new_color = *shade; 
                        painter.circle_stroke(shade_rect.center(), ShadeStrip::DOT_RADIUS, 
                                              Stroke{width: Self::SELECT_WIDTH,
                                                     color: shade.borw()
                                              });
                    }

                    if i == index {
                        painter.circle_filled(shade_rect.center(), ShadeStrip::DOT_RADIUS, shade.borw());
                    }
                }
            } 

            if !response.hovered() {
                self.just_changed = false;
            }

            if response.is_pointer_button_down_on() {
                *base_color = self.new_color;
                self.just_changed = true;

                normal_draw = true;
            }


            if !response.hovered() || self.just_changed {
                normal_draw = true;
            }

            if normal_draw {
                painter.rect_filled(rect, rounding, base_color.to_color32());

                if self.show_hex {
                    let font = FontId::monospace(rect.height() * 0.75f32);
                    let galley = 
                        painter.layout(base_color.to_hex(), font, base_color.borw(), rect.width());
                    let position = rect.center() - galley.rect.size()/2f32;
                    painter.galley(position, galley);
                }
            }
            
            painter.rect_stroke(rect, rounding, Stroke{width: 2f32, color: Color32::BLACK});

            response
        }
    }
}
