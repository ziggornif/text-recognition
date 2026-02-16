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
- **Redressement (deskew)** : Correction des inclinaisons légères (-20° à +20°) par projection horizontale
- **Correction d'orientation** : Détection et correction des rotations 90°/180°/270° via Tesseract PSM 0 (`--auto-rotate`)

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

## Utilisation

### Utilisation de base

L'utilisation la plus simple consiste à extraire le texte d'une image :

```bash
# Extraire le texte d'une image
cargo run -- resources/simple/img-1.png

# Ou avec le binaire installé
text-recognition resources/simple/img-1.png
```

### Afficher l'aide

```bash
cargo run -- --help
```

### Exemples d'utilisation CLI

#### 1. Extraction simple

```bash
# Extraction avec les paramètres par défaut (langue: français, PSM: 3)
cargo run -- resources/simple/img-1.png
```

#### 2. Changer la langue

```bash
# Utiliser l'anglais
cargo run -- resources/simple/img-1.png --language eng

# Combiner plusieurs langues
cargo run -- resources/simple/img-1.png --language eng+fra
```

#### 3. Tester différents modes PSM

```bash
# Mode ligne unique (PSM 7)
cargo run -- resources/simple/img-1.png --psm 7

# Mode colonne unique (PSM 4)
cargo run -- resources/simple/img-1.png --psm 4

# Mode texte épars (PSM 11)
cargo run -- resources/simple/img-1.png --psm 11
```

#### 4. Appliquer du prétraitement

```bash
# Prétraitement complet (grayscale + binarization + denoise)
cargo run -- resources/medium/img-2.png --preprocess

# Prétraitement personnalisé
cargo run -- resources/medium/img-2.png --grayscale --binarize --denoise

# Binarisation avec méthode spécifique
cargo run -- resources/medium/img-2.png --grayscale --binarize --binarize-method otsu

# Binarisation avec seuil fixe
cargo run -- resources/medium/img-2.png --grayscale --binarize --binarize-method fixed:128

# Ajuster le contraste (1.5x)
cargo run -- resources/medium/img-2.png --contrast 1.5
```

#### 5. Mesurer la qualité avec des métriques

```bash
# Comparer avec un texte de référence
cargo run -- resources/simple/img-1.png --expected resources/expected/img-1.txt

# Afficher un rapport détaillé
cargo run -- resources/simple/img-1.png --expected resources/expected/img-1.txt --metrics
```

Le rapport affichera :
- **CER** (Character Error Rate) : Taux d'erreur au niveau caractère
- **WER** (Word Error Rate) : Taux d'erreur au niveau mot
- **Précision** : Pourcentage de caractères corrects
- **Distance de Levenshtein** : Nombre d'opérations d'édition nécessaires

#### 6. Tester tous les modes PSM

```bash
# Tester les 14 modes PSM sur une image
cargo run -- resources/simple/img-1.png --test-all-psm

# Tester tous les modes PSM avec métriques
cargo run -- resources/simple/img-1.png --test-all-psm --expected resources/expected/img-1.txt
```

Cette option est très utile pour déterminer quel mode PSM donne les meilleurs résultats pour un type d'image spécifique.

#### 7. Corriger l'orientation automatiquement

```bash
# Détecter et corriger l'orientation (image à l'envers, pivotée de 90°/270°)
cargo run -- resources/medium/img-6.png --auto-rotate

# Combiner correction d'orientation et prétraitement
cargo run -- resources/medium/img-6.png --auto-rotate --preprocess --grayscale --binarize
```

#### 8. Combiner plusieurs options

```bash
# Prétraitement + langue spécifique + métriques
cargo run -- resources/medium/img-2.png \
  --language fra \
  --psm 3 \
  --preprocess \
  --expected resources/expected/img-2.txt \
  --metrics

# Test complet avec tous les paramètres
cargo run -- resources/complex/img-7.png \
  --language fra \
  --psm 6 \
  --grayscale \
  --binarize \
  --binarize-method adaptive \
  --denoise \
  --contrast 1.3 \
  --expected resources/expected/img-7.txt \
  --metrics
```

