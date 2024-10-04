use ratatui::{
    prelude::*,
    buffer::Buffer,
    crossterm::{
        event, event::{
            Event, KeyCode, KeyEvent, KeyEventKind}
        ,
    },
    layout::{Constraint, Layout, Rect},
    text::{Line, Span},
    widgets::{Block, Tabs, Widget},
    Frame,
};
use std::{
    io,
};

use crate::jade::jfs::*;
use crate::jade::filepane::*;

pub struct App {
    running: bool,
    cwd: Listing,
    file_pane: FilePane,
}
impl App {
    pub fn init() -> io::Result<Self>  {
        let cwd = Listing::new(std::env::current_dir()?);

        Ok(Self {
            running: true,
            file_pane: FilePane::init(&cwd),
            cwd,
        })
    }

    pub fn run_app<B: Backend>(&mut self, mut term: Terminal<B>) -> io::Result<bool> {
        while self.running {
            term.draw(|frame| self.draw_ui(frame))?;
            self.handle_events()?;
        }
        Ok(true)
    }

    pub fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key) if key.kind == KeyEventKind::Press => self.handle_key_press(key),
            _ => {}
        }
        Ok(())
    }

    pub fn handle_key_press(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Esc | KeyCode::Char('q') => self.running = false,
            KeyCode::Char('j') => self.file_pane.move_cursor(SelMove::Down(1)),
            KeyCode::Char('k') => self.file_pane.move_cursor(SelMove::Up(1)),
            _ => {},
        }
    }

    pub fn draw_ui(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [dir_bar,
        file_pane,
        tool_pane,
        com_bar] = Layout::vertical([
            Constraint::Max(1),
            Constraint::Percentage(60),
            Constraint::Min(0),
            Constraint::Min(1),
        ]).areas(area);

        self.draw_dirbar(dir_bar, buf);
        self.file_pane.render(file_pane, buf);
        // self.draw_toolpane(tool_pane, buf);
        // self.draw_combar(com_bar, buf);
    }
}

impl App {
    fn draw_dirbar(&self, area: Rect, buf: &mut Buffer) {
        Span::from(&self.cwd.name).render(area, buf);
    }
}
