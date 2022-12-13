use egui::{Rect, Rounding, vec2, pos2, FontId};

use super::color::Color;

#[derive(Clone, Copy)]
pub struct RoundedRect {
    pub rect: Rect, 
    pub rounding: Rounding
}

#[derive(Copy, Clone, Debug, Default)]
pub struct RoundingLegend {
    nw: f32,
    sw: f32,
    ne: f32, 
    se: f32
}

impl RoundedRect {

    const DEFAULT_INSET_FRAC: f32 = 0.8;

    // Split rounded rectangle, output in row-major order.
    pub fn split(&self, rows: usize, cols: usize) -> Vec<RoundedRect> {
        let RoundedRect { rect, rounding } = self;
        let each_size = vec2(rect.width() / cols as f32, rect.height() / rows as f32);
    
        let mut rrs = vec![];

        for r in 0..rows {
            for c in 0..cols {
                let left_top_rc = pos2(rect.left() + each_size.x * c as f32, rect.top() + each_size.y * r as f32);
                let mut rnd_rc = Rounding::none();
                if r == 0 {
                    if c == 0 {
                        rnd_rc.nw = rounding.nw;
                    } 
                    if c + 1 == cols {
                        rnd_rc.ne = rounding.ne;
                    }
                } 
                if r + 1 == rows {
                    if c == 0 {
                        rnd_rc.sw = rounding.sw;
                    } 
                    if c + 1 == cols {
                        rnd_rc.se = rounding.se;
                    }
                }

                rrs.push((Rect::from_min_size(left_top_rc, each_size), rnd_rc).into());
            }
        }

        rrs
    }


    pub fn split_ver_flat(&self) -> (RoundedRect, RoundedRect) {
        let rrs = self.split(2, 1);
        (rrs[0], rrs[1])
    }

    pub fn label_inset(&self, painter: &egui::Painter, text: String, color: Color, size_ratio: Option<f32>) {
        let font = FontId::proportional(self.rect.height() * size_ratio.unwrap_or(RoundedRect::DEFAULT_INSET_FRAC));
        let galley = painter.layout(text, font, color.to_color32(), self.rect.width());
        let top_pos = self.rect.center() - galley.rect.size()/2.0;
        painter.galley(top_pos, galley);
    }
}

impl From<(Rect, Rounding)> for RoundedRect {
    fn from(rr: (Rect, Rounding)) -> Self {
        RoundedRect { rect: rr.0, rounding: rr.1 }
    }
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

