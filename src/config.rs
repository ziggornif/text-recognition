//! Module de configuration pour Tesseract OCR.
//!
//! Ce module fournit les structures et méthodes pour configurer
//! le moteur OCR avec différents paramètres et modes de segmentation.

use serde::{Deserialize, Serialize};
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
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
/// use text_recognition::config::{OcrConfig, PageSegMode};
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
///     page_seg_mode: PageSegMode::SingleBlock,
///     dpi: 300,
///     tesseract_variables: variables,
/// };
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OcrConfig {
    /// Langue utilisée pour l'OCR (ex: "eng", "fra", "eng+fra").
    pub language: String,

    /// Mode de segmentation de page.
    ///
    /// Détermine comment Tesseract analyse et segmente l'image.
    /// Le mode par défaut est `PageSegMode::Auto` (mode 3).
    pub page_seg_mode: PageSegMode,

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
    /// - `page_seg_mode`: `PageSegMode::Auto` (détection automatique)
    /// - `dpi`: 300 (résolution standard pour documents scannés)
    /// - `tesseract_variables`: HashMap vide (aucune variable personnalisée)
    ///
    /// # Exemple
    ///
    /// ```
    /// use text_recognition::config::{OcrConfig, PageSegMode};
    ///
    /// let config = OcrConfig::default();
    /// assert_eq!(config.language, "fra");
    /// assert_eq!(config.page_seg_mode, PageSegMode::Auto);
    /// assert_eq!(config.dpi, 300);
    /// assert!(config.tesseract_variables.is_empty());
    /// ```
    fn default() -> Self {
        Self {
            language: "fra".to_string(),
            page_seg_mode: PageSegMode::Auto,
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
            page_seg_mode: PageSegMode::Auto,
            dpi: 300,
            tesseract_variables: variables,
        }
    }

    /// Crée une configuration préréglée optimisée pour les captures d'écran.
    ///
    /// Ce preset est idéal pour :
    /// - Captures d'écran de texte
    /// - Interface utilisateur d'applications
    /// - Contenu web capturé
    /// - Fenêtres de dialogue et messages
    ///
    /// # Configuration appliquée
    ///
    /// - **DPI** : 96 (résolution standard pour écrans)
    /// - **Variables Tesseract** :
    ///   - Aucune variable spécifique (utilise les paramètres par défaut)
    ///
    /// # Mode PSM recommandé
    ///
    /// Pour utiliser ce preset efficacement, combinez-le avec :
    /// - `PageSegMode::Auto` (mode 3) : Détection automatique (par défaut)
    /// - `PageSegMode::SparseText` (mode 11) : Pour texte épars ou dispersé
    /// - `PageSegMode::SingleBlock` (mode 6) : Pour bloc de texte unique
    ///
    /// # Exemple
    ///
    /// ```
    /// use text_recognition::config::OcrConfig;
    ///
    /// // Créer un preset pour captures d'écran
    /// let config = OcrConfig::screenshot_preset();
    /// assert_eq!(config.language, "fra");
    /// assert_eq!(config.dpi, 96);
    /// ```
    ///
    /// Pour utiliser ce preset avec un moteur OCR :
    ///
    /// ```no_run
    /// use text_recognition::config::OcrConfig;
    /// use text_recognition::ocr::OcrEngine;
    ///
    /// let config = OcrConfig::screenshot_preset();
    /// let mut engine = OcrEngine::new(config)?;
    /// # Ok::<(), anyhow::Error>(())
    /// ```
    pub fn screenshot_preset() -> Self {
        Self {
            language: "fra".to_string(),
            page_seg_mode: PageSegMode::Auto,
            dpi: 96,
            tesseract_variables: HashMap::new(),
        }
    }

    /// Crée une configuration préréglée optimisée pour les lignes de texte uniques.
    ///
    /// Ce preset est idéal pour :
    /// - Champs de formulaire
    /// - Titres et en-têtes
    /// - Lignes de tableau
    /// - Labels et étiquettes
    /// - Codes-barres alphanumériques
    ///
    /// # Configuration appliquée
    ///
    /// - **DPI** : 150 (résolution intermédiaire)
    /// - **Variables Tesseract** :
    ///   - Aucune variable spécifique (utilise les paramètres par défaut)
    ///
    /// # Mode PSM recommandé
    ///
    /// Pour utiliser ce preset efficacement, combinez-le avec :
    /// - `PageSegMode::SingleLine` (mode 7) : Traite l'image comme une seule ligne (recommandé)
    /// - `PageSegMode::RawLine` (mode 13) : Ligne brute sans hacks spécifiques
    ///
    /// # Exemple
    ///
    /// ```
    /// use text_recognition::config::OcrConfig;
    ///
    /// // Créer un preset pour ligne de texte unique
    /// let config = OcrConfig::single_line_preset();
    /// assert_eq!(config.language, "fra");
    /// assert_eq!(config.dpi, 150);
    /// ```
    ///
    /// Pour utiliser ce preset avec un moteur OCR :
    ///
    /// ```no_run
    /// use text_recognition::config::OcrConfig;
    /// use text_recognition::ocr::OcrEngine;
    ///
    /// let config = OcrConfig::single_line_preset();
    /// let mut engine = OcrEngine::new(config)?;
    /// # Ok::<(), anyhow::Error>(())
    /// ```
    pub fn single_line_preset() -> Self {
        Self {
            language: "fra".to_string(),
            page_seg_mode: PageSegMode::SingleLine,
            dpi: 150,
            tesseract_variables: HashMap::new(),
        }
    }

    /// Crée une configuration préréglée optimisée pour les photos de texte.
    ///
    /// Ce preset est idéal pour :
    /// - Photos de documents prises avec smartphone
    /// - Tableaux blancs et présentations photographiées
    /// - Panneaux et enseignes photographiés
    /// - Documents en conditions d'éclairage variable
    /// - Images avec perspective ou légère déformation
    ///
    /// # Configuration appliquée
    ///
    /// - **DPI** : 200 (résolution intermédiaire-haute)
    /// - **Variables Tesseract** :
    ///   - `tessedit_do_invert` : "0" (désactive l'inversion automatique)
    ///
    /// # Mode PSM recommandé
    ///
    /// Pour utiliser ce preset efficacement, combinez-le avec :
    /// - `PageSegMode::Auto` (mode 3) : Détection automatique (recommandé)
    /// - `PageSegMode::AutoOsd` (mode 1) : Avec détection d'orientation et de script
    /// - `PageSegMode::SparseText` (mode 11) : Pour texte dispersé
    ///
    /// # Note
    ///
    /// Les photos de texte bénéficient souvent d'un prétraitement (redressement,
    /// amélioration du contraste, binarisation). Combinez ce preset avec les
    /// fonctions de prétraitement pour de meilleurs résultats.
    ///
    /// # Exemple
    ///
    /// ```
    /// use text_recognition::config::OcrConfig;
    ///
    /// // Créer un preset pour photos de texte
    /// let config = OcrConfig::photo_preset();
    /// assert_eq!(config.language, "fra");
    /// assert_eq!(config.dpi, 200);
    /// ```
    ///
    /// Pour utiliser ce preset avec un moteur OCR :
    ///
    /// ```no_run
    /// use text_recognition::config::OcrConfig;
    /// use text_recognition::ocr::OcrEngine;
    ///
    /// let config = OcrConfig::photo_preset();
    /// let mut engine = OcrEngine::new(config)?;
    /// # Ok::<(), anyhow::Error>(())
    /// ```
    pub fn photo_preset() -> Self {
        let mut variables = HashMap::new();
        // Désactiver l'inversion automatique qui peut causer des problèmes avec les photos
        variables.insert("tessedit_do_invert".to_string(), "0".to_string());

        Self {
            language: "fra".to_string(),
            page_seg_mode: PageSegMode::Auto,
            dpi: 200,
            tesseract_variables: variables,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test de la configuration par défaut.
    #[test]
    fn test_default_config() {
        let config = OcrConfig::default();

        assert_eq!(config.language, "fra");
        assert_eq!(config.page_seg_mode, PageSegMode::Auto);
        assert_eq!(config.dpi, 300);
        assert!(config.tesseract_variables.is_empty());
    }

    /// Test du preset pour documents.
    #[test]
    fn test_document_preset() {
        let config = OcrConfig::document_preset();

        // Vérifier les paramètres de base
        assert_eq!(config.language, "fra");
        assert_eq!(config.page_seg_mode, PageSegMode::Auto);
        assert_eq!(config.dpi, 300);

        // Vérifier les variables Tesseract spécifiques
        assert_eq!(config.tesseract_variables.len(), 1);
        assert_eq!(
            config.tesseract_variables.get("preserve_interword_spaces"),
            Some(&"1".to_string())
        );
    }

    /// Test du preset pour captures d'écran.
    #[test]
    fn test_screenshot_preset() {
        let config = OcrConfig::screenshot_preset();

        // Vérifier les paramètres de base
        assert_eq!(config.language, "fra");
        assert_eq!(config.page_seg_mode, PageSegMode::Auto);
        assert_eq!(config.dpi, 96); // DPI spécifique aux écrans

        // Vérifier qu'aucune variable Tesseract n'est définie
        assert!(config.tesseract_variables.is_empty());
    }

    /// Test du preset pour ligne de texte unique.
    #[test]
    fn test_single_line_preset() {
        let config = OcrConfig::single_line_preset();

        // Vérifier les paramètres de base
        assert_eq!(config.language, "fra");
        assert_eq!(config.page_seg_mode, PageSegMode::SingleLine);
        assert_eq!(config.dpi, 150);

        // Vérifier qu'aucune variable Tesseract n'est définie
        assert!(config.tesseract_variables.is_empty());
    }

    /// Test du preset pour photos de texte.
    #[test]
    fn test_photo_preset() {
        let config = OcrConfig::photo_preset();

        // Vérifier les paramètres de base
        assert_eq!(config.language, "fra");
        assert_eq!(config.page_seg_mode, PageSegMode::Auto);
        assert_eq!(config.dpi, 200);

        // Vérifier les variables Tesseract spécifiques
        assert_eq!(config.tesseract_variables.len(), 1);
        assert_eq!(
            config.tesseract_variables.get("tessedit_do_invert"),
            Some(&"0".to_string())
        );
    }

    /// Test de la conversion PageSegMode vers Tesseract PSM.
    #[test]
    fn test_page_seg_mode_conversion() {
        assert_eq!(PageSegMode::OsdOnly.to_tesseract_psm(), 0);
        assert_eq!(PageSegMode::AutoOsd.to_tesseract_psm(), 1);
        assert_eq!(PageSegMode::AutoOnly.to_tesseract_psm(), 2);
        assert_eq!(PageSegMode::Auto.to_tesseract_psm(), 3);
        assert_eq!(PageSegMode::SingleColumn.to_tesseract_psm(), 4);
        assert_eq!(PageSegMode::SingleBlockVertText.to_tesseract_psm(), 5);
        assert_eq!(PageSegMode::SingleBlock.to_tesseract_psm(), 6);
        assert_eq!(PageSegMode::SingleLine.to_tesseract_psm(), 7);
        assert_eq!(PageSegMode::SingleWord.to_tesseract_psm(), 8);
        assert_eq!(PageSegMode::CircleWord.to_tesseract_psm(), 9);
        assert_eq!(PageSegMode::SingleChar.to_tesseract_psm(), 10);
        assert_eq!(PageSegMode::SparseText.to_tesseract_psm(), 11);
        assert_eq!(PageSegMode::SparseTextOsd.to_tesseract_psm(), 12);
        assert_eq!(PageSegMode::RawLine.to_tesseract_psm(), 13);
    }

    /// Test que chaque preset a des paramètres distincts.
    #[test]
    fn test_presets_are_distinct() {
        let document = OcrConfig::document_preset();
        let screenshot = OcrConfig::screenshot_preset();
        let single_line = OcrConfig::single_line_preset();
        let photo = OcrConfig::photo_preset();

        // Vérifier que les DPI sont différents pour chaque preset
        assert_ne!(document.dpi, screenshot.dpi);
        assert_ne!(document.dpi, single_line.dpi);
        assert_ne!(screenshot.dpi, photo.dpi);

        // Vérifier que les modes PSM sont différents là où attendu
        assert_ne!(single_line.page_seg_mode, document.page_seg_mode);
    }

    /// Test que les presets peuvent être clonés.
    #[test]
    fn test_config_clone() {
        let config1 = OcrConfig::document_preset();
        let config2 = config1.clone();

        assert_eq!(config1.language, config2.language);
        assert_eq!(config1.page_seg_mode, config2.page_seg_mode);
        assert_eq!(config1.dpi, config2.dpi);
    }
}
