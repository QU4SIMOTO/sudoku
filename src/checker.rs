use crate::grid::*;
use std::collections::HashSet;

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

#[cfg(test)]
mod test {
    use super::*;

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
