use egui::{Rect, Rounding, vec2, pos2};

#[derive(Clone, Copy)]
pub struct RoundedRect {
    pub rect: Rect, 
    pub rounding: Rounding
}

impl RoundedRect {

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
                    } else if c + 1 == cols {
                        rnd_rc.ne = rounding.ne;
                    }
                } else if r + 1 == rows {
                    if c == 0 {
                        rnd_rc.sw = rounding.sw;
                    } else if c + 1 == cols {
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
}

impl From<(Rect, Rounding)> for RoundedRect {
    fn from(rr: (Rect, Rounding)) -> Self {
        RoundedRect { rect: rr.0, rounding: rr.1 }
    }
}
