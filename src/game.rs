use crate::checker::{Checker, CheckerResult};
use crate::grid::*;
use ratatui::{buffer::Buffer, layout::Rect, widgets::StatefulWidget};
use std::fmt::Display;

pub type EntryPosition = (usize, usize);

#[derive(Debug, Clone, Copy)]
pub struct Entry {
    pub position: EntryPosition,
    pub value: usize,
    pub previous_value: usize,
}

pub struct Game {
    pub invalid_subsections: Vec<GridSubsectionType>,
    pub is_complete: bool,
    pub grid: Grid,
    entries: Vec<Entry>,
    checker: Checker,
}

impl Game {
    pub fn new(grid: Grid) -> Self {
        Self {
            grid,
            checker: Checker::new(),
            entries: vec![],
            invalid_subsections: vec![],
            is_complete: false,
        }
    }

    pub fn add_entry(&mut self, position: EntryPosition, value: usize) -> Result<Entry, GridError> {
        let previous_value = self.grid.get_cell(position.0, position.1)?;
        self.grid.set_cell(position.0, position.1, value)?;
        let entry = Entry {
            position,
            value,
            previous_value,
        };
        self.entries.push(entry);
        self.apply_checker();
        Ok(entry)
    }

    fn apply_checker(&mut self) {
        self.invalid_subsections = Vec::new();
        self.is_complete = true;
        for (subsection_type, CheckerResult { valid, complete }) in self
            .checker
            .check_subsections(&self.grid.get_all_subsection_values())
        {
            if !complete {
                self.is_complete = false;
            }
            if !valid {
                self.invalid_subsections.push(subsection_type);
            }
        }
    }

    pub fn undo_entry(&mut self) -> Option<Entry> {
        let entry = self.entries.pop()?;
        self.grid
            .set_cell(entry.position.0, entry.position.1, entry.previous_value)
            .unwrap();
        self.apply_checker();
        Some(entry)
    }

    pub fn unset_cell(&mut self, x: usize, y: usize) -> Result<(), GridError> {
        let previous_value = self.grid.set_cell(x, y, 0)?;
        if previous_value == 0 {
            return Ok(());
        }
        self.entries.push(Entry {
            position: (x, y),
            value: 0,
            previous_value,
        });
        Ok(())
    }

    pub fn get_rows(&self) -> Vec<GridSubsectionValues> {
        self.grid.get_row_values()
    }

    pub fn get_columns(&self) -> Vec<GridSubsectionValues> {
        self.grid.get_column_values()
    }

    pub fn get_square(&self) -> Vec<GridSubsectionValues> {
        self.grid.get_square_values()
    }

    pub fn size(&self) -> usize {
        self.grid.size()
    }
}

impl Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.grid)
    }
}

impl StatefulWidget for &Game {
    type State = (usize, usize);
    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        let mut state = GridState {
            selected: state.clone(),
            subsections: self.invalid_subsections.clone(),
        };
        self.grid.render(area, buf, &mut state);
    }
}

#[cfg(test)]
mod tests {
    #[test]
    // TODO: unit tests for game
    fn test() {}
}
