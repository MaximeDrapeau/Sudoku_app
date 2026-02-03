#[derive(Default, Clone, Copy, serde::Serialize)]
pub struct Sudoku {
    pub grille: [[u8; 9]; 9],
}

#[derive(Default, Clone, serde::Serialize)]
pub struct SudokuEtat {
    pub sudoku: Sudoku,
    pub possibilitiees: [[Vec<u8>; 9]; 9],
    pub changements: Vec<BacktrackEntry>,
    pub unsolvable: bool,
    pub solution: Sudoku,
}

#[derive(Default, Clone, serde::Serialize)]
pub struct BacktrackEntry {
    pub position: [usize; 2],
    pub tried_values: Vec<u8>,
}

impl Sudoku {
    pub fn new() -> Self {
        Sudoku {
            grille: [[0; 9]; 9], // initialize all cells to 0
        }
    }

    pub fn print(&self) {
        for row in &self.grille {
            for &val in row {
                if val == 0 {
                    print!(". ");
                } else {
                    print!("{} ", val);
                }
            }
            println!();
        }
    }

    pub fn verifier_case(&self, ligne: usize, colonne: usize) -> Vec<u8> {
        // Start with all possibilities (1–9)
        let mut resultat: Vec<u8> = (1..=9).collect();

        // Remove numbers already in the same row or column
        for i in 0..9 {
            let ligne_val = self.grille[ligne][i];
            let colonne_val = self.grille[i][colonne];

            if ligne_val != 0 {
                resultat.retain(|&x| x != ligne_val);
            }
            if colonne_val != 0 {
                resultat.retain(|&x| x != colonne_val);
            }
        }

        // Remove numbers already in the same 3×3 subgrid
        let start_row = (ligne / 3) * 3;
        let start_col = (colonne / 3) * 3;

        for i in 0..3 {
            for j in 0..3 {
                let val = self.grille[start_row + i][start_col + j];
                if val != 0 {
                    resultat.retain(|&x| x != val);
                }
            }
        }

        resultat
    }

    pub fn is_solved(&self) -> bool {
        self.grille.iter().flatten().all(|&x| x != 0)
    }

    pub fn find_empty_cell(&self) -> Option<(usize, usize)> {
        for i in 0..9 {
            for j in 0..9 {
                if self.grille[i][j] == 0 {
                    return Some((i, j));
                }
            }
        }
        None
    }

    pub fn count_zeroes(&self) -> usize {
        self.grille.iter().flatten().filter(|&x| *x == 0).count()
    }
}

impl SudokuEtat {
    pub fn new(sudoku: Sudoku) -> Self {
        let possibilitiees: [[Vec<u8>; 9]; 9] = std::array::from_fn(|row| {
            std::array::from_fn(|col| {
                if sudoku.grille[row][col] == 0 {
                    sudoku.verifier_case(row, col)
                } else {
                    // Filled cell → no candidates
                    Vec::new()
                }
            })
        });

        SudokuEtat {
            sudoku,
            possibilitiees,
            changements: Vec::new(),
            unsolvable: false,
            solution: Sudoku::new(),
        }
    }

    /// Recalculate possible values for empty cells
    pub fn verifier_sudoku(&mut self) {
        for ligne in 0..9 {
            for colonne in 0..9 {
                if self.sudoku.grille[ligne][colonne] == 0 {
                    self.possibilitiees[ligne][colonne] = self.sudoku.verifier_case(ligne, colonne);
                } else {
                    self.possibilitiees[ligne][colonne].clear();
                }
            }
        }
    }

    /// Returns true if the sudoku has no valid moves left
    pub fn is_unsolvable(&self) -> bool {
        // Unsolvable if *any* empty cell has no possibilities
        for i in 0..9 {
            for j in 0..9 {
                if self.sudoku.grille[i][j] == 0 && self.possibilitiees[i][j].is_empty() {
                    return true;
                }
            }
        }
        false
    }

    /// Record a change when we assign a value
    pub fn push_change(&mut self, row: usize, col: usize, value: u8) {
        let mut tried = vec![value];
        if let Some(last) = self.changements.last() {
            // if the same cell is already at the top, merge trials
            if last.position == [row, col] {
                tried = last.tried_values.clone();
                tried.push(value);
                self.changements.pop();
            }
        }
        self.changements.push(BacktrackEntry {
            position: [row, col],
            tried_values: tried,
        });
    }

    pub fn mark_unsolvable(&mut self) {
        self.unsolvable = true;
    }

    pub fn has_valid_possibilities(&self) -> bool {
        !(0..9).any(|i| {
            (0..9).any(|j| self.sudoku.grille[i][j] == 0 && self.possibilitiees[i][j].is_empty())
        })
    }
    #[allow(dead_code)]
    pub fn print(&self) {
        // ANSI color codes
        const RESET: &str = "\x1b[0m";
        const GREEN: &str = "\x1b[32m";
        const GRAY: &str = "\x1b[90m";

        // Determine the last changed cell (if any)
        let last_change = self.changements.last().map(|c| c.position);

        println!("+-------+-------+-------+");
        for i in 0..9 {
            print!("| ");
            for j in 0..9 {
                let val = self.sudoku.grille[i][j];

                let mut cell_str = if val == 0 {
                    format!("{}.{RESET}", GRAY)
                } else {
                    format!("{}", val)
                };

                // Highlight the last changed cell in green
                if let Some(pos) = last_change
                    && pos == [i, j]
                    && val != 0
                {
                    cell_str = format!("{GREEN}{}{RESET}", val);
                }

                print!("{} ", cell_str);

                if j % 3 == 2 {
                    print!("| ");
                }
            }
            println!();
            if i % 3 == 2 {
                println!("+-------+-------+-------+");
            }
        }
    }
}

// Convert a Sudoku state to a String to insert in database
pub fn sudoku_to_string(sudoku: &Sudoku) -> String {
    let mut s = String::with_capacity(81);

    for row in 0..9 {
        for col in 0..9 {
            let val = sudoku.grille[row][col];
            let char = if val == 0 {
                '0'
            } else {
                (b'0' + val) as char // b'0' est le code ASCII de 0 (48)
            };
            s.push(char);
        }
    }
    s
}
