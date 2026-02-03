use crate::lib_modules::display_terminal::DisplayTerminal;
use crate::lib_modules::mod_sudoku::{Sudoku, SudokuEtat};
use crate::lib_modules::sauvegarde_sudoku::sauvegarde;

use rusqlite::Connection;

use std::io::{self, Write};

#[allow(dead_code)]
pub struct JeuSudoku {
    pub id: i64,
    pub etat: SudokuEtat,
    pub conn: Connection,
    pub display: DisplayTerminal,
}

impl JeuSudoku {
    #[allow(dead_code)]
    pub fn new(id: i64, grille_sudoku: Sudoku, conn: Connection, display: DisplayTerminal) -> Self {
        let etat = SudokuEtat::new(grille_sudoku);
        Self {
            id,
            etat,
            conn,
            display,
        }
    }
    #[allow(dead_code)]
    pub fn run(&mut self) {
        loop {
            self.display.clear();
            self.display.print_message("===== SUDOKU =====\n");
            self.etat.sudoku.print();

            if self.est_termine() {
                self.display.print_message("Bravo, sudoku complété !");
                break;
            }
            self.display.print_jouer_action();
            let choix = self.lire_choix();
            let parsed: Result<u8, _> = choix.trim().parse();
            if let Ok(num) = parsed {
                match num {
                    1 => self.entrer_valeur(),
                    2 => {
                        let (ligne, colonne) = self.get_coordonnee();
                        let possibilitees = &self.etat.possibilitiees[ligne][colonne];
                        self.display.print_message(&format!(
                            "Voici les possibilitées pour la case [{}][{}]",
                            ligne + 1,
                            colonne + 1
                        ));
                        for item in possibilitees {
                            self.display.print_message(&format!("{}", item));
                        }
                    }
                    3 => {
                        if let Err(e) = sauvegarde(&self.conn, self.id, &self.etat.sudoku) {
                            self.display
                                .print_message(&format!("Erreur lors de la sauvegarde : {}", e));
                        } else {
                            self.display.print_message("Grille sauvegardée avec succès");
                        }
                        self.display.attendre_entree();
                    }
                    4 => break,
                    _ => {
                        self.display.print_message("Le choix doit être 1, 2 ou 3");
                        self.display.attendre_entree();
                    }
                }
            } else {
                self.display.print_message("Le choix doit être un nombre");
                self.display.attendre_entree();
            }
        }
    }

    fn est_termine(&self) -> bool {
        for ligne in 0..9 {
            for colonne in 0..9 {
                if self.etat.sudoku.grille[ligne][colonne] == 0 {
                    return false;
                }
            }
        }
        true
    }

    fn entrer_valeur(&mut self) {
        let (ligne, colonne) = self.get_coordonnee();
        self.display.print_message(&format!(
            "Veuillez entrer la valeur à mettre dans la case [{}][{}] (1-9)",
            ligne + 1,
            colonne + 1
        ));
        let valeur = self.get_valeur();
        let possibilitees = &self.etat.possibilitiees[ligne][colonne];
        if possibilitees.contains(&valeur) {
            self.etat.sudoku.grille[ligne][colonne] = valeur;
            self.etat.verifier_sudoku();
        } else {
            self.display
                .print_message("Cette valeur n'est pas possible pour cette case.");
            self.display.attendre_entree();
        }
    }

    fn lire_choix(&self) -> String {
        self.display.print_pre_ecriture();
        let _ = io::stdout().flush();
        let mut lecture = String::new();
        if io::stdin().read_line(&mut lecture).is_ok() {
            return lecture.trim().to_string();
        }
        String::new()
    }

    fn get_coordonnee(&self) -> (usize, usize) {
        loop {
            self.display
                .print_message("Veuillez entrer la ligne (1-9): ");
            let ligne = self.lire_choix();
            self.display
                .print_message("Veuillez entrer la colonne (1-9): ");
            let colonne = self.lire_choix();
            let ligne_nbr: Result<u8, _> = ligne.trim().parse();
            let colonne_nbr: Result<u8, _> = colonne.trim().parse();
            // Verifie si ligne_nbr et colonne_nbr sont des u8
            if let (Ok(ligne), Ok(colonne)) = (ligne_nbr, colonne_nbr) {
                //Verifie si ligne et colonne sont entre 1 et 9
                if !(1..=9).contains(&ligne) || !(1..=9).contains(&colonne) {
                    self.display
                        .print_message("Les lignes et les colonnes doivent être entre 1 et 9");
                    self.display.attendre_entree();
                    continue;
                }
                let ligne = (ligne - 1) as usize;
                let colonne = (colonne - 1) as usize;
                if self.etat.sudoku.grille[ligne][colonne] != 0 {
                    self.display
                        .print_message("Cette case possède déjà une valeur, veuillez réessayer");
                    self.display.attendre_entree();
                    continue;
                }
                return (ligne, colonne);
            } else {
                self.display
                    .print_message("Les lignes et les colonnes doivent être des nombres valides");
                self.display.attendre_entree();
            }
        }
    }

    fn get_valeur(&self) -> u8 {
        loop {
            let valeur = self.lire_choix();
            let valeur_parse: Result<u8, _> = valeur.trim().parse();
            if let Ok(valeur) = valeur_parse {
                if (1..=9).contains(&valeur) {
                    return valeur;
                } else {
                    self.display
                        .print_message("La valeur doit se trouver entre 1 et 9");
                    self.display.attendre_entree();
                }
            } else {
                self.display
                    .print_message("La valeur doit être un nom valide");
                self.display.attendre_entree();
            }
        }
    }
}
