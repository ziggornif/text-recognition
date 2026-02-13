# Plan de Développement - Text Recognition

## 📚 Contexte et Objectifs

**Objectif** : Créer un projet Rust éducatif pour comprendre et maîtriser le paramétrage de Tesseract OCR, avec un focus sur l'extraction de texte en français.

**Environnement** :
- Tesseract 5.3.4 installé
- Langues disponibles : français (fra), anglais (eng), OSD
- Rust 1.93.1
- Linux

**Compétences visées** :
- Modes de segmentation de page (PSM)
- Prétraitement d'images pour améliorer l'OCR
- Configuration des paramètres Tesseract
- Métriques de qualité et comparaison de résultats
- Utilisation de différents modèles de langues

---

## 🏗️ Structure du Projet

```
text-recognition/
├── Cargo.toml                    # Configuration du projet + dépendances
├── README.md                     # Documentation utilisateur
├── PLAN.md                       # Ce document (plan détaillé)
├── TODO.md                       # Liste des tâches à réaliser
├── CLAUDE.md                     # Guide pour l'agent Claude
├── docs/
│   └── parametrage-tesseract.md  # Documentation approfondie
├── src/
│   ├── lib.rs                    # Bibliothèque principale (exports publics)
│   ├── main.rs                   # CLI simple
│   ├── ocr.rs                    # Wrapper Tesseract avec configuration
│   ├── preprocessing.rs          # Fonctions de prétraitement d'images
│   ├── config.rs                 # Structures de configuration OCR
│   └── metrics.rs                # Calcul de métriques de qualité
├── tests/
│   ├── integration_tests.rs      # Tests d'intégration généraux
│   ├── psm_tests.rs              # Tests spécifiques des modes PSM
│   ├── preprocessing_tests.rs    # Tests du prétraitement
│   └── metrics_tests.rs          # Tests des métriques
└── test_images/                  # Bibliothèque d'images de test
    ├── simple/                   # Images simples, bonne qualité
    │   ├── document_propre.png
    │   └── capture_texte.png
    ├── medium/                   # Qualité moyenne avec défis
    │   ├── document_incline.png
    │   ├── fond_colore.png
    │   └── ombres_legeres.png
    ├── complex/                  # Cas difficiles
    │   ├── document_froisse.png
    │   ├── faible_contraste.png
    │   ├── angle_oblique.png
    │   └── fond_texture.png
    └── expected/                 # Fichiers .txt avec texte attendu
        ├── document_propre.txt
        ├── capture_texte.txt
        └── ...
```

---

## 📦 Dépendances Rust

### Dépendances principales

```toml
[dependencies]
tesseract = "0.15"              # Bindings Rust pour Tesseract
image = "0.25"                  # Manipulation d'images (prétraitement)
imageproc = "0.25"              # Opérations de traitement d'images
clap = { version = "4.5", features = ["derive"] }  # Parsing arguments CLI
anyhow = "1.0"                  # Gestion d'erreurs simplifiée
thiserror = "1.0"               # Création d'erreurs personnalisées
serde = { version = "1.0", features = ["derive"] }  # Sérialisation
serde_json = "1.0"              # Format JSON pour configuration

[dev-dependencies]
tempfile = "3.10"               # Fichiers temporaires pour tests
approx = "0.5"                  # Comparaisons flottantes dans tests
```

### Justification des choix

- **tesseract** : Bindings officiels, bien maintenu, API ergonomique
- **image + imageproc** : Écosystème standard Rust pour traitement d'images
- **clap** : Standard moderne pour CLI, avec macros dérivées
- **anyhow/thiserror** : Combo standard pour gestion d'erreurs Rust
- **serde/serde_json** : Pour sauvegarder/charger des configurations

---

## 🎯 Fonctionnalités Détaillées

### 1. Module `ocr.rs` - Wrapper Tesseract

**Responsabilités** :
- Encapsulation de l'API Tesseract
- Application de configurations OCR
- Extraction de texte avec différents modes

