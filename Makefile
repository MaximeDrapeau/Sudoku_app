EXE = complete_sudoku

COLOR ?= auto # Valid COLOR options: {always, auto, never}
CARGO = cargo --color $(COLOR)

.PHONY: all clean format lint test run

all: $(EXE)

$(EXE): build
	cp -f ./src-tauri/target/release/$(EXE) .

run: $(EXE)
	./$(EXE)

clean:
	rm -rf dist node_modules .direnv src-tauri/gen $(EXE) target
	cd ./src-tauri; $(CARGO) $@

# Cargo Commands
test:
	cd ./src-tauri; $(CARGO) test

format:
	cd ./src-tauri; $(CARGO) fmt --quiet

lint:
	cd ./src-tauri; $(CARGO) clippy -- -D warnings

# Tauri Commands
.PHONY: dev build

node_modules:
	npm ci

eslint: node_modules
	npx eslint --fix

eslint-ci: node_modules
	npx eslint --max-warnings 0

# NB: the app might be completely white for a few seconds if not rebuilding from scratch
dev: node_modules
	npm run tauri dev

build: node_modules
	npm run tauri build -- --no-bundle


