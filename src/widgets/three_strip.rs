use egui::{Ui, InnerResponse};

use crate::util::color::{Color, hue_lerp, chroma_lerp, luminance_lerp};

use super::ShadeStrip;

pub struct ThreeStrip {
    axis: [ShadeStrip; 3],
}

impl ThreeStrip {

    const ROUNDING_LEGEND: [[[f32; 2]; 2]; 3] = [[[1f32, 0f32], [1f32, 0f32]],
                                                [[0f32, 0f32], [0f32, 0f32]],
                                                [[0f32, 1f32], [0f32, 1f32]]];   

    const GAP_REMOVE: f32 = 3f32;

    pub fn new(color: &Color) -> Self {
        ThreeStrip{
            axis: [ShadeStrip::new(color, false),
                    ShadeStrip::new(color, true),
                    ShadeStrip::new(color, false)]
        }
    }
    
    pub fn place(&mut self, ui: &mut Ui, color: &mut Color) -> InnerResponse<()> {
        let lerps = [hue_lerp(0.0, 1.0), luminance_lerp(0.0, 1.0), chroma_lerp(0.0, 1.0)];
        ui.vertical(|ui| {
            for i in 0..3 {
                ui.add(self.axis[i].widget(color, &Self::ROUNDING_LEGEND[i], &lerps[i]));
                ui.add_space(-Self::GAP_REMOVE)
            }
        })
    }
}   