#### 9. Exemples par type d'image

##### Document texte classique
```bash
cargo run -- mon_document.png --psm 3 --language fra
```

##### Screenshot d'interface
```bash
cargo run -- screenshot.png --psm 11 --preprocess
```

##### Photo de document
```bash
cargo run -- photo_doc.jpg \
  --psm 3 \
  --grayscale \
  --binarize \
  --binarize-method adaptive \
  --contrast 1.5
```

##### Ligne de texte unique
```bash
cargo run -- ligne_texte.png --psm 7
```

##### Mot isolé
```bash
cargo run -- mot.png --psm 8
```

### Exemples de sortie

#### Extraction simple
```
2 ABREVIATIONS ET SYMBOLES

Dans le but de faciliter la compréhension de la notice...
```

#### Avec métriques
```
═══════════════════════════════════════════════════════════
                   OCR COMPARISON REPORT
═══════════════════════════════════════════════════════════

METRICS:
--------
Character Error Rate (CER): 0.10%
Word Error Rate (WER):      0.14%
Levenshtein Distance:       1
Accuracy:                   99.90%

STATISTICS:
-----------
Reference: 719 characters, 118 words
OCR:       719 characters, 118 words

SUMMARY:
--------
Quality: Excellent (< 2% error)
Match:   Not exact
```

#### Test de tous les modes PSM
```
Testing all PSM modes on: resources/simple/img-1.png

PSM 0 (OSD Only):
[résultat du mode 0]

PSM 1 (Auto with OSD):
[résultat du mode 1]

...

PSM 13 (Raw line):
[résultat du mode 13]
```

### Utilisation de la bibliothèque

Vous pouvez également utiliser ce projet comme bibliothèque dans vos propres projets Rust.

#### Configuration de Cargo.toml

```toml
[dependencies]
text-recognition = { path = "../text-recognition" }
# Ou si publié sur crates.io :
# text-recognition = "0.1.0"
```

#### Exemple 1 : Extraction simple de texte

```rust
use text_recognition::{OcrEngine, OcrConfig};

fn main() -> anyhow::Result<()> {
    // Créer une configuration par défaut
    let config = OcrConfig::default();
    
    // Créer le moteur OCR
    let mut engine = OcrEngine::new(config)?;
    
    // Extraire le texte depuis une image
    let text = engine.extract_text_from_file("image.png")?;
    
    println!("Texte extrait :\n{}", text);
    
    Ok(())
}
```

#### Exemple 2 : Utiliser un preset de configuration

```rust
use text_recognition::{OcrEngine, OcrConfig};

fn main() -> anyhow::Result<()> {
    // Utiliser le preset "document"
    let config = OcrConfig::document_preset();
    
    let mut engine = OcrEngine::new(config)?;
    let text = engine.extract_text_from_file("document.png")?;
    
    println!("{}", text);
    
    Ok(())
}
```

Presets disponibles :
- `OcrConfig::default()` : Configuration par défaut (PSM 3, langue française)
- `OcrConfig::document_preset()` : Optimisé pour documents texte
- `OcrConfig::screenshot_preset()` : Optimisé pour captures d'écran
- `OcrConfig::single_line_preset()` : Optimisé pour lignes de texte uniques
- `OcrConfig::photo_preset()` : Optimisé pour photos de documents

#### Exemple 3 : Configuration personnalisée

```rust
use text_recognition::{OcrEngine, OcrConfig, PageSegMode};

fn main() -> anyhow::Result<()> {
    // Créer une configuration personnalisée
    let mut config = OcrConfig::default();
    config.language = "eng".to_string();
    config.page_seg_mode = PageSegMode::SingleColumn;
    config.dpi = 300;
    
    let mut engine = OcrEngine::new(config)?;
    let text = engine.extract_text_from_file("image.png")?;
    
    println!("{}", text);
    
    Ok(())
}
```

