use crate::{
    game::{Direction, Game},
    solver::Solver,
    tui,
};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{layout::Rect, prelude::*};

enum Window {
    GameWindow { game: Game },
    SolverWindow { solver: Solver },
}

pub struct App {
    window: Window,
    exit: bool,
}

impl App {
    pub fn new(game: Game) -> Self {
        Self {
            window: Window::GameWindow { game },
            exit: false,
        }
    }

    pub fn run(&mut self, terminal: &mut tui::Tui) -> std::io::Result<()> {
        while !self.exit {
            terminal.draw(|frame| self.render_frame(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn render_frame(&mut self, frame: &mut Frame) {
        frame.render_widget(self, frame.size());
    }

    fn handle_events(&mut self) -> std::io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match self.window {
            Window::GameWindow { ref mut game } => {
                match key_event.code {
                    KeyCode::Char('q') => self.exit(),
                    // move cursor
                    KeyCode::Char('l') => game.move_selected(Direction::Right),
                    KeyCode::Char('h') => game.move_selected(Direction::Left),
                    KeyCode::Char('k') => game.move_selected(Direction::Up),
                    KeyCode::Char('j') => game.move_selected(Direction::Down),
                    // insert number
                    KeyCode::Char('0') | KeyCode::Backspace => game.add_entry_at_selected(0),
                    KeyCode::Char('1') => game.add_entry_at_selected(1),
                    KeyCode::Char('2') => game.add_entry_at_selected(2),
                    KeyCode::Char('3') => game.add_entry_at_selected(3),
                    KeyCode::Char('4') => game.add_entry_at_selected(4),
                    KeyCode::Char('5') => game.add_entry_at_selected(5),
                    KeyCode::Char('6') => game.add_entry_at_selected(6),
                    KeyCode::Char('7') => game.add_entry_at_selected(7),
                    KeyCode::Char('8') => game.add_entry_at_selected(8),
                    KeyCode::Char('9') => game.add_entry_at_selected(9),
                    // other controls
                    KeyCode::Char('u') => {
                        let _ = game.undo_entry();
                    }
                    KeyCode::Char('s') => self.open_solver_window(),
                    _ => {}
                }
            }
            Window::SolverWindow { ref mut solver } => match key_event.code {
                KeyCode::Char('q') => self.exit(),
                KeyCode::Char('n') => solver.next(),
                _ => {}
            },
        }
    }

    fn open_solver_window(&mut self) {
        self.window = Window::SolverWindow {
            solver: Solver::new(
                Game::new(vec![
                    4, 6, 7, 1, 0, 0, 8, 0, 5, // row 0
                    9, 1, 2, 8, 3, 5, 6, 0, 7, // row 1
                    0, 8, 5, 6, 4, 7, 1, 9, 2, // row 2
                    2, 9, 6, 3, 5, 1, 4, 7, 0, // row 3
                    7, 0, 8, 9, 2, 0, 3, 5, 1, // row 4
                    5, 3, 1, 4, 0, 8, 9, 2, 6, // row 5
                    0, 7, 3, 0, 6, 4, 5, 1, 0, // row 6
                    6, 2, 4, 5, 1, 9, 7, 8, 3, // row 7
                    1, 5, 9, 7, 8, 3, 0, 6, 4, // row 8
                ])
                .unwrap(),
            ),
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }
}

impl Widget for &mut App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        match &self.window {
            Window::GameWindow { game } => game.render(area, buf),
            Window::SolverWindow { solver } => solver.render(area, buf),
        }
    }
}
