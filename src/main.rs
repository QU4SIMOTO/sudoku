use std::collections::HashSet;

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
    errors: Vec<GridSubsectionType>,
    checker: Checker,
}

impl Game {
    pub fn new(grid: Grid) -> Self {
        Self {
            grid,
            checker: Checker::new(),
            entries: vec![],
            errors: vec![],
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
        self.errors = self
            .checker
            .check_subsections(&self.grid.get_all_subsections());
        Ok(())
    }

    pub fn undo_entry(&mut self) -> Option<Entry> {
        let entry = self.entries.pop()?;
        self.grid
            .set_cell(entry.x, entry.y, entry.previous_value)
            .unwrap();
        self.errors = self
            .checker
            .check_subsections(&self.grid.get_all_subsections());
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

#[derive(Debug, PartialEq, Eq)]
pub enum GridError {
    InvalidGridSize,
    CellOutOfBounds,
    InvalidCellValue(usize),
    ReadonlyCellMutation,
    InvalidRowNumber,
    InvalidColumnNumber,
    InvalidSquareNumber,
}

#[derive(Debug, PartialEq)]
pub struct Cell {
    value: usize,
    readonly: bool,
}

#[derive(Debug, PartialEq)]
pub struct Grid {
    cells: Vec<Cell>,
    side_size: usize,
    sub_square_size: usize,
}

fn square_root(n: usize) -> Option<usize> {
    let root = (n as f64).sqrt();
    let i = root.floor();
    if root > i {
        None
    } else {
        Some(i as usize)
    }
}

impl Grid {
    pub fn new(cells: Vec<usize>) -> Result<Self, GridError> {
        if cells.len() == 1 {
            return Err(GridError::InvalidGridSize);
        }
        let side_size = square_root(cells.len()).ok_or(GridError::InvalidGridSize)?;
        let sub_square_size = square_root(side_size).ok_or(GridError::InvalidGridSize)?;
        dbg!(side_size, sub_square_size);

        let cells = cells
            .iter()
            .enumerate()
            .map(|(i, cell_value)| {
                if *cell_value > side_size {
                    return Err(GridError::InvalidCellValue(i));
                }
                Ok(Cell {
                    value: *cell_value,
                    readonly: *cell_value != 0,
                })
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Self {
            cells,
            side_size,
            sub_square_size,
        })
    }

    fn get_cell_index(&self, x: usize, y: usize) -> Result<usize, GridError> {
        if x >= self.side_size || y >= self.side_size {
            Err(GridError::CellOutOfBounds)
        } else {
            Ok(y * self.side_size + x)
        }
    }

    pub fn get_cell(&self, x: usize, y: usize) -> Result<usize, GridError> {
        let i = self.get_cell_index(x, y)?;
        Ok(self.cells[i].value)
    }

    pub fn set_cell(&mut self, x: usize, y: usize, value: usize) -> Result<usize, GridError> {
        let i = self.get_cell_index(x, y)?;
        let cell = &mut self.cells[i];
        if cell.readonly {
            return Err(GridError::ReadonlyCellMutation);
        }
        let previous_value = cell.value;
        cell.value = value;
        Ok(previous_value)
    }

    pub fn reset(&mut self) {
        for cell in self.cells.iter_mut().filter(|cell| !cell.readonly) {
            cell.value = 0;
        }
    }

    pub fn get_subsection(&self, subsection_type: GridSubsectionType) -> GridSubSection {
        GridSubSection::new(&self, subsection_type)
    }

    pub fn get_all_subsections(&self) -> Vec<GridSubSection> {
        (0..self.side_size)
            .flat_map(|i| {
                [
                    self.get_subsection(GridSubsectionType::Row(i)),
                    self.get_subsection(GridSubsectionType::Column(i)),
                    self.get_subsection(GridSubsectionType::Square(
                        i % self.sub_square_size,
                        i / self.sub_square_size,
                    )),
                ]
            })
            .collect()
    }

    pub fn get_subsections_for_cell(&self, x: usize, y: usize) -> [GridSubSection; 3] {
        [
            self.get_subsection(GridSubsectionType::Row(y)),
            self.get_subsection(GridSubsectionType::Column(x)),
            self.get_subsection(GridSubsectionType::Square(x / 3, y / 3)),
        ]
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GridSubsectionType {
    Row(usize),
    Column(usize),
    Square(usize, usize),
}

#[derive(Debug, Clone, Copy)]
pub struct GridSubSection<'a> {
    grid: &'a Grid,
    current: usize,
    pub subsection_type: GridSubsectionType,
}

impl<'a> GridSubSection<'a> {
    fn new(grid: &'a Grid, subsection_type: GridSubsectionType) -> Self {
        // validate grid
        Self {
            grid,
            subsection_type,
            current: 0,
        }
    }
}

impl<'a> Iterator for GridSubSection<'a> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current > 8 {
            return None;
        }
        let (x, y) = match self.subsection_type {
            GridSubsectionType::Row(j) => (self.current, j),
            GridSubsectionType::Column(i) => (i, self.current),
            GridSubsectionType::Square(i, j) => {
                let x = i * 3 + (self.current % 3);
                let y = j * 3 + (self.current / 3);
                (x, y)
            }
        };
        self.current += 1;
        Some(self.grid.get_cell(x, y).unwrap())
    }
}

pub struct Checker {
    values: HashSet<usize>,
}

impl Checker {
    pub fn new() -> Self {
        Self {
            values: HashSet::new(),
        }
    }

    pub fn check_subsection(&mut self, subsection: &GridSubSection) -> bool {
        self.values.clear();
        for value in subsection
            .filter(|&value| value != 0)
            .map(|value| value - 1)
        {
            if self.values.contains(&value) {
                return false;
            }
            self.values.insert(value);
        }
        true
    }

    pub fn check_subsections(&mut self, subsections: &[GridSubSection]) -> Vec<GridSubsectionType> {
        subsections
            .iter()
            .filter(|&subsection| !self.check_subsection(subsection))
            .map(|&subsection| subsection.subsection_type)
            .collect()
    }
}

fn main() {}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn invalid_grid_size() {
        assert_eq!(Grid::new(vec![0]), Err(GridError::InvalidGridSize));
        assert_eq!(Grid::new(vec![0, 0, 0]), Err(GridError::InvalidGridSize));
        assert_eq!(
            Grid::new(vec![
                0, 0, 0, // row 0
                0, 0, 0, // row 1
            ]),
            Err(GridError::InvalidGridSize)
        );
        assert_eq!(
            Grid::new(vec![
                0, 0, 0, // row 0
                0, 0, 0, // row 1
                0, 0, 0, // row 3
            ]),
            Err(GridError::InvalidGridSize)
        );
    }

    #[test]
    fn get_cell() {
        let grid = Grid::new(vec![
            2, 0, 0, 0, 0, 0, 0, 0, 1, // row 0
            0, 0, 0, 0, 0, 0, 0, 0, 0, // row 1
            0, 0, 3, 0, 1, 0, 0, 0, 0, // row 2
            0, 0, 0, 0, 0, 0, 0, 0, 0, // row 3
            0, 0, 0, 0, 0, 0, 0, 0, 0, // row 4
            0, 0, 0, 0, 0, 0, 0, 0, 0, // row 5
            0, 0, 0, 0, 0, 0, 0, 0, 0, // row 6
            0, 0, 0, 0, 0, 0, 0, 0, 0, // row 7
            1, 0, 0, 0, 0, 0, 0, 0, 9, // row 8
        ])
        .unwrap();
        assert_eq!(grid.get_cell(10, 0), Err(GridError::CellOutOfBounds));
        assert_eq!(grid.get_cell(0, 10), Err(GridError::CellOutOfBounds));
        assert_eq!(grid.get_cell(0, 0), Ok(2));
        assert_eq!(grid.get_cell(8, 0), Ok(1));
        assert_eq!(grid.get_cell(8, 8), Ok(9));
    }

    #[test]
    fn set_cell() {
        let mut grid = Grid::new(vec![
            2, 0, 0, 0, 0, 0, 0, 0, 1, // row 0
            0, 0, 0, 0, 0, 0, 0, 0, 0, // row 1
            0, 0, 0, 0, 1, 0, 0, 0, 0, // row 2
            0, 0, 0, 0, 0, 0, 0, 0, 0, // row 3
            0, 0, 0, 0, 0, 0, 0, 0, 0, // row 4
            0, 0, 0, 0, 0, 0, 0, 0, 0, // row 5
            0, 0, 0, 0, 0, 0, 0, 0, 0, // row 6
            0, 0, 0, 0, 0, 0, 0, 0, 0, // row 7
            1, 0, 0, 0, 0, 0, 0, 0, 9, // row 8
        ])
        .unwrap();
        assert_eq!(grid.set_cell(9, 9, 3), Err(GridError::CellOutOfBounds));
        assert_eq!(grid.set_cell(0, 0, 3), Err(GridError::ReadonlyCellMutation));
        assert_eq!(grid.set_cell(1, 1, 6), Ok(0));
        assert_eq!(grid.get_cell(1, 1), Ok(6));
    }

    #[test]
    fn reset() {
        let mut grid = Grid::new(vec![
            2, 0, 0, 0, 0, 0, 0, 0, 1, // row 0
            0, 0, 0, 0, 0, 0, 0, 0, 0, // row 1
            0, 0, 0, 0, 1, 0, 0, 0, 0, // row 2
            0, 0, 0, 0, 0, 0, 0, 0, 0, // row 3
            0, 0, 0, 0, 0, 0, 0, 0, 0, // row 4
            0, 0, 0, 0, 0, 0, 0, 0, 0, // row 5
            0, 0, 0, 0, 0, 0, 0, 0, 0, // row 6
            0, 0, 0, 0, 0, 0, 0, 0, 0, // row 7
            1, 0, 0, 0, 0, 0, 0, 0, 9, // row 8
        ])
        .unwrap();
        grid.set_cell(1, 1, 6).unwrap();
        assert_eq!(grid.get_cell(1, 1), Ok(6));
        grid.reset();
        assert_eq!(grid.get_cell(0, 0), Ok(2));
        assert_eq!(grid.get_cell(1, 1), Ok(0));
    }

    #[test]
    fn get_subsection() {
        let grid = Grid::new(vec![
            2, 0, 0, 0, 0, 0, 0, 6, 1, // row 0
            0, 0, 0, 0, 0, 0, 0, 0, 0, // row 1
            0, 0, 3, 0, 1, 0, 0, 0, 0, // row 2
            0, 0, 0, 0, 0, 0, 0, 0, 0, // row 3
            0, 0, 0, 0, 0, 0, 0, 0, 0, // row 4
            0, 0, 0, 0, 0, 0, 0, 0, 0, // row 5
            0, 0, 0, 0, 0, 0, 0, 0, 0, // row 6
            0, 0, 0, 0, 0, 0, 0, 0, 8, // row 7
            1, 0, 0, 0, 0, 0, 0, 7, 9, // row 8
        ])
        .unwrap();
        assert_eq!(
            grid.get_subsection(GridSubsectionType::Row(0))
                .collect::<Vec<_>>(),
            vec![2, 0, 0, 0, 0, 0, 0, 6, 1]
        );
        assert_eq!(
            grid.get_subsection(GridSubsectionType::Row(8))
                .collect::<Vec<_>>(),
            vec![1, 0, 0, 0, 0, 0, 0, 7, 9]
        );
        assert_eq!(
            grid.get_subsection(GridSubsectionType::Column(0))
                .collect::<Vec<_>>(),
            vec![2, 0, 0, 0, 0, 0, 0, 0, 1]
        );
        assert_eq!(
            grid.get_subsection(GridSubsectionType::Column(8))
                .collect::<Vec<_>>(),
            vec![1, 0, 0, 0, 0, 0, 0, 8, 9]
        );
        assert_eq!(
            grid.get_subsection(GridSubsectionType::Square(0, 0))
                .collect::<Vec<_>>(),
            vec![2, 0, 0, 0, 0, 0, 0, 0, 3]
        );
        assert_eq!(
            grid.get_subsection(GridSubsectionType::Square(2, 2))
                .collect::<Vec<_>>(),
            vec![0, 0, 0, 0, 0, 8, 0, 7, 9]
        );
    }

    #[test]
    fn check_subsections() {
        let mut checker = Checker::new();
        assert_eq!(
            checker.check_subsections(
                &Grid::new(vec![
                    2, 0, 0, 0, 0, 0, 0, 6, 1, // row 0
                    0, 0, 0, 0, 0, 0, 0, 0, 0, // row 1
                    0, 0, 3, 0, 1, 0, 0, 0, 0, // row 2
                    0, 0, 0, 0, 0, 0, 0, 0, 0, // row 3
                    0, 0, 0, 0, 0, 0, 0, 0, 0, // row 4
                    0, 0, 0, 0, 0, 0, 0, 0, 0, // row 5
                    0, 0, 0, 0, 0, 0, 0, 0, 0, // row 6
                    0, 0, 0, 0, 0, 0, 0, 0, 8, // row 7
                    1, 0, 0, 0, 0, 0, 0, 7, 9, // row 8
                ])
                .unwrap()
                .get_all_subsections()
            ),
            vec![]
        );
        assert_eq!(
            checker.check_subsections(
                &Grid::new(vec![
                    7, 2, 6, 4, 9, 3, 8, 1, 5, // row 0
                    3, 1, 5, 7, 2, 8, 9, 4, 6, // row 1
                    4, 8, 9, 6, 5, 1, 2, 3, 7, // row 2
                    8, 5, 2, 1, 4, 7, 6, 9, 3, // row 3
                    6, 7, 3, 9, 8, 5, 1, 2, 4, // row 4
                    9, 4, 1, 3, 6, 2, 7, 5, 8, // row 5
                    1, 9, 4, 8, 3, 6, 5, 7, 2, // row 6
                    5, 6, 7, 2, 1, 4, 3, 8, 9, // row 7
                    2, 3, 8, 5, 7, 9, 4, 6, 1, // row 8
                ])
                .unwrap()
                .get_all_subsections()
            ),
            vec![]
        );
        assert_eq!(
            checker.check_subsections(
                &Grid::new(vec![
                    2, 0, 0, 1, 0, 0, 0, 6, 1, // row 0
                    0, 0, 0, 0, 0, 0, 0, 0, 0, // row 1
                    0, 0, 3, 0, 1, 0, 0, 0, 0, // row 2
                    0, 0, 0, 0, 0, 0, 0, 0, 0, // row 3
                    0, 0, 0, 0, 0, 0, 0, 0, 0, // row 4
                    0, 0, 0, 0, 0, 0, 0, 0, 0, // row 5
                    0, 0, 0, 1, 0, 0, 0, 0, 0, // row 6
                    0, 0, 0, 0, 0, 0, 0, 0, 8, // row 7
                    1, 0, 0, 0, 0, 0, 7, 7, 9, // row 8
                ])
                .unwrap()
                .get_all_subsections()
            ),
            vec![
                GridSubsectionType::Row(0),
                GridSubsectionType::Square(1, 0),
                GridSubsectionType::Column(3),
                GridSubsectionType::Row(8),
                GridSubsectionType::Square(2, 2)
            ]
        );
    }
}
