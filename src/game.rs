use crate::checker::{Checker, CheckerResult};
use crate::grid::*;

#[derive(Debug, Clone, Copy)]
pub struct Entry {
    pub x: usize,
    pub y: usize,
    pub value: usize,
    pub previous_value: usize,
}

pub struct Game {
    grid: Grid,
    entries: Vec<Entry>,
    invalid_subsections: Vec<GridSubsectionType>,
    is_complete: bool,
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

    pub fn add_entry(&mut self, x: usize, y: usize, value: usize) -> Result<(), GridError> {
        let previous_value = self.grid.get_cell(x, y)?;
        self.grid.set_cell(x, y, value)?;
        self.entries.push(Entry {
            x,
            y,
            value,
            previous_value,
        });
        self.apply_checker();
        Ok(())
    }

    fn apply_checker(&mut self) {
        for (subsection_type, CheckerResult { valid, complete }) in self
            .checker
            .check_subsections(&self.grid.get_all_subsections())
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
            .set_cell(entry.x, entry.y, entry.previous_value)
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
            x,
            y,
            value: 0,
            previous_value,
        });
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    #[test]
    // TODO: unit tests for game
    fn test() {}
}
