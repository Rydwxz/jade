use crate::jade::fs::*;
use ratatui::{
    style::{Color, Style},
    text::Span,
    widgets::{List, ListItem, ListState, StatefulWidget, Widget},
};
use std::path::PathBuf;

pub struct ListView {
    sel: usize,
    list: DirList,
}

impl Widget for &ListView {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        StatefulWidget::render(
            List::new(
                self.list
                    .items
                    .iter()
                    .map(|e| ListItem::new(Span::from(&e.name))),
            )
            .highlight_style(Style::new().bg(Color::Rgb(255, 0, 0))),
            area,
            buf,
            &mut ListState::default().with_selected(Some(self.sel)),
        );
    }
}

impl ListView {
    pub fn init(cwd: &PathBuf, uname: &str) -> Self {
        Self {
            sel: 0,
            list: DirList::new(cwd.to_path_buf(), uname),
        }
    }

    pub fn jog_selector(&mut self, n: i16) {
        match n < 0 {
            false => self.sel = self.sel.saturating_add(n as usize),
            true => self.sel = self.sel.saturating_sub((n * -1) as usize),
        }
    }
}

pub enum SelMove {
    Up(usize),
    Down(usize),
}
