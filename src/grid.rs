use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span, Text},
    widgets::{StatefulWidget, Widget},
};
use std::collections::HashSet;
use std::fmt::Display;

#[derive(Debug, PartialEq)]
struct Cell {
    value: usize,
    readonly: bool,
}

pub struct GridState {
    pub selected: (usize, usize),
    pub subsections: Vec<GridSubsectionType>,
}

pub type GridPosition = (usize, usize);

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

    pub fn size(&self) -> usize {
        self.side_size
    }

    fn get_cell_index(&self, position: GridPosition) -> Result<usize, GridError> {
        if position.0 >= self.side_size || position.1 >= self.side_size {
            Err(GridError::CellOutOfBounds)
        } else {
            Ok(position.1 * self.side_size + position.0)
        }
    }

    pub fn get_cell(&self, position: GridPosition) -> Result<usize, GridError> {
        let i = self.get_cell_index(position)?;
        Ok(self.cells[i].value)
    }

    pub fn set_cell(&mut self, position: GridPosition, value: usize) -> Result<usize, GridError> {
        let i = self.get_cell_index(position)?;
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

    pub fn get_subsection_values(
        &self,
        subsection_type: GridSubsectionType,
    ) -> GridSubsectionValues {
        GridSubsectionValues::new(&self, subsection_type)
    }

    pub fn get_row_values(&self) -> Vec<GridSubsectionValues> {
        (0..self.side_size)
            .map(|i| self.get_subsection_values(GridSubsectionType::Row(i)))
            .collect()
    }

    pub fn get_column_values(&self) -> Vec<GridSubsectionValues> {
        (0..self.side_size)
            .map(|i| self.get_subsection_values(GridSubsectionType::Column(i)))
            .collect()
    }

    pub fn get_square_values(&self) -> Vec<GridSubsectionValues> {
        (0..self.side_size)
            .map(|i| {
                self.get_subsection_values(GridSubsectionType::Square(
                    i % self.sub_square_size,
                    i / self.sub_square_size,
                ))
            })
            .collect()
    }

    pub fn get_all_subsection_values(&self) -> Vec<GridSubsectionValues> {
        (0..self.side_size)
            .flat_map(|i| {
                [
                    self.get_subsection_values(GridSubsectionType::Row(i)),
                    self.get_subsection_values(GridSubsectionType::Column(i)),
                    self.get_subsection_values(GridSubsectionType::Square(
                        i % self.sub_square_size,
                        i / self.sub_square_size,
                    )),
                ]
            })
            .collect()
    }

    pub fn get_subsections_vaules_for_cell(
        &self,
        position: GridPosition,
    ) -> [GridSubsectionValues; 3] {
        [
            self.get_subsection_values(GridSubsectionType::Row(position.1)),
            self.get_subsection_values(GridSubsectionType::Column(position.0)),
            self.get_subsection_values(GridSubsectionType::Square(position.0 / 3, position.1 / 3)),
        ]
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for i in 0..self.side_size {
            for j in 0..self.side_size {
                match self.get_cell((i, j)).unwrap() {
                    0 if j == 0 => write!(f, "_")?,
                    0 => write!(f, ",_")?,
                    n if j == 0 => write!(f, "{}", n)?,
                    n => write!(f, ",{}", n)?,
                }
            }
            write!(f, "\n")?
        }
        Ok(())
    }
}

impl StatefulWidget for &Grid {
    type State = GridState;

    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State)
    where
        Self: Sized,
    {
        let red_cells: HashSet<(usize, usize)> = state
            .subsections
            .iter()
            .flat_map(|t| GridSubsection::new(self, *t))
            .collect();

        let lines: Vec<Line> = (0..self.side_size)
            .map(|j| {
                let spans = (0..self.side_size)
                    .map(|i| {
                        let is_red = red_cells.contains(&(i, j));
                        let cell = &self.cells[self.get_cell_index((i, j)).unwrap()];
                        let style = if cell.readonly {
                            Style::new().fg(Color::White)
                        } else {
                            Style::new().fg(Color::Blue)
                        };
                        let style = if (i, j) == state.selected {
                            style.bg(Color::DarkGray)
                        } else if is_red {
                            style.bg(Color::Red)
                        } else {
                            style
                        };
                        let cell_string = match cell.value {
                            0 => format!(" _ "),
                            n => format!(" {n} "),
                        };
                        Span::styled(cell_string, style)
                    })
                    .collect::<Vec<Span>>();
                Line::from(spans)
            })
            .collect();
        let text = Text::from(lines);
        text.render(area, buf);
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GridSubsectionType {
    Row(usize),
    Column(usize),
    Square(usize, usize),
}

#[derive(Debug, Clone, Copy)]
pub struct GridSubsection {
    pub subsection_type: GridSubsectionType,
    pub grid_size: usize,
    current: usize,
}

impl GridSubsection {
    fn new(grid: &Grid, subsection_type: GridSubsectionType) -> Self {
        // validate grid
        Self {
            grid_size: grid.side_size,
            subsection_type,
            current: 0,
        }
    }
}

impl Iterator for GridSubsection {
    type Item = (usize, usize);

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
        Some((x, y))
    }
}

