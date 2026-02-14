# Text Recognition - OCR Tesseract Learning

Projet éducatif pour apprendre à paramétrer et utiliser Tesseract OCR avec Rust. Ce projet explore différentes configurations, prétraitements d'images et métriques de qualité pour optimiser la reconnaissance de texte.

## Description

**Text Recognition** est une bibliothèque et un outil en ligne de commande (CLI) qui permet de :

- Extraire du texte depuis des images en utilisant Tesseract OCR
- Tester différents modes de segmentation de page (PSM - Page Segmentation Mode)
- Appliquer des prétraitements d'images pour améliorer la qualité de l'OCR
- Mesurer la qualité des résultats avec des métriques (CER, WER, précision)
- Comparer les résultats OCR avec des textes de référence

Ce projet est principalement **éducatif** : il permet de comprendre comment fonctionnent les différents paramètres de Tesseract et leur impact sur la qualité de reconnaissance.

## Caractéristiques

### Configuration OCR

- **14 modes PSM** : Du mode automatique au mode caractère unique
- **Présets prédéfinis** : Document, screenshot, photo, ligne unique
- **Variables Tesseract** : Configuration fine via variables internes
- **Support multilingue** : Français, anglais, et autres langues supportées par Tesseract

### Prétraitement d'Images

- **Conversion en niveaux de gris** : Simplification des images couleur
- **Binarisation** : Trois méthodes (Otsu, seuil fixe, adaptative)
- **Ajustement de contraste** : Amélioration de la lisibilité
- **Débruitage** : Réduction du bruit (filtre médian)
- **Redressement** : Correction de l'inclinaison (deskew - stub actuel)

### Métriques de Qualité

- **CER** (Character Error Rate) : Taux d'erreur au niveau caractère
- **WER** (Word Error Rate) : Taux d'erreur au niveau mot
- **Distance de Levenshtein** : Nombre d'opérations d'édition
- **Précision** : Pourcentage de caractères corrects
- **Rapport détaillé** : Génération de rapports de comparaison

## Prérequis

- **Rust** : Version 1.70 ou supérieure
- **Tesseract OCR** : Version 4.0 ou supérieure
- **Données linguistiques** : Au minimum `tessdata/fra.traineddata` et `tessdata/eng.traineddata`

### Installation de Tesseract

#### Linux (Debian/Ubuntu)
```bash
sudo apt-get update
sudo apt-get install tesseract-ocr tesseract-ocr-fra tesseract-ocr-eng
sudo apt-get install libtesseract-dev libleptonica-dev
```

#### macOS
```bash
brew install tesseract tesseract-lang
```

#### Windows
Télécharger l'installeur depuis [GitHub Tesseract](https://github.com/UB-Mannheim/tesseract/wiki)

### Vérification de l'installation de Tesseract

```bash
# Vérifier la version de Tesseract
tesseract --version

# Lister les langues disponibles
tesseract --list-langs
```

Vous devriez voir au minimum `eng` et `fra` dans la liste des langues.

## Installation

### 1. Cloner le projet

```bash
git clone https://github.com/votre-username/text-recognition.git
cd text-recognition
```

### 2. Vérifier l'installation de Rust

```bash
# Vérifier la version de Rust
rustc --version
cargo --version
```

Si Rust n'est pas installé, suivez les instructions sur [rustup.rs](https://rustup.rs/).

### 3. Compiler le projet

```bash
# Compilation en mode debug (rapide, pour le développement)
cargo build

# Compilation en mode release (optimisé, pour la production)
cargo build --release
```

### 4. Lancer les tests

```bash
# Exécuter tous les tests
cargo test

# Exécuter les tests avec sortie détaillée
cargo test -- --nocapture
```

Si tous les tests passent (153 tests), l'installation est réussie ! ✅

### 5. Tester le CLI

```bash
# Afficher l'aide
cargo run -- --help

# Tester avec une image simple
cargo run -- resources/simple/img-1.png

# Tester avec des options
cargo run -- resources/simple/img-1.png --psm 3 --lang fra --metrics
```

### Installation en tant que binaire (optionnel)

Pour installer le binaire dans votre système :

```bash
# Installer dans ~/.cargo/bin/
cargo install --path .

# Utiliser directement
text-recognition resources/simple/img-1.png
```