**API Publique** :
```rust
/// Configuration principale pour l'OCR
pub struct OcrEngine {
    // Instance Tesseract interne
}

impl OcrEngine {
    /// Crée un nouveau moteur OCR
    pub fn new(config: OcrConfig) -> Result<Self>;
    
    /// Extrait le texte d'une image (chemin de fichier)
    pub fn extract_text_from_file(&mut self, path: &Path) -> Result<String>;
    
    /// Extrait le texte d'une image en mémoire
    pub fn extract_text_from_image(&mut self, img: &DynamicImage) -> Result<String>;
    
    /// Retourne le niveau de confiance moyen du dernier OCR
    pub fn get_confidence(&mut self) -> i32;
    
    /// Extrait avec format HOCR (HTML + bounding boxes)
    pub fn extract_hocr(&mut self, path: &Path) -> Result<String>;
    
    /// Extrait avec format TSV (colonnes avec coordonnées)
    pub fn extract_tsv(&mut self, path: &Path) -> Result<String>;
}
```

**Détails d'implémentation** :
- Initialisation de Tesseract avec langue française par défaut
- Support de la configuration des variables Tesseract
- Gestion d'erreurs explicites
- Logs optionnels pour le debug

---

### 2. Module `config.rs` - Configuration OCR

**Responsabilités** :
- Définir les structures de configuration
- Présets pour différents cas d'usage
- Sérialisation/désérialisation

**Structures** :
```rust
/// Configuration complète pour l'OCR
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OcrConfig {
    /// Langue(s) à utiliser (ex: "fra", "fra+eng")
    pub language: String,
    
    /// Mode de segmentation de page
    pub page_seg_mode: PageSegMode,
    
    /// Variables Tesseract personnalisées
    pub tesseract_variables: HashMap<String, String>,
    
    /// Appliquer un prétraitement avant OCR
    pub preprocessing: Option<PreprocessingConfig>,
}

/// Modes de segmentation de page
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum PageSegMode {
    /// PSM 0: Détection orientation et script uniquement
    OsdOnly,
    /// PSM 1: Segmentation auto avec OSD
    AutoWithOsd,
    /// PSM 2: Segmentation auto sans OSD
    AutoOnly,
    /// PSM 3: Segmentation auto complète (défaut)
    Auto,
    /// PSM 4: Colonne unique de texte variable
    SingleColumn,
    /// PSM 5: Bloc uniforme de texte vertical
    SingleBlockVertText,
    /// PSM 6: Bloc uniforme de texte (par défaut Tesseract)
    SingleBlock,
    /// PSM 7: Une seule ligne de texte
    SingleLine,
    /// PSM 8: Un seul mot
    SingleWord,
    /// PSM 9: Un mot dans un cercle
    CircleWord,
    /// PSM 10: Un seul caractère
    SingleChar,
    /// PSM 11: Texte épars sans ordre particulier
    SparseText,
    /// PSM 12: Texte épars avec OSD
    SparseTextOsd,
    /// PSM 13: Ligne brute (bypass des hacks Tesseract)
    RawLine,
}

impl OcrConfig {
    /// Configuration par défaut (français, auto)
    pub fn default() -> Self;
    
    /// Preset pour documents scannés
    pub fn document_preset() -> Self;
    
    /// Preset pour captures d'écran
    pub fn screenshot_preset() -> Self;
    
    /// Preset pour photos
    pub fn photo_preset() -> Self;
    
    /// Preset pour texte sur une seule ligne
    pub fn single_line_preset() -> Self;
    
    /// Charge depuis un fichier JSON
    pub fn from_file(path: &Path) -> Result<Self>;
    
    /// Sauvegarde dans un fichier JSON
    pub fn save_to_file(&self, path: &Path) -> Result<()>;
}
```

**Variables Tesseract utiles** :
- `tessedit_char_whitelist` : Liste de caractères autorisés
- `tessedit_char_blacklist` : Liste de caractères interdits
- `preserve_interword_spaces` : Préserver les espaces multiples
- `user_defined_dpi` : DPI de l'image source
- `min_characters_to_try` : Nombre min de caractères à tenter

---

### 3. Module `preprocessing.rs` - Prétraitement d'Images