#[derive(Debug, Clone, Copy)]
pub struct GridSubsectionValues<'a> {
    grid: &'a Grid,
    pub grid_subsection: GridSubsection,
}

impl<'a> GridSubsectionValues<'a> {
    fn new(grid: &'a Grid, subsection_type: GridSubsectionType) -> Self {
        Self {
            grid,
            grid_subsection: GridSubsection::new(grid, subsection_type),
        }
    }
}

impl<'a> Iterator for GridSubsectionValues<'a> {
    type Item = usize;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(position) = self.grid_subsection.next() {
            Some(self.grid.get_cell(position).unwrap())
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
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
        assert_eq!(grid.get_cell((10, 0)), Err(GridError::CellOutOfBounds));
        assert_eq!(grid.get_cell((0, 10)), Err(GridError::CellOutOfBounds));
        assert_eq!(grid.get_cell((0, 0)), Ok(2));
        assert_eq!(grid.get_cell((8, 0)), Ok(1));
        assert_eq!(grid.get_cell((8, 8)), Ok(9));
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
        assert_eq!(grid.set_cell((9, 9), 3), Err(GridError::CellOutOfBounds));
        assert_eq!(
            grid.set_cell((0, 0), 3),
            Err(GridError::ReadonlyCellMutation)
        );
        assert_eq!(grid.set_cell((1, 1), 6), Ok(0));
        assert_eq!(grid.get_cell((1, 1)), Ok(6));
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
        grid.set_cell((1, 1), 6).unwrap();
        assert_eq!(grid.get_cell((1, 1)), Ok(6));
        grid.reset();
        assert_eq!(grid.get_cell((0, 0)), Ok(2));
        assert_eq!(grid.get_cell((1, 1)), Ok(0));
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
            grid.get_subsection_values(GridSubsectionType::Row(0))
                .collect::<Vec<_>>(),
            vec![2, 0, 0, 0, 0, 0, 0, 6, 1]
        );
        assert_eq!(
            grid.get_subsection_values(GridSubsectionType::Row(8))
                .collect::<Vec<_>>(),
            vec![1, 0, 0, 0, 0, 0, 0, 7, 9]
        );
        assert_eq!(
            grid.get_subsection_values(GridSubsectionType::Column(0))
                .collect::<Vec<_>>(),
            vec![2, 0, 0, 0, 0, 0, 0, 0, 1]
        );
        assert_eq!(
            grid.get_subsection_values(GridSubsectionType::Column(8))
                .collect::<Vec<_>>(),
            vec![1, 0, 0, 0, 0, 0, 0, 8, 9]
        );
        assert_eq!(
            grid.get_subsection_values(GridSubsectionType::Square(0, 0))
                .collect::<Vec<_>>(),
            vec![2, 0, 0, 0, 0, 0, 0, 0, 3]
        );
        assert_eq!(
            grid.get_subsection_values(GridSubsectionType::Square(2, 2))
                .collect::<Vec<_>>(),
            vec![0, 0, 0, 0, 0, 8, 0, 7, 9]
        );
    }

    #[test]
    fn get_row() {
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
        let rows = grid.get_row_values();
        assert_eq!(
            rows[0].collect::<Vec<_>>(),
            vec![2, 0, 0, 0, 0, 0, 0, 6, 1,]
        );
        assert_eq!(
            rows[1].collect::<Vec<_>>(),
            vec![0, 0, 0, 0, 0, 0, 0, 0, 0,]
        );
        assert_eq!(
            rows[8].collect::<Vec<_>>(),
            vec![1, 0, 0, 0, 0, 0, 0, 7, 9,]
        );
    }
}
