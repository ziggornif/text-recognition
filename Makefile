# Makefile - text-recognition
# Gère les différences d'environnement entre macOS et Linux pour bindgen/leptonica

# Détection de l'OS et configuration des variables d'environnement
UNAME := $(shell uname)

ifeq ($(UNAME), Darwin)
    export SDKROOT := $(shell xcrun --sdk macosx --show-sdk-path)
    ENV_INFO := macOS (SDKROOT=$(SDKROOT))
else
    export BINDGEN_EXTRA_CLANG_ARGS := --target=x86_64-unknown-linux-gnu
    ENV_INFO := Linux (BINDGEN_EXTRA_CLANG_ARGS=$(BINDGEN_EXTRA_CLANG_ARGS))
endif

.PHONY: all build release check fmt clippy test doc clean info

all: build

## Affiche la configuration détectée
info:
	@echo "OS détecté : $(ENV_INFO)"

## Compile en mode debug
build:
	cargo build

## Compile en mode release
release:
	cargo build --release

## Lance le binaire (usage : make run ARGS="image.png --lang fra")
run:
	cargo run -- $(ARGS)

## Vérifie la compilation sans produire de binaire
check:
	cargo check

## Formate le code
fmt:
	cargo fmt

## Lint avec Clippy (échoue sur les warnings)
clippy:
	cargo clippy --all-targets --all-features -- -D warnings

## Lance les tests
test:
	cargo test

## Validation complète avant commit (fmt + clippy + build + test)
validate: fmt clippy build test
	@echo "Validation complète OK"

## Génère la documentation
doc:
	cargo doc --open

## Nettoie les artefacts de build
clean:
	cargo clean

## Aide
help:
	@echo "Targets disponibles :"
	@grep -E '^## ' Makefile | sed 's/## /  /'
	@echo ""
	@echo "OS courant : $(ENV_INFO)"
