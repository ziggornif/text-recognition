//! Module de configuration pour Tesseract OCR.
//!
//! Ce module fournit les structures et méthodes pour configurer
//! le moteur OCR avec différents paramètres et modes de segmentation.

/// Mode de segmentation de page (Page Segmentation Mode).
///
/// Tesseract propose 14 modes différents pour segmenter et analyser une image.
/// Chaque mode est optimisé pour un type de contenu spécifique.
///
/// # Modes principaux
///
/// - `Auto` : Détection automatique (recommandé pour débuter)
/// - `SingleBlock` : Bloc de texte unique (documents simples)
/// - `SingleLine` : Ligne de texte unique (captures d'écran, OCR de champs)
/// - `SingleWord` : Mot unique
/// - `SingleChar` : Caractère unique
///
/// # Exemple
///
/// ```
/// use text_recognition::config::PageSegMode;
///
/// let mode = PageSegMode::Auto;
/// let line_mode = PageSegMode::SingleLine;
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PageSegMode {
    /// PSM 0: Orientation et détection de script uniquement.
    OsdOnly,

    /// PSM 1: Segmentation automatique avec détection d'orientation et de script.
    AutoOsd,

    /// PSM 2: Segmentation automatique sans détection d'orientation et de script.
    AutoOnly,

    /// PSM 3: Segmentation automatique complète (mode par défaut).
    Auto,

    /// PSM 4: Suppose une seule colonne de texte de tailles variables.
    SingleColumn,

    /// PSM 5: Suppose un seul bloc vertical de texte aligné.
    SingleBlockVertText,

    /// PSM 6: Suppose un seul bloc uniforme de texte.
    SingleBlock,

    /// PSM 7: Traite l'image comme une seule ligne de texte.
    SingleLine,

    /// PSM 8: Traite l'image comme un seul mot.
    SingleWord,

    /// PSM 9: Traite l'image comme un seul mot dans un cercle.
    CircleWord,

    /// PSM 10: Traite l'image comme un seul caractère.
    SingleChar,

    /// PSM 11: Texte épars - trouve autant de texte que possible sans ordre particulier.
    SparseText,

    /// PSM 12: Texte épars avec détection d'orientation et de script.
    SparseTextOsd,

    /// PSM 13: Ligne brute - traite l'image comme une seule ligne, sans hacks spécifiques à Tesseract.
    RawLine,
}

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
/// // Utiliser la configuration par défaut
/// let config = OcrConfig::default();
///
/// // Ou créer une configuration personnalisée
/// let custom_config = OcrConfig {
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

impl Default for OcrConfig {
    /// Crée une configuration OCR par défaut.
    ///
    /// # Valeurs par défaut
    ///
    /// - `language`: "eng" (anglais)
    /// - `dpi`: 300 (résolution standard pour documents scannés)
    ///
    /// # Exemple
    ///
    /// ```
    /// use text_recognition::config::OcrConfig;
    ///
    /// let config = OcrConfig::default();
    /// assert_eq!(config.language, "eng");
    /// assert_eq!(config.dpi, 300);
    /// ```
    fn default() -> Self {
        Self {
            language: "eng".to_string(),
            dpi: 300,
        }
    }
}