### Utilisation en tant que bibliothèque

Pour utiliser ce projet comme bibliothèque dans un autre projet Rust, ajoutez dans votre `Cargo.toml` :

```toml
[dependencies]
text-recognition = { path = "../text-recognition" }
```

Ou, si le projet est publié sur crates.io :

```toml
[dependencies]
text-recognition = "0.1.0"
```

### Dépannage

#### Erreur "tesseract not found"

- **Linux** : Vérifiez que `libtesseract-dev` est installé
- **macOS** : Essayez `brew reinstall tesseract`
- **Windows** : Ajoutez le répertoire d'installation de Tesseract au PATH

#### Erreur "language not found"

```bash
# Installer des langues supplémentaires
# Linux
sudo apt-get install tesseract-ocr-fra tesseract-ocr-eng

# macOS
brew install tesseract-lang
```

#### Erreur de compilation Rust

```bash
# Mettre à jour Rust
rustup update

# Nettoyer et recompiler
cargo clean
cargo build
```

## Structure du Projet

```
text-recognition/
├── src/
│   ├── lib.rs              # Point d'entrée de la bibliothèque
│   ├── main.rs             # CLI
│   ├── config.rs           # Configuration OCR et présets
│   ├── ocr.rs              # Moteur OCR (wrapper Tesseract)
│   ├── preprocessing.rs    # Prétraitement d'images
│   └── metrics.rs          # Calcul de métriques
├── tests/
│   ├── integration_tests.rs    # Tests d'intégration
│   ├── psm_tests.rs            # Tests des modes PSM
│   ├── preprocessing_tests.rs  # Tests de prétraitement
│   └── metrics_tests.rs        # Tests de métriques
├── resources/
│   ├── simple/             # Images simples (texte clair)
│   ├── medium/             # Images moyennes (quelques difficultés)
│   ├── complex/            # Images complexes (qualité variable)
│   └── expected/           # Textes de référence (.txt)
├── docs/                   # Documentation approfondie
├── Cargo.toml
├── README.md
├── TODO.md                 # Suivi des tâches
└── CLAUDE.md               # Instructions pour l'agent Claude
```

## Développement

### Compilation

```bash
# Build en mode debug
cargo build

# Build en mode release (optimisé)
cargo build --release
```

### Tests

```bash
# Lancer tous les tests
cargo test

# Tests avec sortie détaillée
cargo test -- --nocapture

# Tester un module spécifique
cargo test integration_tests
```

### Qualité du Code

```bash
# Formatage
cargo fmt

# Linting
cargo clippy --all-targets --all-features -- -D warnings

# Vérification rapide
cargo check
```

### Documentation

```bash
# Générer et ouvrir la documentation
cargo doc --open
```

## Progression

- **Phase 1** : Fondations ✅ (12/12 tâches)
- **Phase 2** : Configuration Complète ✅ (10/10 tâches)
- **Phase 3** : Prétraitement ✅ (14/14 tâches)
- **Phase 4** : Métriques ✅ (11/11 tâches)
- **Phase 5** : Tests ✅ (11/11 tâches)
- **Phase 6** : Documentation 🔄 (1/10 tâches)
- **Phase 7** : Extensions (optionnel)

**Total** : 59/67 tâches complétées (88.1%)

Voir [`TODO.md`](TODO.md) pour le suivi détaillé des tâches.

## Philosophie du Projet

Ce projet suit une approche **qualité > quantité** :

- Code clair et lisible
- Documentation exhaustive
- Tests complets (153 tests unitaires et d'intégration)
- Respect des bonnes pratiques Rust
- Validation systématique (fmt, clippy, build, test)

L'objectif n'est **pas** la performance maximale, mais la **compréhension** du fonctionnement de Tesseract OCR et l'apprentissage de Rust.

## Licence

Ce projet est à usage éducatif.

## Ressources

- [Tesseract OCR](https://github.com/tesseract-ocr/tesseract)
- [Documentation Tesseract](https://tesseract-ocr.github.io/)
- [Rust Book](https://doc.rust-lang.org/book/)
- [leptess (Rust binding)](https://github.com/houqp/leptess)
- [image-rs](https://github.com/image-rs/image)
