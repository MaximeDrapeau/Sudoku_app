pub mod lib_modules;
use crate::lib_modules::base_donnee::{
    Puzzle, Sauvegarde, charger_puzzle, charger_sauvegarde, list_puzzles, list_sauvegarde, ouvrir_db
};
use crate::lib_modules::events::StepPayload;
use crate::lib_modules::import_sudoku::import_sudoku_from_sudoku;
use crate::lib_modules::mod_sudoku::{Sudoku, SudokuEtat};
use crate::lib_modules::sauvegarde_sudoku::sauvegarde;
use crate::lib_modules::sudoku_difficulty::{get_difficulty, size_to_difficulty};
use serde::Serialize;
use std::sync::Mutex;
use tauri::{Emitter, Manager, State};

use rand::rng;
use rand::seq::IndexedRandom;

#[derive(Default)]
struct AppState {
    pub etat: lib_modules::mod_sudoku::SudokuEtat,
    pub etat_initial: lib_modules::mod_sudoku::SudokuEtat,
    pub errors: [[bool; 9]; 9],
}

#[derive(Default, Serialize, Clone)]
struct EmittedState {
    grille: [[u8; 9]; 9],
    grille_initiale: [[u8; 9]; 9],
    errors: [[bool; 9]; 9],
    is_solved: bool,
    is_solvable: bool,
}

#[derive(Serialize)]
pub struct SudokuResult {
    pub grid: [[u8; 9]; 9],
    pub solved: bool,
    pub unsolvable: bool,
}

#[derive(Serialize)]
pub struct SudokuSolveSteps {
    pub steps: Vec<StepPayload>,
    pub grid: [[u8; 9]; 9],
    pub solved: bool,
    pub unsolvable: bool,
}

#[tauri::command]
fn exit(app_handle: tauri::AppHandle) {
    app_handle.exit(0);
}

#[tauri::command]
fn reset(app: tauri::AppHandle, state: State<'_, Mutex<AppState>>) {
    println!("reset sudoku");

    let mut state = state.lock().unwrap();

    state.etat = state.etat_initial.clone();

    app.emit(
        "sudoku",
        EmittedState {
            grille: state.etat.sudoku.grille,
            grille_initiale: state.etat_initial.sudoku.grille,
            errors: state.errors,
            is_solved: state.etat.sudoku.is_solved(),
            is_solvable: !state.etat.is_unsolvable(),
        },
    )
    .unwrap();
}

#[tauri::command]
fn reset_state(app: tauri::AppHandle, state: State<'_, Mutex<AppState>>) {
    let mut state = state.lock().unwrap();
    state.etat = SudokuEtat::new(Sudoku::new());
    state.etat_initial = state.etat.clone();
    state.errors = [[false; 9]; 9];

    app.emit(
        "sudoku",
        EmittedState {
            grille: state.etat.sudoku.grille,
            grille_initiale: state.etat_initial.sudoku.grille,
            errors: state.errors,
            is_solved: false,
            is_solvable: !state.etat.is_unsolvable(),
        },
    )
    .unwrap();
}

#[tauri::command]
fn set_initial_to_etat(state: State<'_, Mutex<AppState>>) {
    let mut state = state.lock().unwrap();
    state.etat_initial = state.etat.clone();
}

#[tauri::command]
fn new_sudoku(state: State<'_, Mutex<AppState>>, difficulty: usize) -> [[u8; 9]; 9] {
    println!("new sudoku");
    let diff = size_to_difficulty(difficulty);
    let mut new_state;
    loop {
        new_state = lib_modules::sudoku_generator::generate();
        if get_difficulty(&new_state) == diff {
            break;
        }
    }
    let mut state = state.lock().unwrap();
    state.etat = new_state;
    state.etat_initial = state.etat.clone();
    state.etat.sudoku.grille
}

#[tauri::command]
fn import_sudoku(
    state: State<'_, Mutex<AppState>>,
    nom: String,
    app: tauri::AppHandle,
) -> Result<i64, String> {
    let etat_initial = {
        let state = state
            .lock()
            .map_err(|_| "State lock poisoned".to_string())?;
        state.etat_initial.clone()
    };
    let conn = ouvrir_db(&app).map_err(|e| format!("Erreur ouverture base de données: {e}"))?;
    let id = import_sudoku_from_sudoku(&etat_initial, &conn, &nom).map_err(|e| e.to_string())?;
    Ok(id)
}

