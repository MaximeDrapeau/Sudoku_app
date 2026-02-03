use crate::lib_modules::mod_sudoku::{Sudoku, SudokuEtat};
use crate::lib_modules::resolver::{has_unique_solution, resolve_sudoku};
use rand::Rng;

pub fn generate() -> SudokuEtat {
    let sudoku = Sudoku::new();
    let mut state = SudokuEtat::new(sudoku);

    state = resolve_sudoku(state);
    state.solution = state.sudoku;

    let mut rng = rand::rng();
    let mut forbidden = [[false; 9]; 9];

    while state.sudoku.count_zeroes() < 55 {
        let (r, c) = loop {
            let pos = rng.random_range(0..81);
            let row = pos / 9;
            let col = pos % 9;

            if state.sudoku.grille[row][col] != 0 && !forbidden[row][col] {
                break (row, col);
            }
        };

        let r2 = 8 - r;
        let c2 = 8 - c;

        let old1 = state.sudoku.grille[r][c];
        let old2 = state.sudoku.grille[r2][c2];

        state.sudoku.grille[r][c] = 0;
        state.sudoku.grille[r2][c2] = 0;

        let sudoku_copy = state.sudoku;

        if !has_unique_solution(sudoku_copy) {
            state.sudoku.grille[r][c] = old1;
            state.sudoku.grille[r2][c2] = old2;

            forbidden[r][c] = true;
            forbidden[r2][c2] = true;
        }

        let removable_cells_left = 81 - state.sudoku.count_zeroes();
        let forbidden_count = forbidden.iter().flatten().filter(|&&x| x).count();

        if forbidden_count == removable_cells_left {
            break;
        }
    }

    state
}
