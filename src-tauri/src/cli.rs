mod lib_modules;

use std::path::Path;
use std::process;

use crate::lib_modules::base_donnee::{charger_puzzle, charger_sauvegarde, ouvrir_db_dev};
use crate::lib_modules::display_terminal::DisplayTerminal;
use crate::lib_modules::hint_system::get_hint;
use crate::lib_modules::import_sudoku::import_sudoku_from_file;
use crate::lib_modules::jouer_sudoku::JeuSudoku;
use crate::lib_modules::menu::{ChoixFinal, Menu};

fn main() {

    let display = DisplayTerminal::new();
    let menu = Menu::new(display);
    loop {
        match menu.run() {
            ChoixFinal::JouerGrilleImportee(id) => {
                let conn = ouvrir_db_dev().expect("Impossible d'ouvrir la base de données");
                match charger_puzzle(&conn, id) {
                    Ok(sudoku) => {
                        let mut jeu = JeuSudoku::new(id, sudoku, conn, display);
                        jeu.run();
                    }
                    Err(e) => {
                        display.print_message(&format!(
                            "Erreur lors du chargement du puzzle (id={}) : {}",
                            id, e
                        ));
                        display.attendre_entree();
                    }
                }
            }
            ChoixFinal::JouerGenerer => {
                display.print_message("Lancer avec une grille générée");
                let mut state;
                loop {
                    state = lib_modules::sudoku_generator::generate();
                    if lib_modules::sudoku_difficulty::difficulty_from_solution(&state) > 15 {
                        break;
                    }
                    println!(
                        "{:?}",
                        lib_modules::sudoku_difficulty::get_difficulty(&state)
                    );
                }
                state.print();
                println!("{:?}", get_hint(&state));
            }
            ChoixFinal::SelectionerSauvegarde { id_puzzle, date } => {
                let conn = ouvrir_db_dev().expect("Impossible d'ouvrir la base de données");
                match charger_sauvegarde(&conn, &date) {
                    Ok(sudoku) => {
                        let mut jeu = JeuSudoku::new(id_puzzle, sudoku, conn, display);
                        jeu.run();
                    }
                    Err(e) => {
                        display.print_message(&format!(
                            "Erreur lors du chargement de la sauvegarde puzzle (id={}, date={}) : {}",
                            id_puzzle, date, e
                        ));
                        display.attendre_entree();
                    }
                }
            }
            ChoixFinal::Options => {
                display.print_message("Ouvrir le menu Options");
            }
            ChoixFinal::Importer(fichier, nom) => {
                let conn = ouvrir_db_dev().expect("Impossible d'ouvrir la base de données");
                let chemin = Path::new(&fichier);
                let difficulty = "easy"; // Doit utiliser la fonction qui trouve la difficulty
                if let Err(e) = import_sudoku_from_file(chemin, &conn, &nom, difficulty) {
                    display.print_message(&format!("Erreur lors de l'import : {}", e));
                } else {
                    display.print_message("Grille importée avec succès");
                }
                display.attendre_entree();
            }
            ChoixFinal::Resoudre => {
                display.print_message("Résoudre le Sudoku");
            }
            ChoixFinal::Quitter => {
                display.print_message("Fin du Programme");
                process::exit(0);
            }
        }
    }
}
