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
use crate::jade::dirbar::*;

pub struct App {
    running: bool,
    cwd: DirItem,
    dir_bar: DirBar,
    file_pane: FilePane,
    uname: String,
}
impl App {
    pub fn init() -> io::Result<Self>  {
        let uname = match std::env::var("USER") {
            Ok(uname) => uname,
            Err(_) => "notauserdon'tmatchthisthere'snouserhere".to_string(),
        };
        let cwd = DirItem::new(std::env::current_dir()?, &uname);

        Ok( Self {
            running: true,
            dir_bar: DirBar::init(&cwd),
            file_pane: FilePane::init(&cwd.path, &uname),
            uname,
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
            KeyCode::Char('q') | KeyCode::Esc => self.running = false,
            KeyCode::Char('j') | KeyCode::Down => self.file_pane.move_selector(SelMove::Down(1)),
            KeyCode::Char('k') | KeyCode::Up => self.file_pane.move_selector(SelMove::Up(1)),
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
            Constraint::Max(3),
            Constraint::Percentage(60),
            Constraint::Min(0),
            Constraint::Min(1),
        ]).areas(area);

        self.dir_bar.render(dir_bar, buf);
        self.file_pane.render(file_pane, buf);
        // self.draw_toolpane(tool_pane, buf);
        // self.draw_combar(com_bar, buf);
    }
}
