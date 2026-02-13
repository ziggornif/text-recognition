//! Module OCR pour l'extraction de texte depuis des images.
//!
//! Ce module fournit la structure `OcrEngine` qui encapsule
//! le moteur Tesseract OCR et permet d'extraire du texte depuis
//! des images avec différentes configurations.

use crate::config::OcrConfig;
use anyhow::{Context, Result};
use std::path::Path;

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
    /// use text_recognition::config::{OcrConfig, PageSegMode};
    /// use std::collections::HashMap;
    ///
    /// let config = OcrConfig {
    ///     language: "fra".to_string(),
    ///     page_seg_mode: PageSegMode::Auto,
    ///     dpi: 300,
    ///     tesseract_variables: HashMap::new(),
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

    /// Extrait le texte d'une image.
    ///
    /// Cette méthode charge une image depuis un fichier et utilise Tesseract
    /// pour extraire son contenu textuel. Elle applique automatiquement toutes
    /// les variables de configuration Tesseract définies dans `OcrConfig`.
    ///
    /// # Arguments
    ///
    /// * `path` - Chemin vers l'image à analyser
    ///
    /// # Exemple
    ///
    /// ```no_run
    /// use text_recognition::ocr::OcrEngine;
    /// use text_recognition::config::{OcrConfig, PageSegMode};
    /// use std::path::Path;
    /// use std::collections::HashMap;
    ///
    /// let mut variables = HashMap::new();
    /// variables.insert("tessedit_char_whitelist".to_string(), "0123456789".to_string());
    ///
    /// let config = OcrConfig {
    ///     language: "eng".to_string(),
    ///     page_seg_mode: PageSegMode::SingleBlock,
    ///     dpi: 300,
    ///     tesseract_variables: variables,
    /// };
    ///
    /// let engine = OcrEngine::new(config)?;
    /// let text = engine.extract_text_from_file(Path::new("image.png"))?;
    /// println!("Texte extrait: {}", text);
    /// # Ok::<(), anyhow::Error>(())
    /// ```
    ///
    /// # Erreurs
    ///
    /// Retourne une erreur si :
    /// - Le fichier n'existe pas
    /// - L'image est corrompue ou dans un format non supporté
    /// - Tesseract échoue lors de l'extraction
    /// - Les données linguistiques ne sont pas disponibles
    /// - Une variable Tesseract invalide est définie
    pub fn extract_text_from_file(&self, path: &Path) -> Result<String> {
        // Vérifier que le fichier existe
        if !path.exists() {
            anyhow::bail!("Le fichier '{}' n'existe pas", path.display());
        }

        // Convertir le chemin en string
        let path_str = path.to_str().context("Chemin invalide")?;

        // Initialiser Tesseract avec la langue configurée
        let mut tesseract = tesseract::Tesseract::new(None, Some(&self.config.language))
            .context("Échec de l'initialisation de Tesseract")?;

        // Appliquer le mode de segmentation de page
        let psm = match self.config.page_seg_mode {
            crate::config::PageSegMode::OsdOnly => tesseract::PageSegMode::PsmOsdOnly,
            crate::config::PageSegMode::AutoOsd => tesseract::PageSegMode::PsmAutoOsd,
            crate::config::PageSegMode::AutoOnly => tesseract::PageSegMode::PsmAutoOnly,
            crate::config::PageSegMode::Auto => tesseract::PageSegMode::PsmAuto,
            crate::config::PageSegMode::SingleColumn => tesseract::PageSegMode::PsmSingleColumn,
            crate::config::PageSegMode::SingleBlockVertText => {
                tesseract::PageSegMode::PsmSingleBlockVertText
            }
            crate::config::PageSegMode::SingleBlock => tesseract::PageSegMode::PsmSingleBlock,
            crate::config::PageSegMode::SingleLine => tesseract::PageSegMode::PsmSingleLine,
            crate::config::PageSegMode::SingleWord => tesseract::PageSegMode::PsmSingleWord,
            crate::config::PageSegMode::CircleWord => tesseract::PageSegMode::PsmCircleWord,
            crate::config::PageSegMode::SingleChar => tesseract::PageSegMode::PsmSingleChar,
            crate::config::PageSegMode::SparseText => tesseract::PageSegMode::PsmSparseText,
            crate::config::PageSegMode::SparseTextOsd => tesseract::PageSegMode::PsmSparseTextOsd,
            crate::config::PageSegMode::RawLine => tesseract::PageSegMode::PsmRawLine,
        };
        tesseract.set_page_seg_mode(psm);

        // Appliquer le DPI
        tesseract = tesseract
            .set_variable("user_defined_dpi", &self.config.dpi.to_string())
            .context("Échec de la configuration du DPI")?;

        // Appliquer toutes les variables Tesseract personnalisées
        for (key, value) in &self.config.tesseract_variables {
            tesseract = tesseract
                .set_variable(key, value)
                .with_context(|| format!("Échec de la configuration de la variable '{}'", key))?;
        }

        // Charger l'image
        tesseract = tesseract
            .set_image(path_str)
            .context("Échec du chargement de l'image")?;

        // Extraire le texte
        let text = tesseract
            .get_text()
            .context("Échec de l'extraction du texte")?;

        Ok(text)
    }
}
