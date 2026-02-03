use crate::lib_modules::base_donnee::{Sauvegarde, list_puzzles, list_sauvegarde, ouvrir_db_dev};
use crate::lib_modules::display_terminal::DisplayTerminal;
use rusqlite::Connection;

use std::io::{self, Write};

#[allow(dead_code)]
pub enum ChoixFinal {
    JouerGrilleImportee(i64),
    JouerGenerer,
    SelectionerSauvegarde { id_puzzle: i64, date: String },
    Options,
    Importer(String, String),
    Resoudre,
    Quitter,
}
#[allow(dead_code)]
pub enum ChoixMenuPrincipal {
    Jouer,
    Options,
    Importer,
    Resoudre,
    Quitter,
}
#[allow(dead_code)]
pub enum ChoixJouer {
    GrilleImportee,
    Generer,
    SelectSauvegarde,
}

pub struct Menu {
    display: DisplayTerminal,
}

impl Menu {
    #[allow(dead_code)]
    pub fn new(display: DisplayTerminal) -> Self {
        Self { display }
    }
    #[allow(dead_code)]
    pub fn run(&self) -> ChoixFinal {
        match self.menu_principal() {
            ChoixMenuPrincipal::Jouer => self.handle_jouer(),
            ChoixMenuPrincipal::Options => ChoixFinal::Options,
            ChoixMenuPrincipal::Importer => {
                let (fichier, nom) = self.menu_importer();
                ChoixFinal::Importer(fichier, nom)
            }
            ChoixMenuPrincipal::Resoudre => ChoixFinal::Resoudre,
            ChoixMenuPrincipal::Quitter => ChoixFinal::Quitter,
        }
    }
    fn menu_principal(&self) -> ChoixMenuPrincipal {
        loop {
            self.display.clear();
            self.display.print_main_menu();
            self.display.print_pre_ecriture();
            let choix = self.lire_choix();
            let parsed: Result<u8, _> = choix.trim().parse();
            if let Ok(num) = parsed {
                match num {
                    1 => return ChoixMenuPrincipal::Jouer,
                    2 => return ChoixMenuPrincipal::Options,
                    3 => return ChoixMenuPrincipal::Importer,
                    4 => return ChoixMenuPrincipal::Resoudre,
                    5 => return ChoixMenuPrincipal::Quitter,
                    _ => {
                        self.display
                            .print_message("Le choix n'est pas valide (doit être entre 1 et 5)");
                        self.display.attendre_entree();
                    }
                }
            } else {
                self.display.print_message("Le choix doit être un nombre");
                self.display.attendre_entree();
            }
        }
    }