#[tauri::command]
fn import_new_sudoku(
    state: State<'_, Mutex<AppState>>,
    nom: String,
    app: tauri::AppHandle,
) -> Result<String, String> {
    let etat_initial = {
        let state = state
            .lock()
            .map_err(|_| "State lock poisoned".to_string())?;
        state.etat.clone()
    };
    let conn = ouvrir_db(&app).map_err(|e| format!("Erreur ouverture base de données: {e}"))?;
    import_sudoku_from_sudoku(&etat_initial, &conn, &nom).map_err(|e| e.to_string())?;
    Ok("success import".to_string())
}

#[tauri::command]
fn sauvegarde_sudoku(state: State<'_, Mutex<AppState>>, id: i64, app: tauri::AppHandle) {
    let state = state.lock().unwrap();
    match ouvrir_db(&app) {
        Ok(conn) => {
            if let Err(e) = sauvegarde(&conn, id, &state.etat.sudoku) {
                println!("{}", e);
            } else {
                println!("succes Sauvegarde");
            }
        }
        Err(e) => {
            println!("Erreur ouverture base de données: {}", e);
        }
    }
}

#[tauri::command]
fn display_puzzle_from_db(app: tauri::AppHandle) -> Result<Vec<Puzzle>, String> {
    match ouvrir_db(&app) {
        Ok(conn) => list_puzzles(&conn).map_err(|e| e.to_string()),
        Err(e) => Err(format!("Erreur ouverture base de données: {}", e)),
    }
}

#[tauri::command]
fn display_sauvegarde_from_db(app: tauri::AppHandle) -> Result<Vec<Sauvegarde>, String> {
    match ouvrir_db(&app) {
        Ok(conn) => list_sauvegarde(&conn).map_err(|e| e.to_string()),
        Err(e) => Err(format!("Erreur ouverture base de données: {}", e)),
    }
}

#[tauri::command]
fn load_sudoku_from_db(
    id: i64,
    app: tauri::AppHandle,
    state: State<'_, Mutex<AppState>>,
) -> Result<[[u8; 9]; 9], String> {
    let conn = ouvrir_db(&app).map_err(|e| format!("Erreur ouverture base de données: {e}"))?;
    let sudoku = charger_puzzle(&conn, id).map_err(|e| e.to_string())?;

    let sudoku_etat = SudokuEtat::new(sudoku);
    {
        let mut app_state = state
            .lock()
            .map_err(|_| "State lock poisoned".to_string())?;
        app_state.etat = sudoku_etat.clone();
        app_state.etat_initial = sudoku_etat;
    }
    Ok(state
        .lock()
        .map_err(|_| "State lock poisoned".to_string())?
        .etat
        .sudoku
        .grille)
}

#[tauri::command]
fn load_sauvegarde_from_db(
    date: String,
    app: tauri::AppHandle,
    state: State<'_, Mutex<AppState>>,
) -> Result<[[u8; 9]; 9], String> {
    let conn = ouvrir_db(&app).map_err(|e| format!("Erreur ouverture base de données: {e}"))?;
    let sudoku = charger_sauvegarde(&conn, &date).map_err(|e| e.to_string())?;

    let sudoku_etat = SudokuEtat::new(sudoku);
    {
        let mut app_state = state
            .lock()
            .map_err(|_| "State lock poisoned".to_string())?;
        app_state.etat = sudoku_etat;
    }
    Ok(state
        .lock()
        .map_err(|_| "State lock poisoned".to_string())?
        .etat
        .sudoku
        .grille)
}

#[tauri::command]
fn load_import_example(
    id: i64,
    app: tauri::AppHandle,
    state: State<'_, Mutex<AppState>>,
) -> Result<[[u8; 9]; 9], String> {
    let conn = ouvrir_db(&app).map_err(|e| format!("Erreur ouverture base de données: {e}"))?;
    let sudoku = charger_puzzle(&conn, id).map_err(|e| e.to_string())?;

    let sudoku_etat = SudokuEtat::new(sudoku);
    let mut state = state.lock().unwrap();
    state.etat = sudoku_etat;
    state.etat_initial = SudokuEtat::new(Sudoku::new());

    app.emit(
        "sudoku",
        EmittedState {
            grille: state.etat.sudoku.grille,
            grille_initiale: state.etat_initial.sudoku.grille,
            errors: state.errors,
            is_solved: state.etat.sudoku.is_solved(),
            is_solvable: !state.etat.is_unsolvable(),
        },
    )
    .unwrap();
    Ok(state
        .etat
        .sudoku
        .grille)
}

#[tauri::command]
fn get_sudoku(state: State<'_, Mutex<AppState>>) -> [[u8; 9]; 9] {
    let state = state.lock().unwrap();
    state.etat.sudoku.grille
}