#### Exemple 4 : Avec prétraitement d'image

```rust
use text_recognition::{OcrEngine, OcrConfig, PreprocessingConfig, BinarizationMethod};

fn main() -> anyhow::Result<()> {
    let config = OcrConfig::default();
    
    // Configuration du prétraitement
    let preprocessing = PreprocessingConfig {
        to_grayscale: true,
        binarize: true,
        binarization_method: BinarizationMethod::Otsu,
        denoise: true,
        adjust_contrast: true,
        contrast_factor: 1.5,
        deskew: false,
    };
    
    // Créer le moteur avec prétraitement
    let engine = OcrEngine::with_preprocessing(config, preprocessing)?;
    
    let text = engine.extract_text_from_file(std::path::Path::new("noisy_image.png"))?;
    
    println!("{}", text);
    
    Ok(())
}
```

#### Exemple 4b : Correction automatique d'orientation

```rust
use text_recognition::{OcrEngine, OcrConfig};
use std::path::Path;

fn main() -> anyhow::Result<()> {
    let engine = OcrEngine::new(OcrConfig::default())?;
    
    // Détecter l'orientation et corriger (image à l'envers, pivotée, etc.)
    let corrected = engine.detect_and_correct_orientation(Path::new("upside_down.png"))?;
    
    // Extraire le texte depuis l'image corrigée
    let text = engine.extract_text_from_image(&corrected)?;
    
    println!("{}", text);
    
    Ok(())
}
```

#### Exemple 5 : Calculer des métriques de qualité

```rust
use text_recognition::{OcrEngine, OcrConfig, compare_ocr_result};
use std::fs;

fn main() -> anyhow::Result<()> {
    let config = OcrConfig::default();
    let mut engine = OcrEngine::new(config)?;
    
    // Extraire le texte
    let ocr_text = engine.extract_text_from_file("document.png")?;
    
    // Lire le texte de référence
    let expected_text = fs::read_to_string("expected.txt")?;
    
    // Comparer et calculer les métriques
    let metrics = compare_ocr_result(&ocr_text, &expected_text);
    
    println!("CER: {:.2}%", metrics.cer * 100.0);
    println!("WER: {:.2}%", metrics.wer * 100.0);
    println!("Précision: {:.2}%", metrics.accuracy() * 100.0);
    println!("Distance de Levenshtein: {}", metrics.levenshtein_distance);
    
    Ok(())
}
```

#### Exemple 6 : Générer un rapport détaillé

```rust
use text_recognition::{OcrEngine, OcrConfig, generate_diff_report};
use std::fs;

fn main() -> anyhow::Result<()> {
    let config = OcrConfig::default();
    let mut engine = OcrEngine::new(config)?;
    
    let ocr_text = engine.extract_text_from_file("document.png")?;
    let expected_text = fs::read_to_string("expected.txt")?;
    
    // Générer un rapport complet formaté
    let report = generate_diff_report(&ocr_text, &expected_text);
    
    println!("{}", report);
    
    Ok(())
}
```

#### Exemple 7 : Tester plusieurs modes PSM

```rust
use text_recognition::{OcrEngine, OcrConfig, PageSegMode};

fn main() -> anyhow::Result<()> {
    let psm_modes = vec![
        PageSegMode::Auto,
        PageSegMode::SingleBlock,
        PageSegMode::SingleColumn,
        PageSegMode::SingleLine,
    ];
    
    for psm in psm_modes {
        let mut config = OcrConfig::default();
        config.page_seg_mode = psm;
        
        let mut engine = OcrEngine::new(config)?;
        let text = engine.extract_text_from_file("image.png")?;
        
        println!("=== PSM: {:?} ===", psm);
        println!("{}\n", text);
    }
    
    Ok(())
}
```

#### Exemple 8 : Traiter une image depuis la mémoire

