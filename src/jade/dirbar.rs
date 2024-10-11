use ratatui::{
    prelude::*,
    widgets::{
        Widget,
        Block,
        Paragraph,
    },
};

use crate::jade::jfs::*;

pub struct DirBar {
    cwd: DirItem,
}

impl Widget for &DirBar {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Paragraph::new(Line::from_iter(vec![
            Span::raw(&self.cwd.prefix),
            Span::raw(&self.cwd.sub_path),
            Span::raw(&self.cwd.name),
        ]))
        .block(Block::bordered())
        .render(area, buf);
    }
}

impl DirBar {
    pub fn init(cwd: &DirItem) -> Self {
        Self {
            cwd: cwd.clone(),
        }
    }
}
