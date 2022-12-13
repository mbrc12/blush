use std::collections::HashMap;

use egui::{Pos2, pos2};

use crate::util::color::Color;

pub type Index = usize;
pub type IndexRepr = String;

pub fn to_repr(idx: Index) -> IndexRepr {
    format!("{:x}", idx)
}

pub type Location = (usize, usize);

pub struct MapData {
    rows: usize,
    cols: usize,
    map: HashMap<Location, Index>,
    colors: HashMap<Index, Color>,
    next_color: Index,
    
    choose_color_mode: bool,
    choose_color_pos: Pos2,
    choose_color_idx: Index,
}

impl Default for MapData {
    fn default() -> Self {
        Self {
            rows: 2,
            cols: 3,
            map: HashMap::from([
                               ((0, 0), 0), 
                               ((1, 0), 1),
                               ((1, 1), 2)
            ]),
            colors: HashMap::from([
                                  (0, Color::from_hex("#fcfafa")),
                                  (1, Color::from_hex("#c8d3d5")),
                                  (2, Color::from_hex("#a4b8c4"))
            ]),
            next_color: 3,

            choose_color_mode: false,
            choose_color_pos: pos2(0.0, 0.0),
            choose_color_idx: 0
        }
    }
}

impl MapData {
    pub fn size(&self) -> (usize, usize) {
        (self.rows, self.cols)
    }

    pub fn index_at(&self, loc: Location) -> Option<IndexRepr> {
        self.map.get(&loc).map(|x| to_repr(*x))
    }

    pub fn color_at(&self, loc: Location) -> Option<Color> {
        let index = self.map.get(&loc)?;
        self.colors.get(index).copied()
    }

    pub(super) fn add_color(&mut self, loc: Location, pos: Pos2) {
        self.colors.insert(self.next_color, Color::default());
        self.map.insert(loc, self.next_color);
        self.next_color += 1;
        self.start_change_color(loc, pos);
    }

    pub(super) fn start_change_color(&mut self, loc: Location, pos: Pos2) {
        self.choose_color_idx = *self.map.get(&loc).unwrap();
        self.choose_color_pos = pos;
        self.choose_color_mode = true;
    }

    pub(super) fn update_color(&mut self, to: Color) {
        self.colors.insert(self.choose_color_idx, to);
    }

    pub(super) fn delete_color(&mut self, loc: Location) {
        let idx = self.map.get(&loc).unwrap();
        self.colors.remove(idx);
    }

    pub(super) fn distracted(&mut self) {
        self.choose_color_mode = false;
    }

    pub fn choose_color_mode(&self) -> bool {
        self.choose_color_mode
    }

    pub fn choose_color_pos(&self) -> Pos2 {
        self.choose_color_pos
    }

    pub fn active_color(&self) -> Color {
        *self.colors.get(&self.choose_color_idx).unwrap_or(&Color::default())
    }
}

