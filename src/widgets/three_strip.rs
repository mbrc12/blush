use egui::{Ui, InnerResponse};

use crate::{util::color::{Color, hue_lerp, chroma_lerp, luminance_lerp}, state::Chan};

use super::{ShadeStrip, shade_strip::RoundingLegend};

pub struct ThreeStrip {
    axis: [ShadeStrip; 3],
}


impl ThreeStrip {
    const ROUNDING_LEGEND: [RoundingLegend; 3] = [RoundingLegend::new(1, 0, 1, 0),
                                                   RoundingLegend::new(0, 0, 0, 0),
                                                   RoundingLegend::new(0, 1, 0, 1)];

    const GAP_REMOVE: f32 = 3f32;

    pub fn new(color: &Color) -> Self {
        ThreeStrip{
            axis: [
                ShadeStrip::new(color, false, hue_lerp(0.0, 1.0), 
                                ThreeStrip::ROUNDING_LEGEND[0]),
                ShadeStrip::new(color, true, luminance_lerp(0.0, 1.0), 
                                ThreeStrip::ROUNDING_LEGEND[1]),
                ShadeStrip::new(color, false, chroma_lerp(0.0, 1.0), 
                                ThreeStrip::ROUNDING_LEGEND[2])
            ]
                   
        }
    }
    
    pub fn place(&mut self, ui: &mut Ui, color: Color, chan: &mut Chan, max_width: f32, max_height: f32) -> InnerResponse<()> {
        ui.vertical(|ui| {
            for i in 0..3 {
                ui.add(self.axis[i].widget(color, chan, max_width, max_height/3.0));
                ui.add_space(-Self::GAP_REMOVE)
            }
        })
    }
}   

