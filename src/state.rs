use crate::util::{color::Color, buffer::Buffer};

pub type Chan = Buffer<Message>;

#[derive(Default)]
pub struct State {
    base_color: Color
}

#[derive(Default, Clone, Copy)]
pub enum Message {
    #[default] NoOp,
    ChangeColor { to: Color },
}

impl State {
    pub fn base_color(&self) -> Color { self.base_color }

    pub(super) fn process_chan(&mut self, chan: &mut Chan) {
        for msg in chan.items() {
            self.process(msg);
        }
    }

    fn process(&mut self, msg: Message) {
        use Message::*;

        match msg {
            NoOp => (),
            ChangeColor { to } => self.base_color = to,
        }
    }
}
