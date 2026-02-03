#[derive(Clone, Copy, Default)]
pub struct DisplayTerminal;

impl DisplayTerminal {
    #[allow(dead_code)]
    pub fn new() -> Self {
        Self
    }

    #[allow(dead_code)]
    pub fn clear(&self) {
        use std::io::{self, Write};
        print!("\x1B[2J\x1B[H");
        let _ = io::stdout().flush();
    }

    // Fonctions de print pour le menu
    #[allow(dead_code)]
    pub fn print_main_menu(&self) {
        const MENU_TITRE: &str = "===== Menu Principal =====";
        const MENU_ITEM: [&str; 5] = [
            "Jouer",
            "Options",
            "Importer une Grille",
            "Résoundre une Grille",
            "Quitter",
        ];

        println!("{MENU_TITRE}\n");
        for (i, item) in MENU_ITEM.iter().enumerate() {
            println!("{}.   {}", i + 1, item);
        }
    }
    #[allow(dead_code)]
    pub fn print_principal_option1(&self) {
        const MENU_JOUER_TITRE: &str = "===== Options de Grilles =====";
        const MENU_JOUER_ITEM: [&str; 3] = [
            "Grille Importée",
            "Générer une Grille",
            "Continuer Une Partie",
        ];
        println!("{MENU_JOUER_TITRE}\n");
        for (i, item) in MENU_JOUER_ITEM.iter().enumerate() {
            println!("{}.   {}", i + 1, item);
        }
    }

    #[allow(dead_code)]
    pub fn print_principal_option3(&self) {
        println!("Veuillez entrer le path du fichier contenant la grille:  ");
    }

    // Fonctions de print pour Jouer
    #[allow(dead_code)]
    pub fn print_jouer_action(&self) {
        const JOUER_ACTION_TITRE: &str = "===== Actions =====";
        const JOUER_ACTION_ITEM: [&str; 4] = [
            "Entrer une valeur",
            "Voir les possibilités d'une case",
            "Suvegarder la partie",
            "Quitter",
        ];
        println!("{JOUER_ACTION_TITRE}\n");
        for (i, item) in JOUER_ACTION_ITEM.iter().enumerate() {
            println!("{}.   {}", i + 1, item);
        }
    }

    // Fonctions de print générales
    #[allow(dead_code)]
    pub fn print_pre_ecriture(&self) {
        print!("> ");
    }
    #[allow(dead_code)]
    pub fn print_message(&self, message: &str) {
        println!("{message}");
    }
    #[allow(dead_code)]
    pub fn attendre_entree(&self) {
        use std::io::{self, Write};
        print!("(ENTER pour continuer) ");
        let _ = io::stdout().flush();
        let _ = io::stdin().read_line(&mut String::new());
    }
}
