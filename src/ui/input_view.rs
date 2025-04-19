use ratatui::{
    prelude::*,
    widgets::{Block, Paragraph, Widget},
};

// use crate::jade::fs::*;

pub struct InputView {
    buffer: Vec<char>,
}

impl Widget for &InputView {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Paragraph::new(Line::raw("Hello JADE")).render(area, buf);
    }
}

impl InputView {
    pub fn init() -> Self {
        Self {
            buffer: Vec::new(),
        }
    }

    pub fn new_input(&mut self, input: char) {
        self.buffer.push(input);
    }
}