#[tauri::command]
fn fill_from_solution(app: tauri::AppHandle, state: State<'_, Mutex<AppState>>) {
    let mut state = state.lock().unwrap();

    let mut diffs = Vec::new();
    for r in 0..9 {
        for c in 0..9 {
            if state.etat.sudoku.grille[r][c] != state.etat.solution.grille[r][c] {
                diffs.push((r, c));
            }
        }
    }

    if diffs.is_empty() {
        return;
    }

    let mut rng = rng();
    let (row, col) = *diffs.choose(&mut rng).unwrap();

    let val = state.etat.solution.grille[row][col];

    state.etat.sudoku.grille[row][col] = val;
    state.etat.verifier_sudoku();
    state.etat.push_change(row, col, val);

    app.emit(
        "sudoku",
        EmittedState {
            grille: state.etat.sudoku.grille,
            grille_initiale: state.etat_initial.sudoku.grille,
            errors: state.errors,
            is_solved: state.etat.sudoku.is_solved(),
            is_solvable: !state.etat.is_unsolvable(),
        },
    )
    .unwrap();
}

#[tauri::command]
fn force_update(app: tauri::AppHandle, state: State<'_, Mutex<AppState>>) {
    let state = state.lock().unwrap();
    app.emit(
        "sudoku",
        EmittedState {
            grille: state.etat.sudoku.grille,
            grille_initiale: state.etat_initial.sudoku.grille,
            errors: state.errors,
            is_solved: state.etat.sudoku.is_solved(),
            is_solvable: !state.etat.is_unsolvable(),
        },
    )
    .unwrap();
}

#[tauri::command]
fn update_cell(
    app: tauri::AppHandle,
    state: State<'_, Mutex<AppState>>,
    col: usize,
    row: usize,
    val: u8,
) {
    let mut state = state.lock().unwrap();

    state.etat.sudoku.grille[row][col] = 0;
    state.etat.verifier_sudoku();

    let possibilitees = &state.etat.possibilitiees[row][col];
    state.errors[row][col] = !possibilitees.contains(&val);

    state.etat.sudoku.grille[row][col] = val;
    state.etat.verifier_sudoku();

    for r in 0..9 {
        for c in 0..9 {
            if state.errors[r][c] {
                let val = state.etat.sudoku.grille[r][c];

                if val == 0 {
                    state.errors[r][c] = false;
                } else {
                    state.etat.sudoku.grille[r][c] = 0;
                    state.etat.verifier_sudoku();

                    let possibilitees = &state.etat.possibilitiees[r][c];
                    let is_valid = possibilitees.contains(&val);

                    state.etat.sudoku.grille[r][c] = val;
                    state.etat.verifier_sudoku();
                    state.errors[r][c] = !is_valid;
                }
            }
        }
    }

    app.emit(
        "sudoku",
        EmittedState {
            grille: state.etat.sudoku.grille,
            grille_initiale: state.etat_initial.sudoku.grille,
            errors: state.errors,
            is_solved: state.etat.sudoku.is_solved(),
            is_solvable: !state.etat.is_unsolvable(),
        },
    )
    .unwrap();
}

#[tauri::command]
fn resolve_sudoku(state: State<'_, Mutex<AppState>>) -> [[u8; 9]; 9] {
    state.lock().unwrap().etat.solution.grille
}

#[tauri::command]
fn solve(state: State<'_, Mutex<AppState>>) -> SudokuSolveSteps {
    println!("start solve");

    let etat_initial = {
        let guard = state.lock().unwrap();
        guard.etat_initial.clone() // AppState holds etat
    };

    let (solved_etat, steps) = lib_modules::resolver::resolve_collecting_steps(etat_initial);

    println!("Solver returned {} steps", steps.len());
    println!("Solver returned {:?} steps", solved_etat.sudoku.grille);

    SudokuSolveSteps {
        steps,
        grid: solved_etat.sudoku.grille,
        solved: solved_etat.sudoku.is_solved(),
        unsolvable: solved_etat.unsolvable,
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            exit,
            get_sudoku,
            new_sudoku,
            force_update,
            update_cell,
            resolve_sudoku,
            solve,
            fill_from_solution,
            reset,
            import_sudoku,
            import_new_sudoku,
            load_import_example,
            set_initial_to_etat,
            sauvegarde_sudoku,
            display_puzzle_from_db,
            display_sauvegarde_from_db,
            load_sudoku_from_db,
            load_sauvegarde_from_db,
            reset_state,
        ])
        .setup(|app| {
            app.manage(Mutex::new(AppState::default()));

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri");
}
