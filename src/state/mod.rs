use std::{collections::HashMap};

use crate::util::{color::Color, buffer::Buffer};

mod map_data;
pub use map_data::MapData;
pub use map_data::Location;

pub type Chan = Buffer<Message>;

#[derive(Default)]
pub struct State {
    base_color: Color,
    color_map: map_data::MapData,
    color_choose_state: bool,
    choose_loc: Location,
}

#[derive(Default, Clone, Copy, Debug)]
pub enum Message {
    #[default] NoOp,
    ChangeColor { to: Color },
    AddColor { loc: (usize, usize) },
    UpdateColor { loc: (usize, usize) },
    DeleteColor { loc: (usize, usize) }
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
            
            // From three strip ////////// 

            ChangeColor { to } => {
                if self.color_choose_state {
                    self.color_map.update_color(self.choose_loc, to);
                }
            }
        
            // From color map ///////////
            
            AddColor { loc } => self.color_map.add_color(loc),
            
            UpdateColor { loc } => {
                self.color_choose_state = true;
                self.choose_loc = loc;
            }

            DeleteColor { loc } => self.color_map.delete_color(loc)
        }
    }

    pub fn base_color(&self) -> Color {
        self.base_color
    }

    pub fn color_map(&self) -> &map_data::MapData {
        &self.color_map
    }
}


