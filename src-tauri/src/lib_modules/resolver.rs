use crate::lib_modules::mod_sudoku::{Sudoku, SudokuEtat};
use crate::lib_modules::events::{StepPayload};
use rand::prelude::IndexedRandom;
use rand::rng;

pub fn resolve_sudoku(mut state: SudokuEtat) -> SudokuEtat {
    while !state.sudoku.is_solved() && !state.is_unsolvable() {
        state = resolving_step(state);

        if !state.has_valid_possibilities() {
            state = backtrack(state);
            state.verifier_sudoku();
            continue;
        }

        state.verifier_sudoku();
    }

    state
}
#[allow(dead_code)]
pub fn resolve_collecting_steps(mut state: SudokuEtat) -> (SudokuEtat, Vec<StepPayload>) {
    let mut steps = Vec::new();
    state.verifier_sudoku();

    loop {
        if state.sudoku.is_solved() || state.is_unsolvable() {
            return (state, steps);
        }

        let before = state.changements.clone();
        state = resolving_step(state);
        let after = state.changements.clone();

        if let Some(new_last) = after.last() {
            let changed = before.last().map(|b| b.position != new_last.position
                                        || b.tried_values != new_last.tried_values)
                                        .unwrap_or(true);

            if changed {
                let [r, c] = new_last.position;
                let v = state.sudoku.grille[r][c];

                steps.push(StepPayload {
                    row: r,
                    col: c,
                    value: v,
                    grid: state.sudoku.grille,
                });
            }
        }

        state.verifier_sudoku();

        if state.has_valid_possibilities() {
            continue; // nothing to backtrack
        }

        let before_bt = state.changements.clone();
        state = backtrack(state);
        let after_bt = state.changements.clone();

        let mut prefix_len = 0;
        while prefix_len < before_bt.len()
            && prefix_len < after_bt.len()
            && before_bt[prefix_len].position == after_bt[prefix_len].position
        {
            prefix_len += 1;
        }

        for popped in before_bt.iter().skip(prefix_len) {
            let [r, c] = popped.position;
            steps.push(StepPayload {
                row: r,
                col: c,
                value: 0,
                grid: state.sudoku.grille,
            });
        }

        if let Some(last) = after_bt.last() {
            let [r, c] = last.position;
            let v = state.sudoku.grille[r][c];

            steps.push(StepPayload {
                row: r,
                col: c,
                value: v,
                grid: state.sudoku.grille,
            });
        }

        state.verifier_sudoku();
    }
}


fn resolving_step(mut state: SudokuEtat) -> SudokuEtat {
    let mut entropic_position: Option<[usize; 2]> = None;
    let mut entropic_min = 10;

    for i in 0..9 {
        for j in 0..9 {
            let len = state.possibilitiees[i][j].len();
            if state.sudoku.grille[i][j] == 0 && len > 0 && len < entropic_min {
                entropic_min = len;
                entropic_position = Some([i, j]);
            }
        }
    }

    let Some([row, col]) = entropic_position else {
        return state;
    };

    let mut rng = rng();

    if let Some(&value) = state.possibilitiees[row][col].choose(&mut rng) {
        // remove that chosen value from the vector
        state.possibilitiees[row][col].retain(|&x| x != value);

        state.sudoku.grille[row][col] = value;
        state.push_change(row, col, value);
        state.verifier_sudoku();


        return state;
    }

    state
}

fn backtrack(mut state: SudokuEtat) -> SudokuEtat {
    while let Some(mut entry) = state.changements.pop() {
        let [row, col] = entry.position;

        state.sudoku.grille[row][col] = 0;

        let all_poss = state.sudoku.verifier_case(row, col);

        let remaining: Vec<u8> = all_poss
            .into_iter()
            .filter(|v| !entry.tried_values.contains(v))
            .collect();

        let mut rng = rng();

        if let Some(&next_val) = remaining.choose(&mut rng) {
            state.sudoku.grille[row][col] = next_val;

            entry.tried_values.push(next_val);
            state.changements.push(entry);

            state.verifier_sudoku();

            return state;
        }
    }

    state.mark_unsolvable();
    state
}

pub fn count_solutions(sudoku: &mut Sudoku, count: &mut usize) {
    // Stop early — no need to find more
    if *count >= 2 {
        return;
    }

    let Some((row, col)) = sudoku.find_empty_cell() else {
        *count += 1; // Found a complete solution
        return;
    };

    // Get all allowed values for this empty cell
    let possible = sudoku.verifier_case(row, col);

    for val in possible {
        sudoku.grille[row][col] = val;

        count_solutions(sudoku, count);

        sudoku.grille[row][col] = 0;

        if *count >= 2 {
            return; // No uniqueness
        }
    }
}

pub fn has_unique_solution(mut sudoku: Sudoku) -> bool {
    let mut count = 0;
    count_solutions(&mut sudoku, &mut count);
    count == 1
}
