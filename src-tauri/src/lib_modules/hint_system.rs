use crate::lib_modules::mod_sudoku::SudokuEtat;
use rand::Rng;


#[allow(dead_code)]
pub fn get_hint(state: &SudokuEtat) -> (usize, usize, u8) {
    let mut rng = rand::rng();

    loop {
        let pos = rng.random_range(0..81);
        let row = pos / 9;
        let col = pos % 9;

        if state.sudoku.grille[row][col] == 0 {
            return (row, col, state.solution.grille[row][col]);
        }
    }
}
