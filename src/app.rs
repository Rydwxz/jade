use ratatui::{
    buffer::Buffer,
    crossterm::{
        event,
        event::{Event, KeyCode, KeyEvent, KeyEventKind},
    },
    layout::{Constraint, Layout, Rect},
    prelude::*,
    text::{Line, Span},
    widgets::{Block, Tabs, Widget},
    Frame,
};
use std::io;

use crate::jade::fs::*;
use crate::ui::{input_view::*, list_view::*};

pub struct App {
    continue_running: bool,
    cwd: DirItem,
    input_view: InputView,
    list_view: ListView,
    uname: String,
}
impl App {
    pub fn init() -> io::Result<Self> {
        let uname = match std::env::var("USER") {
            Ok(uname) => uname,
            Err(_) => "notauserdon'tmatchthisthere'snouserhere".to_string(),
        };
        let cwd = DirItem::new(std::env::current_dir()?, &uname);

        Ok(Self {
            continue_running: true,
            input_view: InputView::init(),
            list_view: ListView::init(&cwd.path, &uname),
            cwd,
            uname,
        })
    }

    pub fn run_app<B: Backend>(&mut self, mut term: Terminal<B>) -> io::Result<bool> {
        while self.continue_running {
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
            KeyCode::Esc => self.continue_running = false,
            KeyCode::Down => self.list_view.jog_selector(1),
            KeyCode::Up => self.list_view.jog_selector(-1),
            KeyCode::Char(c) => self.input_view.new_input(c),
            _ => {}
        }
    }

    pub fn draw_ui(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [list_view, input_view] =
            Layout::vertical([Constraint::Ratio(1, 1), Constraint::Min(2)]).areas(area);

        self.list_view.render(list_view, buf);
        self.input_view.render(input_view, buf);
    }
}

enum AppMode {
    Normal(Keymap),
    AlternateWindow,
    Custom(Keymap),
}

struct Keymap {}
