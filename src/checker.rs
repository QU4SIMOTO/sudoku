use crate::grid::*;
use std::collections::HashSet;

#[derive(Debug)]
pub struct Checker {
    values: HashSet<usize>,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct CheckerResult {
    pub complete: bool,
    pub valid: bool,
}

impl Checker {
    pub fn new() -> Self {
        Self {
            values: HashSet::new(),
        }
    }

    pub fn check_subsection(&mut self, subsection: &GridSubsectionValues) -> CheckerResult {
        self.values.clear();
        let valid = subsection.fold(true, |acc, curr| {
            if curr != 0 && self.values.contains(&curr) {
                false
            } else {
                self.values.insert(curr);
                acc
            }
        });
        return CheckerResult {
            complete: !self.values.contains(&0),
            valid,
        };
    }

    pub fn check_subsections(
        &mut self,
        subsections: &[GridSubsectionValues],
    ) -> Vec<(GridSubsectionType, CheckerResult)> {
        subsections
            .iter()
            .map(|&subsection| {
                (
                    subsection.grid_subsection.subsection_type,
                    self.check_subsection(&subsection),
                )
            })
            .collect()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_subsections_valid() {
        let mut checker = Checker::new();
        let grid = Grid::new(vec![
            7, 2, 6, 4, 9, 3, 8, 1, 5, // row 0
            3, 1, 5, 7, 2, 8, 9, 4, 6, // row 1
            4, 8, 9, 6, 5, 1, 2, 3, 7, // row 2
            8, 5, 2, 1, 4, 7, 6, 9, 3, // row 3
            6, 7, 3, 9, 8, 5, 1, 2, 4, // row 4
            9, 4, 1, 3, 6, 2, 7, 5, 8, // row 5
            1, 9, 4, 8, 3, 6, 5, 7, 2, // row 6
            5, 6, 7, 2, 1, 4, 3, 8, 0, // row 7
            2, 3, 8, 5, 7, 9, 4, 0, 1, // row 8
        ])
        .unwrap();
        assert_eq!(
            checker.check_subsections(&[
                grid.get_subsection_values(GridSubsectionType::Row(0)),
                grid.get_subsection_values(GridSubsectionType::Column(0)),
                grid.get_subsection_values(GridSubsectionType::Square(0, 0)),
                grid.get_subsection_values(GridSubsectionType::Row(8)),
                grid.get_subsection_values(GridSubsectionType::Column(8)),
                grid.get_subsection_values(GridSubsectionType::Square(2, 2))
            ]),
            vec![
                (
                    GridSubsectionType::Row(0),
                    CheckerResult {
                        valid: true,
                        complete: true
                    }
                ),
                (
                    GridSubsectionType::Column(0),
                    CheckerResult {
                        valid: true,
                        complete: true
                    }
                ),
                (
                    GridSubsectionType::Square(0, 0),
                    CheckerResult {
                        valid: true,
                        complete: true
                    }
                ),
                (
                    GridSubsectionType::Row(8),
                    CheckerResult {
                        valid: true,
                        complete: false
                    }
                ),
                (
                    GridSubsectionType::Column(8),
                    CheckerResult {
                        valid: true,
                        complete: false
                    }
                ),
                (
                    GridSubsectionType::Square(2, 2),
                    CheckerResult {
                        valid: true,
                        complete: false
                    }
                )
            ]
        );
    }

    #[test]
    fn check_subsections_invalid() {
        let mut checker = Checker::new();
        let grid = Grid::new(vec![
            7, 2, 7, 4, 9, 3, 8, 1, 5, // row 0
            3, 9, 5, 7, 2, 8, 9, 4, 6, // row 1
            6, 8, 9, 6, 5, 1, 2, 3, 7, // row 2
            8, 5, 2, 1, 4, 7, 6, 9, 3, // row 3
            6, 7, 3, 9, 8, 5, 1, 2, 4, // row 4
            9, 4, 1, 3, 6, 2, 7, 5, 8, // row 5
            1, 9, 4, 8, 3, 6, 5, 7, 2, // row 6
            5, 6, 7, 2, 1, 4, 3, 8, 0, // row 7
            2, 3, 8, 5, 7, 9, 4, 0, 8, // row 8
        ])
        .unwrap();
        assert_eq!(
            checker.check_subsections(&[
                grid.get_subsection_values(GridSubsectionType::Row(0)),
                grid.get_subsection_values(GridSubsectionType::Column(0)),
                grid.get_subsection_values(GridSubsectionType::Square(0, 0)),
                grid.get_subsection_values(GridSubsectionType::Row(8)),
                grid.get_subsection_values(GridSubsectionType::Column(8)),
                grid.get_subsection_values(GridSubsectionType::Square(2, 2))
            ]),
            vec![
                (
                    GridSubsectionType::Row(0),
                    CheckerResult {
                        valid: false,
                        complete: true
                    }
                ),
                (
                    GridSubsectionType::Column(0),
                    CheckerResult {
                        valid: false,
                        complete: true
                    }
                ),
                (
                    GridSubsectionType::Square(0, 0),
                    CheckerResult {
                        valid: false,
                        complete: true
                    }
                ),
                (
                    GridSubsectionType::Row(8),
                    CheckerResult {
                        valid: false,
                        complete: false
                    }
                ),
                (
                    GridSubsectionType::Column(8),
                    CheckerResult {
                        valid: false,
                        complete: false
                    }
                ),
                (
                    GridSubsectionType::Square(2, 2),
                    CheckerResult {
                        valid: false,
                        complete: false
                    }
                )
            ]
        );
    }
}
