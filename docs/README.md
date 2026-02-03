# ConOps - Guide de génération du PDF

Ce document explique comment générer le PDF `ConOps.pdf` à partir du Markdown `ConOps.md` et des diagrammes PlantUML.

---

# 1. Prérequis

Assurez-vous que les logiciels suivants sont installés sur votre système.

## 1.1 Pandoc

- Site officiel : [https://pandoc.org/installing.html](https://pandoc.org/installing.html)  
- Installation sur Ubuntu/Debian :

```bash
sudo apt update
sudo apt install pandoc
```

## 1.2 XeLaTeX (pour la génération PDF via Pandoc)

- Installation sur Ubuntu/Debian :

```bash
sudo apt install texlive-xetex texlive-fonts-recommended texlive-latex-recommended
```

## 1.3 PlantUML (pour générer les diagrammes PDF)

- Installation sur Ubuntu/Debian :

```bash
sudo apt install plantuml graphviz
```

---

# 2. Organisation des fichiers

- `ConOps.md` : Document Markdown principal.  
- `diagramme1.puml`, `diagramme2.puml`, `diagramme3.puml` : Diagrammes PlantUML.  
- `Makefile` : Fichier pour automatiser la génération du PDF et des diagrammes.

---

# 3. Génération du PDF

## 3.1 Générer uniquement les diagrammes PlantUML

```bash
make diagrams
```

- Crée les fichiers PDF correspondants aux diagrammes (`diagramme1.pdf`, `diagramme2.pdf`, etc.).

## 3.2 Générer le PDF ConOps

```bash
make conops
```

- Vérifie si les diagrammes PDF existent.  
- Génère automatiquement les diagrammes manquants.  
- Compile ensuite `ConOps.md` en `ConOps.pdf` via Pandoc et XeLaTeX.

## 3.3 Générer diagrammes + ConOps en une seule commande

Si votre Makefile définit la cible `build` :

```bash
make build
```

- Génère tous les diagrammes et le PDF ConOps en une seule étape.

## 3.4 Nettoyer les fichiers PDF générés

```bash
make clean
```

- Supprime tous les fichiers PDF générés (`diagramme*.pdf` et `ConOps.pdf`).

---

# 4. Notes et recommandations

- Tous les fichiers `.puml` doivent se trouver dans le même dossier que le Makefile.  
- Les liens dans `ConOps.md` seront cliquables et colorés dans le PDF final.  
- Vous pouvez ajuster la mise en page du PDF en modifiant les options `geometry` et `fontsize` dans le Makefile.
