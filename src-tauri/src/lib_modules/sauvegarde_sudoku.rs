use crate::lib_modules::base_donnee::inserer_sauvegarde;
use crate::lib_modules::mod_sudoku::{Sudoku, sudoku_to_string};
use chrono::{Local, SecondsFormat::Millis};

use rusqlite::Connection;

pub fn sauvegarde(conn: &Connection, id: i64, sudoku: &Sudoku) -> Result<(), String> {
    let etat_str = sudoku_to_string(sudoku);
    let date_time = Local::now().to_rfc3339_opts(Millis, false);
    inserer_sauvegarde(conn, id, &etat_str, &date_time).map_err(|e| e.to_string())?;
    Ok(())
}
