use sudoku::{app::App, game::Game, grid::Grid, tui};

fn main() -> std::io::Result<()> {
    let game = Game::new(vec![
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
    .unwrap();
    let mut terminal = tui::init()?;
    let app_result = App::new(game).run(&mut terminal);
    tui::restore()?;
    app_result
}
