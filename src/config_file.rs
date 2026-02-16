//! Chargement de la configuration depuis des fichiers JSON ou TOML.
//!
//! Ce module fournit la structure [`AppConfig`] et la fonction [`load_config`]
//! pour lire la configuration OCR et de prétraitement depuis un fichier externe.
//!
//! # Formats supportés
//!
//! - **JSON** : extension `.json`
//! - **TOML** : extension `.toml`
//!
//! # Exemple de fichier JSON
//!
//! ```json
//! {
//!   "ocr": {
//!     "language": "fra",
//!     "page_seg_mode": "Auto",
//!     "dpi": 300,
//!     "tesseract_variables": {}
//!   },
//!   "preprocessing": {
//!     "to_grayscale": true,
//!     "binarize": true,
//!     "binarization_method": "Otsu",
//!     "adjust_contrast": false,
//!     "contrast_factor": 1.0,
//!     "denoise": true,
//!     "deskew": false
//!   }
//! }
//! ```
//!
//! # Exemple de fichier TOML
//!
//! ```toml
//! [ocr]
//! language = "fra"
//! page_seg_mode = "Auto"
//! dpi = 300
//!
//! [ocr.tesseract_variables]
//!
//! [preprocessing]
//! to_grayscale = true
//! binarize = true
//! binarization_method = "Otsu"
//! adjust_contrast = false
//! contrast_factor = 1.0
//! denoise = true
//! deskew = false
//! ```

use crate::config::OcrConfig;
use crate::preprocessing::PreprocessingConfig;
use anyhow::{Context, Result, anyhow};
use serde::{Deserialize, Serialize};
use std::path::Path;

/// Configuration complète de l'application, chargeable depuis un fichier JSON ou TOML.
///
/// Les deux champs sont optionnels : un fichier peut ne contenir que la section `ocr`
/// ou que la section `preprocessing`.
///
/// # Exemple
///
/// ```
/// use text_recognition::config_file::AppConfig;
/// use text_recognition::config::OcrConfig;
///
/// let config = AppConfig {
///     ocr: Some(OcrConfig::default()),
///     preprocessing: None,
/// };
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    /// Configuration du moteur OCR (optionnel).
    pub ocr: Option<OcrConfig>,

    /// Configuration du prétraitement d'images (optionnel).
    pub preprocessing: Option<PreprocessingConfig>,
}

impl Default for AppConfig {
    fn default() -> Self {
        Self {
            ocr: Some(OcrConfig::default()),
            preprocessing: None,
        }
    }
}

