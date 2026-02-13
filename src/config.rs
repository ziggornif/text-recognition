//! Module de configuration pour Tesseract OCR.
//!
//! Ce module fournit les structures et méthodes pour configurer
//! le moteur OCR avec différents paramètres et modes de segmentation.

/// Configuration pour le moteur OCR.
///
/// Cette structure contient tous les paramètres nécessaires pour
/// configurer le comportement de Tesseract lors de l'extraction de texte.
///
/// # Exemple
///
/// ```
/// use text_recognition::config::OcrConfig;
///
/// let config = OcrConfig {
///     language: "eng".to_string(),
///     dpi: 300,
/// };
/// ```
#[derive(Debug, Clone)]
pub struct OcrConfig {
    /// Langue utilisée pour l'OCR (ex: "eng", "fra", "eng+fra").
    pub language: String,

    /// Résolution DPI de l'image (points par pouce).
    /// Une valeur typique est 300 DPI pour des documents scannés.
    pub dpi: u32,
}
