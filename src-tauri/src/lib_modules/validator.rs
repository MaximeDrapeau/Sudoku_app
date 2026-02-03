use crate::lib_modules::mod_sudoku::Sudoku;

pub fn validate(sudoku: &Sudoku) -> bool {
    for i in 0..9 {
        for j in 0..9 {
            let val = sudoku.grille[i][j];
            if val == 0 {
                continue;
            }

            for k in 0..9 {
                if k != j && sudoku.grille[i][k] == val {
                    return false;
                }
                if k != i && sudoku.grille[k][j] == val {
                    return false;
                }
            }

            let block_row = (i / 3) * 3;
            let block_col = (j / 3) * 3;
            for r in block_row..block_row + 3 {
                for c in block_col..block_col + 3 {
                    if (r != i || c != j) && sudoku.grille[r][c] == val {
                        return false;
                    }
                }
            }
        }
    }
    true
}