    fn menu_jouer(&self) -> ChoixJouer {
        loop {
            self.display.clear();
            self.display.print_principal_option1();
            self.display.print_pre_ecriture();
            let choix = self.lire_choix();
            let parsed: Result<u8, _> = choix.trim().parse();
            if let Ok(num) = parsed {
                match num {
                    1 => return ChoixJouer::GrilleImportee,
                    2 => return ChoixJouer::Generer,
                    3 => return ChoixJouer::SelectSauvegarde,
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

    fn menu_importer(&self) -> (String, String) {
        self.display.clear();
        self.display
            .print_message("=== Importer une grille ===\n\n");

        // path of file
        self.display
            .print_message("Entrez le chemin du fichier contenant la grille :");
        self.display.print_pre_ecriture();
        let fichier = self.lire_choix();

        // Nom du puzzle
        self.display
            .print_message("Entrez un nom pour ce puzzle : ");
        self.display.print_pre_ecriture();
        let nom = self.lire_choix();

        (fichier, nom)
    }

    fn menu_choisir_puzzle_dev(&self, conn: &Connection) -> Option<i64> {
        let puzzles = match list_puzzles(conn) {
            Ok(p) => p,
            Err(e) => {
                self.display
                    .print_message(&format!("Erreur BD lors de la lecture des puzzles : {}", e));
                return None;
            }
        };

        if puzzles.is_empty() {
            self.display
                .print_message("Aucun puzzle disponible dans la BD");
            return None;
        }

        self.display.print_message("===== Choisir Puzzle =====\n\n");
        for (i, p) in puzzles.iter().enumerate() {
            self.display
                .print_message(&format!("{}, {}, {}", i + 1, p.difficulty, p.nom));
        }
        loop {
            self.display
                .print_message("Entrez le numéro du puzzle à jouer : ");
            self.display.print_pre_ecriture();
            let choix = self.lire_choix();
            let parsed: Result<usize, _> = choix.trim().parse();
            if let Ok(num) = parsed {
                if num == 0 || num > puzzles.len() {
                    self.display.print_message("Numéro invalide, Réessayez");
                    continue;
                }
                let id = puzzles[num - 1].id;
                return Some(id);
            } else {
                self.display.print_message("Veuillez entrer un nombre");
            }
        }
    }

    fn menu_choisir_sauvegarde(&self, conn: &Connection) -> Option<Sauvegarde> {
        let sauvegardes = match list_sauvegarde(conn) {
            Ok(s) => s,
            Err(e) => {
                self.display.print_message(&format!(
                    "Erreur BD lors de la lecture des sauvegardes : {}",
                    e
                ));
                return None;
            }
        };
        if sauvegardes.is_empty() {
            self.display
                .print_message("Aucune sauvegardes disponible dans la BD");
            return None;
        }
        self.display
            .print_message("===== Choisir Sauvegarde =====\n\n");
        for (i, s) in sauvegardes.iter().enumerate() {
            self.display.print_message(&format!(
                "{}, {}, {}, {}",
                i + 1,
                s.difficulty,
                s.nom,
                s.date_sauvegarde
            ));
        }
        loop {
            self.display
                .print_message("Entrez le numéro de la sauvegarde : ");
            self.display.print_pre_ecriture();
            let choix = self.lire_choix();
            let parsed: Result<usize, _> = choix.trim().parse();
            if let Ok(num) = parsed {
                if num == 0 || num > sauvegardes.len() {
                    self.display.print_message("Numéro invalide, Réessayez");
                    continue;
                }
                return Some(sauvegardes[num - 1].clone());
            } else {
                self.display.print_message("Veuillez entrer un nombre");
            }
        }
    }

    fn handle_jouer(&self) -> ChoixFinal {
        match self.menu_jouer() {
            ChoixJouer::GrilleImportee => self.handle_grille_importee(),
            ChoixJouer::Generer => ChoixFinal::JouerGenerer,
            ChoixJouer::SelectSauvegarde => self.handle_sauvegarde(),
        }
    }

    fn handle_grille_importee(&self) -> ChoixFinal {
        let conn = ouvrir_db_dev().expect("Impossible d'ouvrir la BD");
        match self.menu_choisir_puzzle_dev(&conn) {
            Some(id) => {
                self.display
                    .print_message(&format!("Puzzle choisi ID = {}", id));
                self.display.attendre_entree();
                ChoixFinal::JouerGrilleImportee(id)
            }
            None => {
                self.display.print_message("Aucun Puzzle sélectionné.\n");
                self.display.print_message("Retour au menu Jouer...\n");
                self.display.attendre_entree();
                self.handle_jouer()
            }
        }
    }

    fn handle_sauvegarde(&self) -> ChoixFinal {
        let conn = ouvrir_db_dev().expect("Impossible d'ouvrir la BD");
        match self.menu_choisir_sauvegarde(&conn) {
            Some(s) => {
                self.display.print_message(&format!(
                    "Sauvegarde choisi nom = {}, difficulté = {}, date = {}",
                    s.nom, s.difficulty, s.date_sauvegarde
                ));
                self.display.attendre_entree();
                ChoixFinal::SelectionerSauvegarde {
                    id_puzzle: s.id_puzzle,
                    date: s.date_sauvegarde.clone(),
                }
            }
            None => {
                self.display
                    .print_message("Aucune Sauvegarde sélectionné.\n");
                self.display.print_message("Retour au menu Jouer...\n");
                self.display.attendre_entree();
                self.handle_jouer()
            }
        }
    }

    fn lire_choix(&self) -> String {
        let _ = io::stdout().flush();
        let mut lecture = String::new();
        if io::stdin().read_line(&mut lecture).is_ok() {
            return lecture.trim().to_string();
        }
        String::new()
    }
}
