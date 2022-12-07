use egui::{Ui, InnerResponse};

use crate::util::color::{Color, hue_lerp, chroma_lerp, luminance_lerp};

use super::{ShadeStrip, shade_strip::RoundingLegend};

pub struct ThreeStrip {
    axis: [ShadeStrip; 3],
}


impl ThreeStrip {
    const ROUNDING_LEGEND: [RoundingLegend; 3] = [RoundingLegend::new(1, 0, 1, 0),
                                                   RoundingLegend::new(0, 0, 0, 0),
                                                   RoundingLegend::new(0, 1, 0, 1)];

    const GAP_REMOVE: f32 = 3f32;

    pub fn new(color: &Color, max_width: f32, max_height: f32) -> Self {
        ThreeStrip{
            axis: [
                ShadeStrip::new(color, false, max_width, max_height/3.0, hue_lerp(0.0, 1.0), 
                                ThreeStrip::ROUNDING_LEGEND[0]),
                ShadeStrip::new(color, true, max_width, max_height/3.0, luminance_lerp(0.0, 1.0), 
                                ThreeStrip::ROUNDING_LEGEND[1]),
                ShadeStrip::new(color, false, max_width, max_height/3.0, chroma_lerp(0.0, 1.0), 
                                ThreeStrip::ROUNDING_LEGEND[2])
            ]
                   
        }
    }
    
    pub fn place(&mut self, ui: &mut Ui, color: &mut Color) -> InnerResponse<()> {
        let lerps = [hue_lerp(0.0, 1.0), luminance_lerp(0.0, 1.0), chroma_lerp(0.0, 1.0)];
        ui.vertical(|ui| {
            for i in 0..3 {
                ui.add(self.axis[i].widget(color));
                ui.add_space(-Self::GAP_REMOVE)
            }
        })
    }
}   

