use ratatui::{
    prelude::*,
    buffer::Buffer,
    crossterm::{
        event, event::{
            Event, KeyCode, KeyEvent, KeyEventKind}
        ,
    },
    layout::{Constraint, Layout, Rect},
    style::Color,
    text::{Line, Span},
    widgets::{Block, Tabs, Widget},
    Frame,
};
use std::{
    io,
    path::PathBuf,
};

use crate::jade::jfs::*;

pub struct App {
    running: bool,
    cwd: Listing,
    list: Vec<Listing>,
}
impl App {
    pub fn init() -> io::Result<Self>  {
        let cwd = Listing::new(std::env::current_dir()?);

        Ok(Self {
            running: true,
            list: list_dir(&cwd.path),
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
            Constraint::Min(1),
            Constraint::Percentage(60),
            Constraint::Min(0),
            Constraint::Min(1),
        ]).areas(area);

        self.draw_dirbar(dir_bar, buf);
        self.draw_filepane(file_pane, buf);
        self.draw_toolpane(tool_pane, buf);
        self.draw_combar(com_bar, buf);
    }
}

impl App {
    fn draw_dirbar(&self, area: Rect, buf: &mut Buffer) {
        Span::from(&self.cwd.name).render(area, buf);
    }
    fn draw_filepane(&self, area: Rect, buf: &mut Buffer) {
        for entry in &self.list {
            Line::from(Span::from(&entry.name)).render(area, buf);
        }
    }
    fn draw_toolpane(&self, area: Rect, buf: &mut Buffer) {

    }
    fn draw_combar(&self, area: Rect, buf: &mut Buffer) {

    }
}