**Responsabilités** :
- Améliorer la qualité des images avant OCR
- Fonctions de traitement d'image modulaires
- Pipeline de prétraitement configurable

**API Publique** :
```rust
/// Configuration du prétraitement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PreprocessingConfig {
    pub grayscale: bool,
    pub binarization: Option<BinarizationMethod>,
    pub contrast_adjustment: Option<f32>,
    pub denoise: bool,
    pub deskew: bool,
}

/// Méthodes de binarisation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BinarizationMethod {
    /// Seuil d'Otsu (automatique)
    Otsu,
    /// Seuil fixe (0-255)
    Threshold(u8),
    /// Seuil adaptatif (taille de bloc)
    Adaptive(u32),
}

/// Applique un pipeline de prétraitement
pub fn preprocess_image(
    img: &DynamicImage,
    config: &PreprocessingConfig,
) -> Result<DynamicImage>;

/// Conversion en niveaux de gris
pub fn to_grayscale(img: &DynamicImage) -> GrayImage;

/// Binarisation (noir et blanc)
pub fn binarize(img: &GrayImage, method: BinarizationMethod) -> GrayImage;

/// Ajustement du contraste
pub fn adjust_contrast(img: &GrayImage, factor: f32) -> GrayImage;

/// Débruitage
pub fn denoise(img: &GrayImage) -> GrayImage;

/// Détection et correction de l'inclinaison
pub fn deskew(img: &GrayImage) -> Result<GrayImage>;
```

**Algorithmes à implémenter** :
1. **Niveaux de gris** : Conversion RGB → Luminance
2. **Binarisation Otsu** : Calcul du seuil optimal automatiquement
3. **Binarisation adaptative** : Seuil local par zones
4. **Ajustement contraste** : Multiplication des intensités
5. **Débruitage** : Filtre médian ou gaussien
6. **Deskew** : Détection d'angle via transformée de Hough + rotation

---

### 4. Module `metrics.rs` - Métriques de Qualité

**Responsabilités** :
- Comparer texte obtenu vs texte attendu
- Calculer des métriques de précision
- Générer des rapports détaillés

**API Publique** :
```rust
/// Résultat d'une comparaison OCR
#[derive(Debug, Clone)]
pub struct OcrMetrics {
    /// Taux d'erreur au niveau caractère (Character Error Rate)
    pub cer: f64,
    
    /// Taux d'erreur au niveau mot (Word Error Rate)
    pub wer: f64,
    
    /// Précision globale (0.0 à 1.0)
    pub accuracy: f64,
    
    /// Niveau de confiance Tesseract
    pub confidence: i32,
    
    /// Temps d'exécution (ms)
    pub execution_time_ms: u128,
    
    /// Détails des erreurs
    pub errors: Vec<TextError>,
}

/// Type d'erreur de reconnaissance
#[derive(Debug, Clone)]
pub enum TextError {
    Substitution { expected: char, got: char, position: usize },
    Insertion { char: char, position: usize },
    Deletion { expected: char, position: usize },
}

/// Compare le texte obtenu avec le texte attendu
pub fn compare_ocr_result(expected: &str, obtained: &str) -> OcrMetrics;

/// Calcule le Character Error Rate (distance de Levenshtein normalisée)
pub fn calculate_cer(expected: &str, obtained: &str) -> f64;

/// Calcule le Word Error Rate
pub fn calculate_wer(expected: &str, obtained: &str) -> f64;

/// Génère un rapport détaillé des différences
pub fn generate_diff_report(expected: &str, obtained: &str) -> String;
```

