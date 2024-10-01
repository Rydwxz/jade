use std::io;
use crossterm::event::{KeyCode, KeyEvent};
use crossterm::terminal;
use ratatui::{prelude::*, Terminal};
use ratatui::crossterm::{
    execute,
    terminal::{
        EnterAlternateScreen,
        LeaveAlternateScreen,
    },
    event, event::{
        Event,
        KeyEventKind,
    },
};

mod file; use file::*;

fn main() -> Result<(), io::Error> {

    let terminal = Terminal::new(CrosstermBackend::new(io::stderr())).expect("failed to connect to stderr");
    execute!(io::stderr(), EnterAlternateScreen).expect("failed to enter alternate screen");
    let mut jade = JadeApp::init()?;

    match jade.run_app(terminal) {
        Ok(_good_job) => (),
        Err(err) => println!("{err:}"),
    }

    execute!(io::stderr(), LeaveAlternateScreen).expect("failed to leave alternate screen");
    ratatui::restore();

    Ok(())
}


struct JadeApp {
    running: bool,
    files: Vec<File>,
    cwd: std::path::PathBuf,
}
impl JadeApp {
    fn init() -> io::Result<Self>  {
        let cwd = std::env::current_dir()?;

        Ok(Self {
            running: true,
            files: Vec::new(),
            cwd,
        })
    }

    fn run_app<B: Backend>(&mut self, mut term: Terminal<B>) -> io::Result<bool> {
        while self.running {
            term.draw(|frame| self.draw_ui(frame))?;
            self.handle_events()?;
        }
        Ok(true)
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key) if key.kind == KeyEventKind::Press => self.handle_key_press(key),
            _ => {}
        }
        Ok(())
    }

    fn handle_key_press(&mut self, key: KeyEvent) {
        match key.code {
            KeyCode::Esc => self.running = false,
            _ => {},
        }
    }

    fn draw_ui(&self, frame: &mut Frame) {
        frame.render_widget(self, frame.area());
    }
}

impl Widget for &JadeApp {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let [dir_bar,
        file_pane,
        tool_pane,
        com_bar] = Layout::horizontal([
            Constraint::Min(1),
            Constraint::Percentage(60),
            Constraint::Min(0),
            Constraint::Min(1),
        ]).areas(area);

        Block::new().style(THEME.root).render(area, buf);
        self.draw_dirbar(dir_bar, buf);
        self.draw_filepane(file_pane, buf);
        self.draw_toolpane(tool_pane, buf);
        self.draw_combar(com_bar, buf);
    }
}

impl JadeApp {
    fn draw_dirbar(&self, area: Rect, buf: &mut Buffer) -> {

    }
}
