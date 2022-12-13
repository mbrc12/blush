use std::time::{SystemTime, Duration};

use egui::{Ui, InnerResponse};

use crate::{util::{color::{Color, hue_lerp, chroma_lerp, luminance_lerp}, RoundingLegend}, state::{Chan, Message}};

use super::ShadeStrip;

pub struct ThreeStrip {
    axis: [ShadeStrip; 3],
    entered: bool,
    first_shown: Option<SystemTime>
}

impl ThreeStrip {
    const ROUNDING_LEGEND: [RoundingLegend; 3] = [RoundingLegend::new(1, 0, 1, 0),
                                                   RoundingLegend::new(0, 0, 0, 0),
                                                   RoundingLegend::new(0, 1, 0, 1)];

    const GAP_REMOVE: f32 = 3f32;
    const IDLE_TIME: u64 = 1000;

    pub fn new(color: &Color) -> Self {
        ThreeStrip{
            axis: [
                ShadeStrip::new(color, false, hue_lerp(0.0, 1.0), 
                                ThreeStrip::ROUNDING_LEGEND[0]),
                ShadeStrip::new(color, true, luminance_lerp(0.0, 1.0), 
                                ThreeStrip::ROUNDING_LEGEND[1]),
                ShadeStrip::new(color, false, chroma_lerp(0.0, 1.0), 
                                ThreeStrip::ROUNDING_LEGEND[2])
            ],
            entered: false,
            first_shown: None
        }
    }

    // User didn't enter the popup even after some time of showing the popup.
    fn idle_elapsed(&self) -> bool {
        if let Some(first_shown) = self.first_shown {
            SystemTime::now().duration_since(first_shown).unwrap() >= Duration::from_millis(ThreeStrip::IDLE_TIME)
        } else { false }
    }
    
    pub fn place(&mut self, ui: &mut Ui, color: Color, chan: &mut Chan, 
                 max_width: f32, max_height: f32) -> InnerResponse<()> {
        let resp = ui.vertical(|ui| {
            for i in 0..3 {
                ui.add(self.axis[i].construct(color, chan, max_width, max_height/3.0, false));
                ui.add_space(-Self::GAP_REMOVE)
            }
        });

        if self.first_shown.is_none() {
            self.first_shown = Some(SystemTime::now());
        }

        if resp.response.hovered() {
            self.entered = true;
        }

        if !self.entered && self.idle_elapsed() {
            chan.push(Message::Distracted);
            self.entered = false;
            self.first_shown = None;
        }

        if !resp.response.hovered() && self.entered {
            chan.push(Message::Distracted);
            self.entered = false;
            self.first_shown = None;
        }
        
        resp
    }
}   

