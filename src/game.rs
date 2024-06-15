use crate::checker::{Checker, CheckerResult};
use crate::grid::*;
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    widgets::{StatefulWidget, Widget},
};
use std::fmt::Display;

#[derive(Debug, Clone, Copy)]
pub struct Entry {
    pub position: GridPosition,
    pub value: usize,
    pub previous_value: usize,
}

#[derive(Debug)]
pub struct Game {
    pub selected: GridPosition,
    pub invalid_subsections: Vec<GridSubsectionType>,
    is_complete: bool,
    grid: Grid,
    entries: Vec<Entry>,
    checker: Checker,
}

impl Game {
    pub fn new(cells: Vec<usize>) -> Result<Self, GridError> {
        let grid = Grid::new(cells).unwrap();
        Ok(Self::from_grid(grid))
    }

    pub fn from_grid(grid: Grid) -> Self {
        Self {
            grid,
            selected: (0, 0),
            checker: Checker::new(),
            entries: vec![],
            invalid_subsections: vec![],
            is_complete: false,
        }
    }

    pub fn add_entry(&mut self, position: GridPosition, value: usize) -> Result<Entry, GridError> {
        let previous_value = self.grid.get_cell(position)?;
        self.grid.set_cell(position, value)?;
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
            .set_cell(entry.position, entry.previous_value)
            .unwrap();
        self.apply_checker();
        self.selected = entry.position;
        Some(entry)
    }

    pub fn unset_cell(&mut self, position: GridPosition) -> Result<(), GridError> {
        let previous_value = self.grid.set_cell(position, 0)?;
        if previous_value == 0 {
            return Ok(());
        }
        self.entries.push(Entry {
            position,
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

    pub fn is_correct(&self) -> bool {
        self.is_complete && self.invalid_subsections.is_empty()
    }

    pub fn reset(&mut self) {
        self.grid.reset();
        self.is_complete = false;
        self.entries.clear();
        self.apply_checker();
    }
}

impl Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.grid)
    }
}

impl Widget for &Game {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let mut state = GridState {
            selected: self.selected.clone(),
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
