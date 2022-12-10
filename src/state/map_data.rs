use std::collections::HashMap;

use crate::util::color::Color;

pub type Index = usize;
pub type IndexRepr = String;

fn to_repr(idx: Index) -> String {
    format!("{:x}", idx)
}

pub type Location = (usize, usize);

pub struct MapData {
    rows: usize,
    cols: usize,
    map: HashMap<Location, Index>,
    colors: HashMap<Index, Color>,
    next_color: Index,
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
                                  (0, Color::from_hex("#ddff00")),
                                  (1, Color::from_hex("#fefefe")),
                                  (2, Color::from_hex("#0a2a1f"))
            ]),
            next_color: 3,
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

    pub(super) fn add_color(&mut self, loc: Location) {
        self.colors.insert(self.next_color, Color::default());
        self.map.insert(loc, self.next_color);
        self.next_color += 1;
    }

    pub(super) fn update_color(&mut self, loc: Location, to: Color) {
        let idx = self.map.get(&loc).unwrap();
        self.colors.insert(*idx, to);
    }

    pub(super) fn delete_color(&mut self, loc: Location) {
        let idx = self.map.get(&loc).unwrap();
        self.colors.remove(idx);
    }
}

