//! Module de configuration pour Tesseract OCR.
//!
//! Ce module fournit les structures et méthodes pour configurer
//! le moteur OCR avec différents paramètres et modes de segmentation.

use std::collections::HashMap;

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

impl PageSegMode {
    /// Convertit le mode de segmentation vers le code PSM Tesseract.
    ///
    /// Tesseract utilise des codes numériques pour identifier les modes de segmentation.
    /// Cette méthode retourne le code approprié (0-13) pour le mode actuel.
    ///
    /// # Exemple
    ///
    /// ```
    /// use text_recognition::config::PageSegMode;
    ///
    /// let mode = PageSegMode::Auto;
    /// assert_eq!(mode.to_tesseract_psm(), 3);
    ///
    /// let line_mode = PageSegMode::SingleLine;
    /// assert_eq!(line_mode.to_tesseract_psm(), 7);
    /// ```
    pub fn to_tesseract_psm(self) -> i32 {
        match self {
            PageSegMode::OsdOnly => 0,
            PageSegMode::AutoOsd => 1,
            PageSegMode::AutoOnly => 2,
            PageSegMode::Auto => 3,
            PageSegMode::SingleColumn => 4,
            PageSegMode::SingleBlockVertText => 5,
            PageSegMode::SingleBlock => 6,
            PageSegMode::SingleLine => 7,
            PageSegMode::SingleWord => 8,
            PageSegMode::CircleWord => 9,
            PageSegMode::SingleChar => 10,
            PageSegMode::SparseText => 11,
            PageSegMode::SparseTextOsd => 12,
            PageSegMode::RawLine => 13,
        }
    }
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
/// use std::collections::HashMap;
///
/// // Utiliser la configuration par défaut
/// let config = OcrConfig::default();
///
/// // Ou créer une configuration personnalisée
/// let mut variables = HashMap::new();
/// variables.insert("tessedit_char_whitelist".to_string(), "0123456789".to_string());
///
/// let custom_config = OcrConfig {
///     language: "eng".to_string(),
///     dpi: 300,
///     tesseract_variables: variables,
/// };
/// ```
#[derive(Debug, Clone)]
pub struct OcrConfig {
    /// Langue utilisée pour l'OCR (ex: "eng", "fra", "eng+fra").
    pub language: String,

    /// Résolution DPI de l'image (points par pouce).
    /// Une valeur typique est 300 DPI pour des documents scannés.
    pub dpi: u32,

    /// Variables de configuration Tesseract.
    ///
    /// Permet de personnaliser finement le comportement de Tesseract via des variables.
    /// Exemples de variables courantes :
    /// - `tessedit_char_whitelist`: Caractères autorisés (ex: "0123456789" pour chiffres uniquement)
    /// - `tessedit_char_blacklist`: Caractères interdits
    /// - `preserve_interword_spaces`: Préserver les espaces multiples ("1" = oui, "0" = non)
    pub tesseract_variables: HashMap<String, String>,
}

impl Default for OcrConfig {
    /// Crée une configuration OCR par défaut.
    ///
    /// # Valeurs par défaut
    ///
    /// - `language`: "fra" (français)
    /// - `dpi`: 300 (résolution standard pour documents scannés)
    /// - `tesseract_variables`: HashMap vide (aucune variable personnalisée)
    ///
    /// # Exemple
    ///
    /// ```
    /// use text_recognition::config::OcrConfig;
    ///
    /// let config = OcrConfig::default();
    /// assert_eq!(config.language, "fra");
    /// assert_eq!(config.dpi, 300);
    /// assert!(config.tesseract_variables.is_empty());
    /// ```
    fn default() -> Self {
        Self {
            language: "fra".to_string(),
            dpi: 300,
            tesseract_variables: HashMap::new(),
        }
    }
}

impl OcrConfig {
    /// Crée une configuration préréglée optimisée pour les documents standards.
    ///
    /// Ce preset est idéal pour :
    /// - Documents scannés multi-pages
    /// - Pages de livres
    /// - Articles imprimés
    /// - Documents administratifs
    ///
    /// # Configuration appliquée
    ///
    /// - **DPI** : 300 (résolution standard pour documents scannés)
    /// - **Variables Tesseract** :
    ///   - `preserve_interword_spaces` : "1" (préserve les espaces multiples)
    ///
    /// # Mode PSM recommandé
    ///
    /// Pour utiliser ce preset efficacement, combinez-le avec :
    /// - `PageSegMode::Auto` (mode 3) : Détection automatique de mise en page
    /// - `PageSegMode::SingleColumn` (mode 4) : Pour documents en colonne unique
    ///
    /// # Exemple
    ///
    /// ```
    /// use text_recognition::config::OcrConfig;
    ///
    /// // Créer un preset pour documents
    /// let config = OcrConfig::document_preset();
    /// assert_eq!(config.language, "fra");
    /// assert_eq!(config.dpi, 300);
    /// ```
    ///
    /// Pour utiliser ce preset avec un moteur OCR :
    ///
    /// ```no_run
    /// use text_recognition::config::OcrConfig;
    /// use text_recognition::ocr::OcrEngine;
    ///
    /// let config = OcrConfig::document_preset();
    /// let mut engine = OcrEngine::new(config)?;
    /// # Ok::<(), anyhow::Error>(())
    /// ```
    pub fn document_preset() -> Self {
        let mut variables = HashMap::new();
        // Préserver les espaces multiples pour respecter la mise en page
        variables.insert("preserve_interword_spaces".to_string(), "1".to_string());

        Self {
            language: "fra".to_string(),
            dpi: 300,
            tesseract_variables: variables,
        }
    }
}
