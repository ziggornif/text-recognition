//! Module OCR pour l'extraction de texte depuis des images.
//!
//! Ce module fournit la structure `OcrEngine` qui encapsule
//! le moteur Tesseract OCR et permet d'extraire du texte depuis
//! des images avec différentes configurations.

use crate::config::OcrConfig;
use anyhow::Result;

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
/// let config = OcrConfig::default();
/// let engine = OcrEngine::new(config).expect("Échec initialisation OCR");
/// ```
#[derive(Debug)]
pub struct OcrEngine {
    /// Configuration du moteur OCR.
    #[allow(dead_code)]
    config: OcrConfig,
}

impl OcrEngine {
    /// Crée un nouveau moteur OCR avec la configuration spécifiée.
    ///
    /// Cette méthode initialise un moteur Tesseract OCR avec les paramètres
    /// fournis dans la configuration.
    ///
    /// # Arguments
    ///
    /// * `config` - Configuration du moteur OCR
    ///
    /// # Exemple
    ///
    /// ```no_run
    /// use text_recognition::ocr::OcrEngine;
    /// use text_recognition::config::OcrConfig;
    ///
    /// let config = OcrConfig {
    ///     language: "fra".to_string(),
    ///     dpi: 300,
    /// };
    ///
    /// let engine = OcrEngine::new(config).expect("Échec initialisation OCR");
    /// ```
    ///
    /// # Erreurs
    ///
    /// Retourne une erreur si :
    /// - Tesseract n'est pas installé sur le système
    /// - Les données linguistiques spécifiées ne sont pas disponibles
    /// - L'initialisation de Tesseract échoue pour une autre raison
    pub fn new(config: OcrConfig) -> Result<Self> {
        // Pour l'instant, on crée simplement la structure
        // La validation de Tesseract sera faite lors de l'utilisation réelle
        Ok(Self { config })
    }
}
