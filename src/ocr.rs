//! Module OCR pour l'extraction de texte depuis des images.
//!
//! Ce module fournit la structure `OcrEngine` qui encapsule
//! le moteur Tesseract OCR et permet d'extraire du texte depuis
//! des images avec différentes configurations.

use crate::config::OcrConfig;

/// Moteur OCR principal basé sur Tesseract.
///
/// Cette structure encapsule un moteur Tesseract configuré
/// et fournit des méthodes pour extraire du texte depuis des images.
///
/// # Exemple
///
/// ```no_run
/// use text_recognition::ocr::OcrEngine;
/// use text_recognition::config::OcrConfig;
///
/// // La méthode new() sera implémentée dans la tâche 1.8
/// // let config = OcrConfig::default();
/// // let mut engine = OcrEngine::new(config).expect("Échec initialisation OCR");
/// ```
#[derive(Debug)]
#[allow(dead_code)]
pub struct OcrEngine {
    /// Configuration du moteur OCR.
    config: OcrConfig,
}
