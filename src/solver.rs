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
            .flat_map(|(i, row)| {
                row.into_iter()
                    .enumerate()
                    .filter_map(|(j, value)| match value {
                        0 => Some((j, i)),
                        _ => None,
                    })
                    .collect::<Vec<_>>()
            })
            .rev()
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
        if self.game.invalid_subsections.len() > 0 {
            let last_entry = self
                .entries_added
                .pop()
                // TODO: handle this better
                .expect("Game isn't solvable or was given in invalid state");
            let next_value = last_entry.value + 1;
            if next_value > self.game.size() {
                self.empty_positions.push(last_entry.position);
                self.game.add_entry(last_entry.position, 0).unwrap();
                return;
            }
            self.entries_added.push(
                self.game
                    .add_entry(last_entry.position, next_value)
                    .unwrap(),
            );
            return;
        }
        self.entries_added.push(
            self.game
                .add_entry(self.empty_positions.pop().unwrap(), 1)
                .unwrap(),
        );
    }
}
