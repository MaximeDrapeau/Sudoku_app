use crate::lib_modules::mod_sudoku::Sudoku;
use rusqlite::{Connection, Result};
use serde::Serialize;
use tauri::{AppHandle, Manager};

#[derive(Serialize)]
pub struct Puzzle {
    pub id: i64,
    pub nom: String,
    pub difficulty: String,
}

#[derive(Clone, Serialize)]
pub struct Sauvegarde {
    pub id_puzzle: i64,
    pub nom: String,
    pub difficulty: String,
    pub date_sauvegarde: String,
}

// SQL for table creation
const INIT_SQL: &str = r#"
        CREATE TABLE IF NOT EXISTS puzzle (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            nom TEXT,
            difficulty TEXT,
            grille TEXT UNIQUE NOT NULL
        );

        CREATE TABLE IF NOT EXISTS sauvegarde (
            id_puzzle INTEGER NOT NULL,
            date_sauvegarde TEXT NOT NULL PRIMARY KEY,
            etat TEXT NOT NULL,

            FOREIGN KEY(id_puzzle) REFERENCES puzzle(id)

        );
    "#;

// Dev Version
pub fn ouvrir_db_dev() -> Result<Connection> {
    let base_dir = std::env::current_dir()
        .expect("Impossible d'obtenir le répertoire courant")
        .join("data");
    std::fs::create_dir_all(&base_dir).expect("Impossible de créer le dossier /data");
    let db_path = base_dir.join("sudoku.db");
    let conn = Connection::open(db_path)?;

    conn.execute_batch(INIT_SQL)?;

    Ok(conn)
}

// Tauri Version
#[allow(dead_code)]
pub fn ouvrir_db(app: &AppHandle) -> Result<Connection> {
    let dir = app
        .path()
        .app_data_dir()
        .expect("Impossible d'obtenir AppDataDir");
    std::fs::create_dir_all(&dir).expect("Impossible de créer le dossier AppDataDir");
    let db_path = dir.join("sudoku.db");
    let conn = Connection::open(db_path)?;

    conn.execute_batch(INIT_SQL)?;

    Ok(conn)
}

pub fn inserer_grille(conn: &Connection, grille: &str, difficulty: &str, nom: &str) -> Result<i64> {
    let rows = conn.execute(
        "INSERT OR IGNORE INTO puzzle (grille, difficulty, nom) VALUES (?1, ?2, ?3)",
        rusqlite::params![grille, difficulty, nom],
    )?;

    if rows == 1 {
        Ok(conn.last_insert_rowid())
    } else {
        conn.query_row("SELECT id FROM puzzle WHERE grille = ?1", [grille], |row| {
            row.get(0)
        })
    }
}

pub fn inserer_sauvegarde(
    conn: &Connection,
    id_puzzle: i64,
    etat: &str,
    date_time: &str,
) -> Result<()> {
    conn.execute(
        "INSERT OR IGNORE INTO sauvegarde (id_puzzle, date_sauvegarde, etat)
        VALUES (?1, ?2, ?3)",
        rusqlite::params![id_puzzle, date_time, etat],
    )?;
    Ok(())
}

pub fn list_puzzles(conn: &Connection) -> Result<Vec<Puzzle>> {
    let mut select_query = conn.prepare("SELECT id, nom, difficulty FROM puzzle ORDER BY ID")?;

    let iter = select_query.query_map([], |row| {
        Ok(Puzzle {
            id: row.get(0)?,
            nom: row.get(1)?,
            difficulty: row.get(2)?,
        })
    })?;

    let mut puzzles = Vec::new();
    for p in iter {
        puzzles.push(p?);
    }
    Ok(puzzles)
}

pub fn list_sauvegarde(conn: &Connection) -> Result<Vec<Sauvegarde>> {
    let mut select_query = conn.prepare(
        "SELECT s.id_puzzle, p.nom, p.difficulty, s.date_sauvegarde FROM sauvegarde s 
    JOIN puzzle p ON s.id_puzzle = p.id ORDER BY s.id_puzzle ASC, s.date_sauvegarde DESC",
    )?;

    let iter = select_query.query_map([], |row| {
        Ok(Sauvegarde {
            id_puzzle: row.get(0)?,
            nom: row.get(1)?,
            difficulty: row.get(2)?,
            date_sauvegarde: row.get(3)?,
        })
    })?;
    let mut sauvegardes = Vec::new();
    for s in iter {
        sauvegardes.push(s?);
    }
    Ok(sauvegardes)
}

pub fn charger_puzzle(conn: &Connection, id: i64) -> Result<Sudoku> {
    let mut query = conn.prepare("SELECT grille FROM puzzle WHERE id = ?1")?;
    let grille_str: String = query.query_row([id], |row| row.get(0))?;

    if grille_str.len() != 81 {
        panic!("Grille en BD invalide: longueur != 81 (ceci ne devrait pas être possible)");
    }

    let mut sudoku = Sudoku::new();

    for (index, char) in grille_str.chars().enumerate() {
        let row = index / 9;
        let col = index % 9;

        let val = match char {
            '0' => 0,
            '1'..='9' => char.to_digit(10).unwrap() as u8,
            _ => panic!(
                "Caractère invalide dans la grille en BD: {} (ceci ne devrait pas être possible",
                char
            ),
        };

        sudoku.grille[row][col] = val;
    }
    Ok(sudoku)
}

pub fn charger_sauvegarde(conn: &Connection, date: &String) -> Result<Sudoku> {
    let mut query = conn.prepare("SELECT etat FROM sauvegarde WHERE date_sauvegarde = ?1")?;
    let grille_str: String = query.query_row([date], |row| row.get(0))?;

    if grille_str.len() != 81 {
        panic!("Grille en BD invalide: longueur != 81 (ceci ne devrait pas être possible)");
    }

    let mut sudoku = Sudoku::new();

    for (index, char) in grille_str.chars().enumerate() {
        let row = index / 9;
        let col = index % 9;

        let val = match char {
            '0' => 0,
            '1'..='9' => char.to_digit(10).unwrap() as u8,
            _ => panic!(
                "Caractère invalide dans la grille en BD: {} (ceci ne devrait pas être possible",
                char
            ),
        };
        sudoku.grille[row][col] = val;
    }
    Ok(sudoku)
}
