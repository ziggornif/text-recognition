//! Bibliothèque d'OCR (Optical Character Recognition) pour Rust.
//!
//! Cette bibliothèque fournit une interface simple et flexible pour
//! extraire du texte depuis des images en utilisant Tesseract OCR.
//!
//! # Exemple d'utilisation
//!
//! ```no_run
//! use text_recognition::{OcrEngine, OcrConfig};
//! use std::path::Path;
//!
//! let config = OcrConfig::default();
//! let engine = OcrEngine::new(config)?;
//! let text = engine.extract_text_from_file(Path::new("image.png"))?;
//! println!("Texte extrait: {}", text);
//! # Ok::<(), anyhow::Error>(())
//! ```
//!
//! # Modules
//!
//! - `config` : Configuration du moteur OCR et modes de segmentation
//! - `ocr` : Moteur OCR principal pour l'extraction de texte
//! - `preprocessing` : Prétraitement d'images pour améliorer la qualité OCR
//! - `metrics` : Calcul de métriques de qualité OCR (CER, WER)

pub mod config;
pub mod metrics;
pub mod ocr;
pub mod preprocessing;

// Exports publics pour faciliter l'utilisation de la bibliothèque
pub use config::{OcrConfig, PageSegMode};
pub use metrics::{
    OcrMetrics, TextError, calculate_cer, calculate_wer, compare_ocr_result, generate_diff_report,
    levenshtein_distance,
};
pub use ocr::OcrEngine;
pub use preprocessing::{BinarizationMethod, Orientation, PreprocessingConfig, rotate_orientation};
