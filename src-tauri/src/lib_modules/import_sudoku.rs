use rusqlite::Connection;
use std::fs;
use std::path::Path;

use crate::lib_modules::base_donnee::inserer_grille;
use crate::lib_modules::mod_sudoku::{SudokuEtat, Sudoku, sudoku_to_string};
use crate::lib_modules::validator;
use crate::lib_modules::sudoku_difficulty::{Difficulty};

pub fn import_sudoku_from_file(
    fichhier: &Path,
    conn: &Connection,
    nom: &str,
    difficulty: &str,
) -> Result<(), String> {
    let sudoku = create_sudoku_from_file(fichhier)?;
    let grille_str = sudoku_to_string(&sudoku);

    inserer_grille(conn, &grille_str, difficulty, nom).map_err(|e| e.to_string())?;

    Ok(())
}

#[allow(dead_code)]
pub fn import_sudoku_from_sudoku(state: &SudokuEtat, conn: &Connection, nom: &str)-> Result<i64, String> {
    let difficulty = format!("{:?}", Difficulty::Easy);
    let grille_str = sudoku_to_string(&state.sudoku);
    let id = inserer_grille(conn, &grille_str, &difficulty, nom).map_err(|e| e.to_string())?;
    Ok(id)
}

pub fn create_sudoku_from_file(fichier: &Path) -> Result<Sudoku, String> {
    let lecture_fichier =
        fs::read_to_string(fichier).map_err(|e| format!("impossible de lire le fichier: {}", e))?;

    let lecture_nettoyee: String = lecture_fichier
        .chars()
        .filter(|x| !x.is_whitespace())
        .collect();
    if lecture_nettoyee.len() != 81 {
        return Err(format!(
            "La grille doit contenir exactement 81 caractères, nombre de caractères présent dans le fichier: {}",
            lecture_nettoyee.len()
        ));
    }

    let mut sudoku = Sudoku::new();

    for (index, char) in lecture_nettoyee.chars().enumerate() {
        let row = index / 9;
        let col = index % 9;

        let val = match char {
            '.' | '0' => 0,
            '1'..='9' => char.to_digit(10).unwrap() as u8,
            _ => {
                return Err(format!(
                    "Caractère invalide ({}) à la position {} (ligne: {}, colonne: {}). Utiliser 0-9 ou '.' pour 0",
                    char, index, row, col
                ));
            }
        };

        sudoku.grille[row][col] = val;
    }
    if !validator::validate(&sudoku) {
        return Err("La grille n'est pas valide selon les règles du Sudoku".to_string());
    }
    Ok(sudoku)
}
