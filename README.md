# Sudoku Solver

## Installation

### Prérequis
Avant de commencer, assurez-vous d’avoir installé :
- **Rust** et **Cargo** : [https://www.rust-lang.org/tools/install](https://www.rust-lang.org/tools/install)
- **GNU Make**
- Tous les prérequis de **Tauri**: [https://tauri.app/fr/start/prerequisites](https://tauri.app/fr/start/prerequisites)

Vérifiez les installations avec :
```bash
rustc --version
cargo --version
make --version
npm --version
npm run tauri -- --version
```

### Compilation
Dans le répertoire du projet, exécutez :
```bash
make build
```
> Cette commande compile le programme et crée l’exécutable `complete_sudoku` dans `target/debug/`. Elle le déplace dans la racine du projet.

### Nettoyage
Pour supprimer les fichiers de compilation :
```bash
make clean
```

---

## Exécution
Pour lancer l’application :
```bash
make run
```

---

## Tests
Pour exécuter les tests automatiques :
```bash
make test
```
> Ces commandes lancent les tests unitaires et fonctionnels intégrés au projet.
