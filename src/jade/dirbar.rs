use ratatui::{
    prelude::*,
    widgets::{
        Widget,
        Block,
    },
};

use crate::jade::jfs::*;

pub struct DirBar {
    cwd: DirItem,
}

impl Widget for &DirBar {
    fn render(self, area: Rect, buf: &mut Buffer) {
        Block::bordered().render(area, buf);
        Line::from(vec![
            Span::raw(&self.cwd.prefix),
            Span::raw(&self.cwd.sub_path),
            Span::raw(&self.cwd.name),
        ]).render(area, buf);
    }
}

impl DirBar {
    pub fn init(cwd: &DirItem) -> Self {
        Self {
            cwd: cwd.clone(),
        }
    }
}
