use std::{collections::HashMap};

use crate::util::{color::Color, buffer::Buffer};

mod map_data;
pub use map_data::IndexRepr;

use egui::Pos2;
pub use map_data::MapData;
pub use map_data::Location;

pub type Chan = Buffer<Message>;

#[derive(Default)]
pub struct State {
    color_map: map_data::MapData,
}

#[derive(Default, Clone, Copy, Debug)]
pub enum Message {
    #[default] NoOp,

    ChangeColor { to: Color },
    Distracted,

    AddColor { loc: (usize, usize), pos: Pos2 },
    UpdateColor { loc: (usize, usize), pos: Pos2 },
    DeleteColor { loc: (usize, usize) },
}

impl State {
    pub(super) fn process_chan(&mut self, chan: &mut Chan) {
        for msg in chan.items() {
            self.process(msg);
        }
    }

    fn process(&mut self, msg: Message) {
        use Message::*;

        println!("{:?}", msg);

        match msg {
            NoOp => (),
            
            // From three-strip ////////// 

            ChangeColor { to } => {
                self.color_map.update_color(to);
            }

            Distracted => {
                self.color_map.distracted();
            }
        
            // From color-map ///////////
            
            AddColor { loc, pos } => {
                self.color_map.add_color(loc, pos);
            }
            
            UpdateColor { loc, pos } => {
                self.color_map.start_change_color(loc, pos);
            }

            DeleteColor { loc } => {
                self.color_map.delete_color(loc);
            }
        }
    }

    pub fn color_map(&self) -> &map_data::MapData {
        &self.color_map
    }
}


