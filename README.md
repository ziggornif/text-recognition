# Text Recognition - OCR Tesseract Learning

Projet personnel d'apprentissage pour maîtriser Tesseract OCR avec Rust.

## Objectif

Apprendre à paramétrer et utiliser Tesseract OCR en testant différentes configurations, prétraitements d'images et métriques de qualité.

## Stack

- **Rust** - Langage principal
- **Tesseract OCR** - Moteur de reconnaissance de texte
- **image** - Manipulation d'images

## Installation

### Prérequis

```bash
# Installer Tesseract (système)
sudo apt install tesseract-ocr libtesseract-dev libleptonica-dev

# Vérifier l'installation
tesseract --version
```

### Build

```bash
cargo build
cargo test
```

## Utilisation

```bash
# Exécuter l'OCR sur une image
cargo run -- <chemin_image>
```

## Structure

```
src/
├── lib.rs            # Exports publics
├── main.rs           # CLI
├── ocr.rs            # Wrapper Tesseract
├── config.rs         # Configuration OCR
├── preprocessing.rs  # Prétraitement d'images
└── metrics.rs        # Métriques de qualité
```

## Développement

```bash
# Formatage
cargo fmt

# Linting
cargo clippy --all-targets --all-features -- -D warnings

# Build
cargo build

# Tests
cargo test
```

## Progression

Voir [`TODO.md`](TODO.md) pour le suivi détaillé des tâches.

## Docs

- [Guide CLAUDE.md](CLAUDE.md) - Instructions pour l'agent Claude
- [TODO.md](TODO.md) - Liste des tâches par phase
