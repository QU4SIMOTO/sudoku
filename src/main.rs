use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{layout::Rect, prelude::*};
use std::io;
use sudoku::game::Game;
use sudoku::grid::Grid;
use sudoku::tui;

fn main() -> io::Result<()> {
    let game = Game::new(
        Grid::new(vec![
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
    );
    let mut terminal = tui::init()?;
    let app_result = App::new(game).run(&mut terminal);
    tui::restore()?;
    app_result
}

pub struct App {
    game: Game,
    selected: (usize, usize),
    exit: bool,
}

enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl App {
    pub fn new(game: Game) -> Self {
        Self {
            game,
            exit: false,
            selected: (0, 0),
        }
    }
    pub fn run(&mut self, terminal: &mut tui::Tui) -> io::Result<()> {
        while !self.exit & !self.game.is_complete {
            terminal.draw(|frame| self.render_frame(frame))?;
            self.handle_events()?;
        }
        Ok(())
    }

    fn render_frame(&mut self, frame: &mut Frame) {
        frame.render_widget(self, frame.size());
    }

    fn handle_events(&mut self) -> io::Result<()> {
        match event::read()? {
            Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
                self.handle_key_event(key_event)
            }
            _ => {}
        };
        Ok(())
    }

    fn handle_key_event(&mut self, key_event: KeyEvent) {
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            KeyCode::Char('l') => self.move_selected(Direction::Right),
            KeyCode::Char('h') => self.move_selected(Direction::Left),
            KeyCode::Char('k') => self.move_selected(Direction::Up),
            KeyCode::Char('j') => self.move_selected(Direction::Down),
            KeyCode::Char('0') => self.set_selected_value(0),
            KeyCode::Char('1') => self.set_selected_value(1),
            KeyCode::Char('2') => self.set_selected_value(2),
            KeyCode::Char('3') => self.set_selected_value(3),
            KeyCode::Char('4') => self.set_selected_value(4),
            KeyCode::Char('5') => self.set_selected_value(5),
            KeyCode::Char('6') => self.set_selected_value(6),
            KeyCode::Char('7') => self.set_selected_value(7),
            KeyCode::Char('8') => self.set_selected_value(8),
            KeyCode::Char('9') => self.set_selected_value(9),
            _ => {}
        }
    }

    fn move_selected(&mut self, direction: Direction) {
        match direction {
            Direction::Left => {
                if self.selected.0 > 0 {
                    self.selected.0 = (self.selected.0 as isize - 1) as usize;
                }
            }
            Direction::Right => {
                if self.selected.0 < 8 {
                    self.selected.0 += 1;
                }
            }
            Direction::Up => {
                if self.selected.1 > 0 {
                    self.selected.1 = (self.selected.1 as isize - 1) as usize;
                }
            }
            Direction::Down => {
                if self.selected.1 < 8 {
                    self.selected.1 += 1;
                }
            }
        }
    }

    fn set_selected_value(&mut self, value: usize) {
        let _ = self.game.add_entry(self.selected, value);
    }

    fn exit(&mut self) {
        self.exit = true;
    }
}

impl Widget for &mut App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        self.game.render(area, buf, &mut self.selected);
    }
}
