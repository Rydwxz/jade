use std::path::PathBuf;
use ratatui::{
    widgets::{Widget, StatefulWidget, List, ListItem, ListState},
    style::{Style, Color},
    text::{Span},
};
use crate::jade::jfs::*;

pub struct ListView {
    sel: usize,
    list: DirList,
}

impl Widget for &ListView {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        StatefulWidget::render(
            List::new(
                self.list.items.iter().map(
                    |e| { ListItem::new(Span::from(&e.name)) }
                )
            ).highlight_style(Style::new().bg(Color::Rgb(255, 0, 0))),
            area,
            buf,
            &mut ListState::default()
            .with_selected(Some(self.sel))
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

    pub fn move_selector(&mut self, n: SelMove) {
        match n {
            SelMove::Up(n) => match n < self.sel {
                true => self.sel -= n,
                false => self.sel = 0,
            },
            SelMove::Down(n) => match self.sel + n < self.list.len() {
                true => self.sel += n,
                false => self.sel = self.list.len() - 1,
            },
        }
    }
}

pub enum SelMove {
    Up(usize),
    Down(usize)
}