**Algorithmes** :
- **CER** : Distance de Levenshtein au niveau caractères / longueur texte attendu
- **WER** : Distance de Levenshtein au niveau mots / nombre de mots attendus
- **Précision** : 1.0 - CER
- **Diff** : Algorithme de différence (Myers' diff ou similaire)

---

### 5. Module `main.rs` - CLI Simple

**Responsabilités** :
- Interface en ligne de commande
- Appel des fonctions de la lib
- Affichage formaté des résultats

**Arguments CLI** :
```bash
# Utilisation basique
text-recognition <IMAGE_PATH>

# Avec options
text-recognition <IMAGE_PATH> \
    --lang fra \
    --psm 6 \
    --preprocess binarize,deskew \
    --output result.txt \
    --verbose

# Comparer avec un texte attendu
text-recognition <IMAGE_PATH> \
    --expected expected.txt \
    --metrics

# Utiliser une configuration JSON
text-recognition <IMAGE_PATH> \
    --config custom_config.json

# Tester tous les modes PSM
text-recognition <IMAGE_PATH> --test-all-psm
```

**Structure CLI (avec clap)** :
```rust
use clap::Parser;

#[derive(Parser)]
#[command(name = "text-recognition")]
#[command(about = "Outil d'extraction de texte avec Tesseract OCR")]
struct Cli {
    /// Chemin vers l'image à analyser
    image_path: PathBuf,
    
    /// Langue(s) à utiliser (ex: "fra", "fra+eng")
    #[arg(short, long, default_value = "fra")]
    lang: String,
    
    /// Mode de segmentation de page (0-13)
    #[arg(short, long)]
    psm: Option<u8>,
    
    /// Appliquer un prétraitement (comma-separated: grayscale,binarize,deskew)
    #[arg(long)]
    preprocess: Option<String>,
    
    /// Fichier de sortie (affiche sur stdout si absent)
    #[arg(short, long)]
    output: Option<PathBuf>,
    
    /// Fichier texte attendu pour comparaison
    #[arg(short, long)]
    expected: Option<PathBuf>,
    
    /// Afficher les métriques de qualité
    #[arg(short, long)]
    metrics: bool,
    
    /// Tester tous les modes PSM et afficher les résultats
    #[arg(long)]
    test_all_psm: bool,
    
    /// Charger configuration depuis JSON
    #[arg(short, long)]
    config: Option<PathBuf>,
    
    /// Mode verbeux (affiche logs détaillés)
    #[arg(short, long)]
    verbose: bool,
}
```

---

### 6. Module `lib.rs` - Exports Publics

**Responsabilités** :
- Exposer l'API publique de la bibliothèque
- Documentation de haut niveau

```rust
//! # Text Recognition
//!
//! Bibliothèque Rust pour l'extraction de texte d'images avec Tesseract OCR.
//!
//! ## Exemple d'utilisation
//!
//! ```rust
//! use text_recognition::{OcrEngine, OcrConfig};
//!
//! let config = OcrConfig::default();
//! let mut engine = OcrEngine::new(config)?;
//! let text = engine.extract_text_from_file("image.png")?;
//! println!("Texte extrait : {}", text);
//! ```

pub mod ocr;
pub mod config;
pub mod preprocessing;
pub mod metrics;

pub use ocr::OcrEngine;
pub use config::{OcrConfig, PageSegMode, PreprocessingConfig};
pub use preprocessing::{preprocess_image, BinarizationMethod};
pub use metrics::{OcrMetrics, compare_ocr_result};

// Ré-export des erreurs communes
pub use anyhow::{Result, Error};
```

---

## 🧪 Stratégie de Tests

### Tests unitaires (dans chaque module)

**`src/preprocessing.rs`** :
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_grayscale_conversion() {
        // Créer une image test RGB
        // Convertir en grayscale
        // Vérifier que les pixels sont corrects
    }
    
    #[test]
    fn test_binarization_otsu() {
        // Image avec contraste connu
        // Binariser
        // Vérifier que le seuil est correct
    }
    
    #[test]
    fn test_contrast_adjustment() {
        // Image test
        // Ajuster contraste (facteur 1.5)
        // Vérifier que les valeurs sont modifiées correctement
    }
}
```

**`src/metrics.rs`** :
```rust
#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_cer_identical_strings() {
        assert_eq!(calculate_cer("bonjour", "bonjour"), 0.0);
    }
    
    #[test]
    fn test_cer_one_error() {
        // "bonjour" vs "boujour" (n->u)
        let cer = calculate_cer("bonjour", "boujour");
        assert!((cer - 0.142857).abs() < 0.001); // 1/7
    }
    
    #[test]
    fn test_wer_calculation() {
        let expected = "Bonjour tout le monde";
        let obtained = "Bonjour toutlemonde"; // Mot fusionné
        let wer = calculate_wer(expected, obtained);
        // 2 erreurs (suppression "tout", insertion "toutlemonde") / 4 mots
        assert!((wer - 0.5).abs() < 0.1);
    }
}
```

### Tests d'intégration (`tests/`)

**`tests/integration_tests.rs`** :
```rust
use text_recognition::*;
use std::path::PathBuf;

#[test]
fn test_extract_simple_document() {
    let mut engine = OcrEngine::new(OcrConfig::document_preset()).unwrap();
    let text = engine.extract_text_from_file("test_images/simple/document_propre.png").unwrap();
    
    // Vérifier que le texte contient certains mots clés
    assert!(text.contains("Bonjour"));
    assert!(text.len() > 10);
}

#[test]
fn test_confidence_level() {
    let mut engine = OcrEngine::new(OcrConfig::default()).unwrap();
    engine.extract_text_from_file("test_images/simple/document_propre.png").unwrap();
    let confidence = engine.get_confidence();
    
    // Document propre devrait avoir confiance > 80
    assert!(confidence > 80);
}
```

**`tests/psm_tests.rs`** :
```rust
use text_recognition::*;

#[test]
fn test_all_psm_modes() {
    let test_image = "test_images/simple/document_propre.png";
    
    for psm in [
        PageSegMode::Auto,
        PageSegMode::SingleBlock,
        PageSegMode::SingleLine,
        PageSegMode::SingleColumn,
    ] {
        let mut config = OcrConfig::default();
        config.page_seg_mode = psm;
        
        let mut engine = OcrEngine::new(config).unwrap();
        let result = engine.extract_text_from_file(test_image);
        
        assert!(result.is_ok(), "PSM {:?} failed", psm);
    }
}

#[test]
fn test_single_line_mode() {
    // Image contenant une seule ligne
    let mut config = OcrConfig::single_line_preset();
    let mut engine = OcrEngine::new(config).unwrap();
    
    let text = engine.extract_text_from_file("test_images/simple/single_line.png").unwrap();
    
    // Vérifier qu'il n'y a pas de retour à la ligne
    assert!(!text.contains('\n'));
}
```

**`tests/preprocessing_tests.rs`** :
```rust
use text_recognition::*;

#[test]
fn test_preprocessing_improves_ocr() {
    let image_path = "test_images/medium/faible_contraste.png";
    
    // Sans prétraitement
    let mut engine_no_prep = OcrEngine::new(OcrConfig::default()).unwrap();
    let text_no_prep = engine_no_prep.extract_text_from_file(image_path).unwrap();
    let conf_no_prep = engine_no_prep.get_confidence();
    
    // Avec prétraitement
    let mut config_with_prep = OcrConfig::default();
    config_with_prep.preprocessing = Some(PreprocessingConfig {
        grayscale: true,
        binarization: Some(BinarizationMethod::Otsu),
        contrast_adjustment: Some(1.5),
        denoise: true,
        deskew: false,
    });
    
    let mut engine_prep = OcrEngine::new(config_with_prep).unwrap();
    let text_prep = engine_prep.extract_text_from_file(image_path).unwrap();
    let conf_prep = engine_prep.get_confidence();
    
    // Le prétraitement devrait améliorer la confiance
    assert!(conf_prep >= conf_no_prep);
}
```

**`tests/metrics_tests.rs`** :
```rust
use text_recognition::*;
use std::fs;

#[test]
fn test_ocr_accuracy_with_expected() {
    let image_path = "test_images/simple/document_propre.png";
    let expected_text = fs::read_to_string("test_images/expected/document_propre.txt").unwrap();
    
    let mut engine = OcrEngine::new(OcrConfig::default()).unwrap();
    let obtained_text = engine.extract_text_from_file(image_path).unwrap();
    
    let metrics = compare_ocr_result(&expected_text, &obtained_text);
    
    // Pour une image simple, on attend une bonne précision
    assert!(metrics.accuracy > 0.95); // > 95% de précision
    assert!(metrics.cer < 0.05);      // < 5% d'erreurs caractères
}
```

---

## 📖 Documentation

### `README.md` - Documentation Utilisateur

Contenu :
- Description du projet et objectifs
- Installation (prérequis Tesseract)
- Guide de démarrage rapide
- Exemples d'utilisation CLI et lib
- Organisation du code
- Comment contribuer / apprendre

### `docs/parametrage-tesseract.md` - Guide Complet

Contenu :
- **Modes PSM** : Description détaillée de chaque mode avec cas d'usage
- **Variables Tesseract** : Liste et explication des variables utiles
- **Prétraitement** : Techniques et quand les utiliser
- **Langues et modèles** : Comment utiliser différents modèles
- **Résultats de tests** : Tableaux comparatifs des performances
- **Bonnes pratiques** : Recommandations basées sur les expérimentations

Exemple de structure :

```markdown
# Paramétrage de Tesseract OCR

## 1. Modes de Segmentation de Page (PSM)

### PSM 0 - OSD Only (Orientation and Script Detection)
**Utilisation** : Détection de l'orientation et du script uniquement, pas d'OCR.
**Cas d'usage** : Détecter si le texte est à l'envers ou quelle écriture (latin, arabe, etc.)

### PSM 3 - Fully Automatic (défaut)
**Utilisation** : Segmentation automatique complète, sans détection d'orientation.
**Cas d'usage** : Documents standards avec mise en page complexe (colonnes, paragraphes, etc.)
**Performance** : ⭐⭐⭐⭐ (bon compromis général)

### PSM 6 - Single Block
**Utilisation** : Assume un bloc uniforme de texte.
**Cas d'usage** : Paragraphes sans structure complexe, captures d'écran de texte
**Performance** : ⭐⭐⭐⭐⭐ (meilleur pour texte simple)

### PSM 7 - Single Line
**Utilisation** : Traite l'image comme une seule ligne de texte.
**Cas d'usage** : Titres, en-têtes, champs de formulaire
**Performance** : ⭐⭐⭐⭐⭐ (excellent si effectivement une ligne)

...

## 2. Variables Tesseract Importantes

### `tessedit_char_whitelist`
**Description** : Liste des caractères autorisés (tous les autres sont ignorés)
**Exemple** : `"0123456789"` pour extraire uniquement des chiffres
**Cas d'usage** : Extraction de codes, numéros de série, dates

### `preserve_interword_spaces`
**Description** : Préserve les espaces multiples entre les mots
**Valeur** : "0" ou "1" (défaut: 0)
**Cas d'usage** : Tableaux formatés avec espaces

...

## 3. Techniques de Prétraitement

### Binarisation (Noir & Blanc)
**Objectif** : Séparer le texte du fond
**Méthodes** :
- **Otsu** : Automatique, bon pour la plupart des cas
- **Seuil fixe** : Quand on connaît le niveau optimal
- **Adaptatif** : Pour fonds non uniformes

**Quand l'utiliser** :
- ✅ Faible contraste texte/fond
- ✅ Fond coloré ou bruyant
- ❌ Texte déjà bien contrasté (peut dégrader)

...
```

---

## 🗓️ Plan d'Implémentation par Phases

### Phase 1 : Fondations (1-2h)
**Objectif** : Projet fonctionnel avec extraction basique

Tâches : 1.1 à 1.12

**Validation** : Capable d'extraire du texte d'une image et l'afficher

---

### Phase 2 : Configuration Complète (1-2h)
**Objectif** : Support de tous les modes PSM et variables Tesseract

Tâches : 2.1 à 2.10

**Validation** : Peut configurer et tester différents modes PSM via CLI

---

### Phase 3 : Prétraitement d'Images (2-3h)
**Objectif** : Améliorer OCR via traitement d'images

Tâches : 3.1 à 3.14

**Validation** : Prétraitement améliore OCR sur images de qualité moyenne

---

### Phase 4 : Métriques et Comparaison (1-2h)
**Objectif** : Mesurer la qualité de l'OCR

Tâches : 4.1 à 4.11

**Validation** : Peut comparer résultat OCR avec texte attendu et obtenir métriques

---

### Phase 5 : Tests et Images (2-3h)
**Objectif** : Construire suite de tests complète

Tâches : 5.1 à 5.11

**Validation** : Tous les tests passent, projet robuste

---

### Phase 6 : Documentation (1-2h)
**Objectif** : Documentation complète et exemples

Tâches : 6.1 à 6.10

**Validation** : Documentation claire et complète

---

### Phase 7 (Optionnelle) : Fonctionnalités Avancées

**Idées d'extensions** :
- Support de fichiers de configuration JSON/TOML
- Mode batch (traiter plusieurs images)
- Export de métriques en CSV
- Visualisation des bounding boxes (format HOCR)
- Comparaison de performances entre prétraitements
- Support d'autres langues (téléchargement auto de modèles)
- Interface web simple (avec WASM ou serveur HTTP)

---

## 🎓 Aspects Pédagogiques

### Compétences Rust développées
- ✅ Organisation d'un projet multi-modules
- ✅ Gestion d'erreurs avec `Result`, `anyhow`, `thiserror`
- ✅ CLI avec `clap` et macros dérivées
- ✅ FFI avec bibliothèque C (Tesseract)
- ✅ Tests unitaires et d'intégration
- ✅ Sérialisation avec `serde`
- ✅ Manipulation d'images (crate `image`)
- ✅ Documentation rustdoc

### Compétences OCR/Tesseract
- ✅ Compréhension des modes PSM et leurs cas d'usage
- ✅ Impact du prétraitement sur la qualité OCR
- ✅ Configuration de Tesseract pour différents scénarios
- ✅ Mesure de performance (CER, WER, confiance)
- ✅ Identification de cas limites et solutions

---

## 📊 Exemples de Résultats Attendus

### Tableau comparatif PSM (à générer lors des tests)

| Mode PSM | Image Simple | Image Medium | Image Complex | Temps (ms) |
|----------|-------------|--------------|---------------|------------|
| 3 (Auto) | 98.5% | 85.2% | 65.0% | 350 |
| 6 (SingleBlock) | 99.1% | 88.5% | 70.3% | 280 |
| 7 (SingleLine) | 99.8% | N/A | N/A | 180 |
| 11 (SparseText) | 95.2% | 90.1% | 78.5% | 420 |

### Impact du prétraitement

| Prétraitement | Sans | Grayscale | + Binarize | + Contrast | + Denoise |
|---------------|------|-----------|------------|------------|-----------|
| Précision | 75% | 78% | 85% | 89% | 91% |
| Temps (ms) | 250 | 270 | 310 | 330 | 380 |

---

## ✅ Checklist Finale

Avant de considérer le projet terminé :

- [ ] Tous les modules sont implémentés et documentés
- [ ] Suite de tests complète qui passe
- [ ] Au moins 10 images de test dans `test_images/`
- [ ] README.md complet avec exemples
- [ ] Documentation Tesseract (`docs/parametrage-tesseract.md`)
- [ ] CLI fonctionnelle avec toutes les options
- [ ] Possibilité d'utiliser comme bibliothèque externe
- [ ] Code formaté (`cargo fmt`) et lint propre (`cargo clippy`)
- [ ] Commentaires et documentation rustdoc
- [ ] Exemples d'utilisation testés et fonctionnels

---

## 🚀 Pour Aller Plus Loin

**Idées de projets dérivés** :
1. **OCR Comparator** : Application web pour comparer différentes configs
2. **Document Processor** : Pipeline complet scan → OCR → PDF searchable
3. **Receipt Parser** : Extraction structurée de données de tickets de caisse
4. **License Plate Reader** : Reconnaissance de plaques d'immatriculation
5. **Form Extractor** : Remplir automatiquement des formulaires depuis des scans

---

*Ce plan est un document vivant qui sera mis à jour au fur et à mesure de l'implémentation avec les découvertes et ajustements nécessaires.*
