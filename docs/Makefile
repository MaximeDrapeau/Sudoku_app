# Makefile pour ConOps, diagrammes PlantUML et présentation

# === Fichiers sources ===
SRC = ConOps.md
OUT = ConOps.pdf

# Tous les fichiers .puml trouvés automatiquement
DIAGRAMS_PUML = $(wildcard *.puml)
DIAGRAMS_PDF = $(DIAGRAMS_PUML:.puml=.pdf)

# Présentation LaTeX
SRC_TEX = presentation.tex
OUT_PDF = presentation.pdf

# === Commandes ===
PANDOC = pandoc $(SRC) -o $(OUT) \
    --pdf-engine=xelatex \
    -V classoption=oneside \
    -V geometry:margin=1.5cm \
    -V fontsize=12pt \
    -V colorlinks=true \
    -V linkcolor=blue \
    -V urlcolor=blue

XELATEX = xelatex -interaction=nonstopmode

# === Cible par défaut ===
all: conops

# === Construction du ConOps (Markdown → PDF) ===
conops: $(DIAGRAMS_PDF)
	@echo "Construction de $(OUT)..."
	$(PANDOC)

# === Génération automatique des diagrammes PlantUML ===
$(DIAGRAMS_PDF): %.pdf : %.puml
	@echo "Génération PDF pour $< ..."
	plantuml -tpdf $< -o ./

# === Générer explicitement tous les diagrammes ===
diagrams: $(DIAGRAMS_PDF)

# === Compilation de la présentation Beamer ===
presentation:
	@echo "Compilation de $(SRC_TEX) avec XeLaTeX..."
	$(XELATEX) $(SRC_TEX)
	$(XELATEX) $(SRC_TEX)  # seconde passe pour les références

# === Nettoyage des fichiers générés ===
clean:
	@echo "Suppression de tous les fichiers générés..."
	rm -f $(OUT) $(DIAGRAMS_PDF) $(OUT_PDF) *.aux *.log *.toc *.out *.nav *.snm
