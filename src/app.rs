use crate::{
    game::{Direction, Game},
    solver::Solver,
    tui,
};
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    layout::Rect,
    prelude::*,
    widgets::{block::Title, Block, Borders, Paragraph},
};

enum Window {
    Game { game: Game },
    Solver { solver: Solver },
    Menu,
}

pub struct App {
    window: Window,
    exit: bool,
}

const DUMMY_CELLS: [usize; 81] = [
    4, 6, 7, 1, 0, 0, 8, 0, 5, // row 0
    9, 1, 2, 8, 3, 5, 6, 0, 7, // row 1
    0, 8, 5, 6, 4, 7, 1, 9, 2, // row 2
    2, 9, 6, 3, 5, 1, 4, 7, 0, // row 3
    7, 0, 8, 9, 2, 0, 3, 5, 1, // row 4
    5, 3, 1, 4, 0, 8, 9, 2, 6, // row 5
    0, 7, 3, 0, 6, 4, 5, 1, 0, // row 6
    6, 2, 4, 5, 1, 9, 7, 8, 3, // row 7
    1, 5, 9, 7, 8, 3, 0, 6, 4, // row 8
];

impl App {
    pub fn new() -> Self {
        Self {
            window: Window::Menu,
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
            Window::Game { ref mut game } => {
                match key_event.code {
                    KeyCode::Char('q') => self.open_menu_window(),
                    // move cursor
                    KeyCode::Char('l') | KeyCode::Right => game.move_selected(Direction::Right),
                    KeyCode::Char('h') | KeyCode::Left => game.move_selected(Direction::Left),
                    KeyCode::Char('k') | KeyCode::Up => game.move_selected(Direction::Up),
                    KeyCode::Char('j') | KeyCode::Down => game.move_selected(Direction::Down),
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
                    _ => {}
                }
            }
            Window::Solver { ref mut solver } => match key_event.code {
                KeyCode::Char('q') => self.open_menu_window(),
                KeyCode::Char('n') => solver.next(),
                _ => {}
            },
            Window::Menu => match key_event.code {
                KeyCode::Char('q') => self.exit(),
                KeyCode::Char('g') => self.open_game_window(),
                KeyCode::Char('s') => self.open_solver_window(),
                _ => {}
            },
        }
    }

    fn open_solver_window(&mut self) {
        self.window = Window::Solver {
            solver: Solver::new(Game::new(Vec::from(DUMMY_CELLS)).unwrap()),
        }
    }

    fn open_game_window(&mut self) {
        self.window = Window::Game {
            game: Game::new(Vec::from(DUMMY_CELLS)).unwrap(),
        };
    }

    fn open_menu_window(&mut self) {
        self.window = Window::Menu;
    }

    fn exit(&mut self) {
        self.exit = true;
    }
}

impl Widget for &mut App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        match &self.window {
            Window::Game { game } => {
                let title = Title::from(" Sudoku Game".bold());
                let instructions = Title::from(Line::from(vec![
                    " Move selection ".into(),
                    "<h>/<j>/<k>/<l>".blue().bold(),
                    " Insert number ".into(),
                    "<1-9>".blue().bold(),
                    " Clear cell ".into(),
                    "<0>/<BackSpace>".blue().bold(),
                    " Undo ".into(),
                    "<u> ".blue().bold(),
                    " Quit to menu ".into(),
                    "<q> ".blue().bold(),
                ]));
                let block = Block::default()
                    .title(title.alignment(Alignment::Center))
                    .title(instructions.alignment(Alignment::Center))
                    .title_position(ratatui::widgets::block::Position::Bottom)
                    .borders(Borders::ALL);
                let layout = Layout::new(
                    layout::Direction::Vertical,
                    [Constraint::Percentage(80), Constraint::Percentage(20)],
                )
                .split(area);
                game.render(layout[0], buf);
                block.render(layout[1], buf);
            }
            Window::Solver { solver } => {
                let title = Title::from(" Sudoku Solver".bold());
                let instructions = Title::from(Line::from(vec![
                    " Next ".into(),
                    "<n>".blue().bold(),
                    " Quit to menu ".into(),
                    "<q> ".blue().bold(),
                ]));
                let block = Block::default()
                    .title(title.alignment(Alignment::Center))
                    .title(instructions.alignment(Alignment::Center))
                    .title_position(ratatui::widgets::block::Position::Bottom)
                    .borders(Borders::ALL);
                let layout = Layout::new(
                    layout::Direction::Vertical,
                    [Constraint::Percentage(80), Constraint::Percentage(20)],
                )
                .split(area);
                solver.render(layout[0], buf);
                block.render(layout[1], buf);
            }
            Window::Menu => {
                let title = Title::from(" Sudoku Main Menu ".bold());
                let instructions = Title::from(Line::from(vec![
                    " Game ".into(),
                    "<g>".blue().bold(),
                    " Solver ".into(),
                    "<s>".blue().bold(),
                    " Quit ".into(),
                    "<q> ".blue().bold(),
                ]));
                let block = Block::default()
                    .title(title.alignment(Alignment::Center))
                    .title(instructions.alignment(Alignment::Center))
                    .title_position(ratatui::widgets::block::Position::Bottom)
                    .borders(Borders::ALL);

                Paragraph::new("TODO add different starting grids to select")
                    .centered()
                    .block(block)
                    .render(area, buf);
            }
        }
    }
}
