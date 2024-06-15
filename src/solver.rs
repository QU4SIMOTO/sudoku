use crate::{
    game::{Entry, Game},
    grid::GridPosition,
};

pub struct Solver {
    pub game: Game,
    empty_positions: Vec<GridPosition>,
    entries_added: Vec<Entry>,
}

impl Solver {
    pub fn new(game: Game) -> Self {
        // TODO: handle game with entries
        let empty_positions: Vec<GridPosition> = game
            .get_rows()
            .into_iter()
            .enumerate()
            .flat_map(|(y, row)| {
                row.into_iter()
                    .enumerate()
                    .filter_map(|(x, value)| match value {
                        0 => Some((x, y)),
                        _ => None,
                    })
                    .collect::<Vec<_>>()
            })
            .collect();
        Self {
            game,
            empty_positions,
            entries_added: Vec::new(),
        }
    }

    pub fn next(&mut self) {
        if self.game.is_correct() {
            return;
        }
        if self.game.invalid_subsections.len() == 0 {
            self.entries_added.push(
                self.game
                    .add_entry(self.empty_positions.pop().unwrap(), 1)
                    .unwrap(),
            );
        }
        let Entry {
            position, value, ..
        } = self
            .entries_added
            .pop()
            // TODO: handle this better
            .expect("Game isn't solvable or was given in invalid state");
        let next_value = if value + 1 <= self.game.size() {
            value + 1
        } else {
            self.empty_positions.push(position);
            0
        };
        self.entries_added
            .push(self.game.add_entry(position, next_value).unwrap());
    }

    pub fn solve(game: Game) -> Game {
        let mut solver = Self::new(game);
        while !solver.game.is_correct() {
            solver.next();
        }
        solver.game
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solves_a_valid_game() {
        assert_eq!(
            Solver::solve(
                Game::new(vec![
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
            )
            .get_rows()
            .into_iter()
            .flat_map(|t| t.collect::<Vec<_>>())
            .collect::<Vec<_>>(),
            vec![
                4, 6, 7, 1, 9, 2, 8, 3, 5, // row 0
                9, 1, 2, 8, 3, 5, 6, 4, 7, // row 1
                3, 8, 5, 6, 4, 7, 1, 9, 2, // row 2
                2, 9, 6, 3, 5, 1, 4, 7, 8, // row 3
                7, 4, 8, 9, 2, 6, 3, 5, 1, // row 4
                5, 3, 1, 4, 7, 8, 9, 2, 6, // row 5
                8, 7, 3, 2, 6, 4, 5, 1, 9, // row 6
                6, 2, 4, 5, 1, 9, 7, 8, 3, // row 7
                1, 5, 9, 7, 8, 3, 2, 6, 4, // row 8
            ]
        );
    }
}
