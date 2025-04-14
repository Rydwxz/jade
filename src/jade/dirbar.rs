use ratatui::{
    prelude::*,
    widgets::{Block, Paragraph, Widget},
};

// use crate::jade::jfs::*;

pub struct Statusline {}

impl Widget for &Statusline {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Paragraph::new(Line::raw("Hello JADE")).render(area, buf);
    }
}

impl Statusline {
    pub fn init() -> Self {
        Self {}
    }
}
