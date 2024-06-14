use crate::game::{Entry, EntryPosition, Game};

pub struct Solver {
    pub game: Game,
    empty_positions: Vec<EntryPosition>,
    entries_added: Vec<Entry>,
}

impl Solver {
    pub fn new(game: Game) -> Self {
        // TODO: handle game with entries
        let empty_positions: Vec<(usize, usize)> = game
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
        if self.game.is_complete {
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
}
