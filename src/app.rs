use crate::{
    game::{Direction, Game},
    tui,
};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{layout::Rect, prelude::*};

pub struct App {
    game: Game,
    exit: bool,
}

impl App {
    pub fn new(game: Game) -> Self {
        Self { game, exit: false }
    }
    pub fn run(&mut self, terminal: &mut tui::Tui) -> std::io::Result<()> {
        while !self.exit & !self.game.is_correct() {
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
        match key_event.code {
            KeyCode::Char('q') => self.exit(),
            // move cursor
            KeyCode::Char('l') => self.game.move_selected(Direction::Right),
            KeyCode::Char('h') => self.game.move_selected(Direction::Left),
            KeyCode::Char('k') => self.game.move_selected(Direction::Up),
            KeyCode::Char('j') => self.game.move_selected(Direction::Down),
            // insert number
            KeyCode::Char('0') | KeyCode::Backspace => self.game.add_entry_at_selected(0),
            KeyCode::Char('1') => self.game.add_entry_at_selected(1),
            KeyCode::Char('2') => self.game.add_entry_at_selected(2),
            KeyCode::Char('3') => self.game.add_entry_at_selected(3),
            KeyCode::Char('4') => self.game.add_entry_at_selected(4),
            KeyCode::Char('5') => self.game.add_entry_at_selected(5),
            KeyCode::Char('6') => self.game.add_entry_at_selected(6),
            KeyCode::Char('7') => self.game.add_entry_at_selected(7),
            KeyCode::Char('8') => self.game.add_entry_at_selected(8),
            KeyCode::Char('9') => self.game.add_entry_at_selected(9),
            // other controls
            KeyCode::Char('u') => {
                let _ = self.game.undo_entry();
            }
            _ => {}
        }
    }

    fn exit(&mut self) {
        self.exit = true;
    }
}

impl Widget for &mut App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        self.game.render(area, buf)
    }
}