/// Charge une configuration depuis un fichier JSON ou TOML.
///
/// Le format est déterminé par l'extension du fichier :
/// - `.json` → désérialisation JSON
/// - `.toml` → désérialisation TOML
/// - Toute autre extension → erreur
///
/// # Arguments
///
/// * `path` - Chemin vers le fichier de configuration
///
/// # Exemple
///
/// ```no_run
/// use text_recognition::config_file::load_config;
/// use std::path::Path;
///
/// let config = load_config(Path::new("config.toml")).unwrap();
/// if let Some(ocr) = config.ocr {
///     println!("Langue : {}", ocr.language);
/// }
/// ```
///
/// # Erreurs
///
/// Retourne une erreur si :
/// - Le fichier n'existe pas ou n'est pas lisible
/// - L'extension n'est pas `.json` ou `.toml`
/// - Le contenu n'est pas un JSON/TOML valide
/// - Les champs ne correspondent pas à la structure attendue
pub fn load_config(path: &Path) -> Result<AppConfig> {
    let extension = path
        .extension()
        .and_then(|e| e.to_str())
        .map(|e| e.to_lowercase());

    let content = std::fs::read_to_string(path)
        .with_context(|| format!("Impossible de lire le fichier '{}'", path.display()))?;

    match extension.as_deref() {
        Some("json") => serde_json::from_str(&content)
            .with_context(|| format!("Fichier JSON invalide : '{}'", path.display())),
        Some("toml") => toml::from_str(&content)
            .with_context(|| format!("Fichier TOML invalide : '{}'", path.display())),
        other => Err(anyhow!(
            "Extension non supportée : '{}'. Utilisez .json ou .toml",
            other.unwrap_or("(aucune)")
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::config::PageSegMode;
    use crate::preprocessing::BinarizationMethod;
    use std::io::Write;
    use tempfile::NamedTempFile;

    fn write_temp(extension: &str, content: &str) -> NamedTempFile {
        let mut file = tempfile::Builder::new()
            .suffix(extension)
            .tempfile()
            .expect("Impossible de créer un fichier temporaire");
        file.write_all(content.as_bytes())
            .expect("Impossible d'écrire dans le fichier temporaire");
        file
    }

    // ─── JSON ────────────────────────────────────────────────────────────────

    #[test]
    fn test_load_json_ocr_only() {
        let content = r#"{
            "ocr": {
                "language": "eng",
                "page_seg_mode": "SingleLine",
                "dpi": 150,
                "tesseract_variables": {}
            }
        }"#;
        let file = write_temp(".json", content);
        let config = load_config(file.path()).unwrap();

        let ocr = config.ocr.expect("Section ocr absente");
        assert_eq!(ocr.language, "eng");
        assert_eq!(ocr.page_seg_mode, PageSegMode::SingleLine);
        assert_eq!(ocr.dpi, 150);
        assert!(config.preprocessing.is_none());
    }

    #[test]
    fn test_load_json_preprocessing_only() {
        let content = r#"{
            "preprocessing": {
                "to_grayscale": true,
                "binarize": true,
                "binarization_method": "Otsu",
                "adjust_contrast": false,
                "contrast_factor": 1.0,
                "denoise": true,
                "deskew": false
            }
        }"#;
        let file = write_temp(".json", content);
        let config = load_config(file.path()).unwrap();

        assert!(config.ocr.is_none());
        let prep = config.preprocessing.expect("Section preprocessing absente");
        assert!(prep.to_grayscale);
        assert!(prep.binarize);
        assert_eq!(prep.binarization_method, BinarizationMethod::Otsu);
        assert!(prep.denoise);
        assert!(!prep.deskew);
    }

    #[test]
    fn test_load_json_full_config() {
        let content = r#"{
            "ocr": {
                "language": "fra",
                "page_seg_mode": "Auto",
                "dpi": 300,
                "tesseract_variables": {
                    "tessedit_char_whitelist": "0123456789"
                }
            },
            "preprocessing": {
                "to_grayscale": true,
                "binarize": true,
                "binarization_method": { "Fixed": 128 },
                "adjust_contrast": true,
                "contrast_factor": 1.5,
                "denoise": false,
                "deskew": true
            }
        }"#;
        let file = write_temp(".json", content);
        let config = load_config(file.path()).unwrap();

        let ocr = config.ocr.unwrap();
        assert_eq!(ocr.language, "fra");
        assert_eq!(ocr.dpi, 300);
        assert_eq!(
            ocr.tesseract_variables.get("tessedit_char_whitelist"),
            Some(&"0123456789".to_string())
        );

        let prep = config.preprocessing.unwrap();
        assert_eq!(prep.binarization_method, BinarizationMethod::Fixed(128));
        assert!(prep.adjust_contrast);
        assert!((prep.contrast_factor - 1.5).abs() < 0.001);
        assert!(prep.deskew);
    }

    #[test]
    fn test_load_json_invalid_content() {
        let file = write_temp(".json", "{ invalid json }");
        assert!(load_config(file.path()).is_err());
    }

    // ─── TOML ────────────────────────────────────────────────────────────────

    #[test]
    fn test_load_toml_ocr_only() {
        let content = r#"
[ocr]
language = "fra"
page_seg_mode = "Auto"
dpi = 300

[ocr.tesseract_variables]
"#;
        let file = write_temp(".toml", content);
        let config = load_config(file.path()).unwrap();

        let ocr = config.ocr.expect("Section ocr absente");
        assert_eq!(ocr.language, "fra");
        assert_eq!(ocr.page_seg_mode, PageSegMode::Auto);
        assert_eq!(ocr.dpi, 300);
        assert!(config.preprocessing.is_none());
    }

    #[test]
    fn test_load_toml_full_config() {
        let content = r#"
[ocr]
language = "fra"
page_seg_mode = "SingleBlock"
dpi = 300

[ocr.tesseract_variables]

[preprocessing]
to_grayscale = true
binarize = true
binarization_method = "Otsu"
adjust_contrast = true
contrast_factor = 1.5
denoise = true
deskew = false
"#;
        let file = write_temp(".toml", content);
        let config = load_config(file.path()).unwrap();

        let ocr = config.ocr.unwrap();
        assert_eq!(ocr.page_seg_mode, PageSegMode::SingleBlock);

        let prep = config.preprocessing.unwrap();
        assert!(prep.to_grayscale);
        assert!(prep.binarize);
        assert!(prep.adjust_contrast);
        assert!((prep.contrast_factor - 1.5).abs() < 0.001);
        assert!(prep.denoise);
        assert!(!prep.deskew);
    }

    #[test]
    fn test_load_toml_invalid_content() {
        let file = write_temp(".toml", "invalid = toml [[[ content");
        assert!(load_config(file.path()).is_err());
    }

    // ─── Erreurs ─────────────────────────────────────────────────────────────

    #[test]
    fn test_load_unsupported_extension() {
        let file = write_temp(".yaml", "key: value");
        let result = load_config(file.path());
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("Extension non supportée")
        );
    }

    #[test]
    fn test_load_nonexistent_file() {
        let result = load_config(Path::new("/tmp/this_file_does_not_exist.json"));
        assert!(result.is_err());
    }
}