```rust
use text_recognition::{OcrEngine, OcrConfig};
use image::DynamicImage;

fn main() -> anyhow::Result<()> {
    let config = OcrConfig::default();
    let mut engine = OcrEngine::new(config)?;
    
    // Charger une image depuis n'importe quelle source
    let img = image::open("image.png")?;
    
    // Ou créer/modifier une image programmatiquement
    let processed_img = process_image(img);
    
    // Extraire le texte depuis DynamicImage
    let text = engine.extract_text_from_image(&processed_img)?;
    
    println!("{}", text);
    
    Ok(())
}

fn process_image(img: DynamicImage) -> DynamicImage {
    // Appliquer des transformations personnalisées
    img.grayscale()
}
```

#### Exemple 9 : Utilisation avec variables Tesseract personnalisées

```rust
use text_recognition::{OcrEngine, OcrConfig};
use std::collections::HashMap;

fn main() -> anyhow::Result<()> {
    let mut config = OcrConfig::default();
    
    // Ajouter des variables Tesseract personnalisées
    let mut vars = HashMap::new();
    vars.insert("tessedit_char_whitelist".to_string(), 
                "0123456789ABCDEF".to_string());
    
    config.tesseract_variables = vars;
    
    let mut engine = OcrEngine::new(config)?;
    let text = engine.extract_text_from_file("hex_code.png")?;
    
    println!("{}", text);
    
    Ok(())
}
```

#### Exemple 10 : Comparer différents prétraitements

```rust
use text_recognition::{
    OcrEngine, OcrConfig, PreprocessingConfig, 
    BinarizationMethod, compare_ocr_result
};
use std::fs;

fn main() -> anyhow::Result<()> {
    let expected = fs::read_to_string("expected.txt")?;
    
    // Test sans prétraitement
    let config1 = OcrConfig::default();
    let mut engine1 = OcrEngine::new(config1)?;
    let text1 = engine1.extract_text_from_file("image.png")?;
    let metrics1 = compare_ocr_result(&text1, &expected);
    
    // Test avec binarisation Otsu
    let config2 = OcrConfig::default();
    let preprocessing2 = PreprocessingConfig {
        to_grayscale: true,
        binarize: true,
        binarization_method: BinarizationMethod::Otsu,
        denoise: false,
        adjust_contrast: false,
        contrast_factor: 1.0,
        deskew: false,
    };
    let engine2 = OcrEngine::with_preprocessing(config2, preprocessing2)?;
    let text2 = engine2.extract_text_from_file(std::path::Path::new("image.png"))?;
    let metrics2 = compare_ocr_result(&text2, &expected);
    
    // Test avec binarisation adaptative
    let config3 = OcrConfig::default();
    let preprocessing3 = PreprocessingConfig {
        to_grayscale: true,
        binarize: true,
        binarization_method: BinarizationMethod::Adaptive,
        denoise: true,
        adjust_contrast: true,
        contrast_factor: 1.3,
        deskew: false,
    };
    let engine3 = OcrEngine::with_preprocessing(config3, preprocessing3)?;
    let text3 = engine3.extract_text_from_file(std::path::Path::new("image.png"))?;
    let metrics3 = compare_ocr_result(&text3, &expected);
    
    println!("Sans prétraitement: CER={:.2}%, WER={:.2}%", 
             metrics1.cer * 100.0, metrics1.wer * 100.0);
    println!("Avec Otsu:          CER={:.2}%, WER={:.2}%", 
             metrics2.cer * 100.0, metrics2.wer * 100.0);
    println!("Avec Adaptive:      CER={:.2}%, WER={:.2}%", 
             metrics3.cer * 100.0, metrics3.wer * 100.0);
    
    Ok(())
}
```

#### Documentation complète

Pour plus de détails sur l'API, consultez la documentation générée :

```bash
cargo doc --open
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
- **Phase 6** : Documentation 🔄 (3/10 tâches)
- **Phase 7** : Extensions (optionnel)

**Total** : 61/67 tâches complétées (91.0%)

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
