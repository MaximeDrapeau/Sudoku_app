use crate::lib_modules::mod_sudoku::SudokuEtat;

#[derive(Debug, PartialEq)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard,
    Expert,
}
#[allow(dead_code)]
pub fn difficulty_from_solution(state: &SudokuEtat) -> usize {
    let mut score = 0;

    let mut temp_state = state.clone();
    temp_state.verifier_sudoku();

    while temp_state.sudoku.count_zeroes() != 0 {
        //Naked Singles
        if let Some((r, c)) = find_naked_single(&temp_state) {
            temp_state.sudoku.grille[r][c] = temp_state.solution.grille[r][c];
            temp_state.verifier_sudoku();
            continue;
        }

        //Hidden Singles
        if let Some((r, c)) = find_hidden_single(&temp_state) {
            temp_state.sudoku.grille[r][c] = temp_state.solution.grille[r][c];
            temp_state.verifier_sudoku();
            continue;
        }

        let mut best: Option<(usize, usize)> = None;
        let mut entropic_min = 10;

        for i in 0..9 {
            for j in 0..9 {
                let len = temp_state.possibilitiees[i][j].len();
                if temp_state.sudoku.grille[i][j] == 0 && len > 1 && len < entropic_min {
                    entropic_min = len;
                    best = Some((i, j));
                }
            }
        }

        let Some((row, col)) = best else {
            println!("Unsolvable during difficulty analysis");
            return usize::MAX;
        };

        let n = temp_state.possibilitiees[row][col].len();
        score += (n * n).saturating_sub(1);

        temp_state.sudoku.grille[row][col] = temp_state.solution.grille[row][col];
        temp_state.verifier_sudoku();
    }

    score
}

fn find_naked_single(state: &SudokuEtat) -> Option<(usize, usize)> {
    for r in 0..9 {
        for c in 0..9 {
            if state.sudoku.grille[r][c] == 0 && state.possibilitiees[r][c].len() == 1 {
                return Some((r, c));
            }
        }
    }
    None
}

fn find_hidden_single(state: &SudokuEtat) -> Option<(usize, usize)> {
    for r in 0..9 {
        let mut counter = [0usize; 10];
        let mut cell_pos = [(0usize, 0usize); 10];

        for c in 0..9 {
            if state.sudoku.grille[r][c] == 0 {
                for &v in &state.possibilitiees[r][c] {
                    let idx = v as usize;
                    counter[idx] += 1;
                    cell_pos[idx] = (r, c);
                }
            }
        }

        for v in 1..=9 {
            if counter[v] == 1 {
                return Some(cell_pos[v]);
            }
        }
    }

    for c in 0..9 {
        let mut counter = [0usize; 10];
        let mut cell_pos = [(0usize, 0usize); 10];

        for r in 0..9 {
            if state.sudoku.grille[r][c] == 0 {
                for &v in &state.possibilitiees[r][c] {
                    let idx = v as usize;
                    counter[idx] += 1;
                    cell_pos[idx] = (r, c);
                }
            }
        }

        for v in 1..=9 {
            if counter[v] == 1 {
                return Some(cell_pos[v]);
            }
        }
    }

    for br in 0..3 {
        for bc in 0..3 {
            let mut counter = [0usize; 10];
            let mut cell_pos = [(0usize, 0usize); 10];

            for r in (br * 3)..(br * 3 + 3) {
                for c in (bc * 3)..(bc * 3 + 3) {
                    if state.sudoku.grille[r][c] == 0 {
                        for &v in &state.possibilitiees[r][c] {
                            let idx = v as usize;
                            counter[idx] += 1;
                            cell_pos[idx] = (r, c);
                        }
                    }
                }
            }

            for v in 1..=9 {
                if counter[v] == 1 {
                    return Some(cell_pos[v]);
                }
            }
        }
    }

    None
}

pub fn size_to_difficulty(size: usize) -> Difficulty {
    match size {
        0..=5 => Difficulty::Easy,
        6..=10 => Difficulty::Medium,
        11..=15 => Difficulty::Hard,
        _ => Difficulty::Expert,
    }
}

pub fn get_difficulty(state: &SudokuEtat) -> Difficulty {
    let score = difficulty_from_solution(state);
    size_to_difficulty(score)
}
