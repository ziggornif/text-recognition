//! Module OCR pour l'extraction de texte depuis des images.
//!
//! Ce module fournit la structure `OcrEngine` qui encapsule
//! le moteur Tesseract OCR et permet d'extraire du texte depuis
//! des images avec différentes configurations.

use crate::config::OcrConfig;
use crate::preprocessing::{PreprocessingConfig, preprocess_image};
use anyhow::{Context, Result};
use image::DynamicImage;
use std::path::Path;
use std::process::Command;

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
    config: OcrConfig,
    /// Configuration optionnelle du prétraitement d'images.
    preprocessing_config: Option<PreprocessingConfig>,
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
        Ok(Self {
            config,
            preprocessing_config: None,
        })
    }

    /// Crée un nouveau moteur OCR avec configuration de prétraitement.
    ///
    /// Cette méthode permet d'activer le prétraitement d'images avant l'OCR,
    /// ce qui peut améliorer significativement la qualité des résultats.
    ///
    /// # Arguments
    ///
    /// * `config` - Configuration du moteur OCR
    /// * `preprocessing_config` - Configuration du prétraitement d'images
    ///
    /// # Exemple
    ///
    /// ```no_run
    /// use text_recognition::ocr::OcrEngine;
    /// use text_recognition::config::OcrConfig;
    /// use text_recognition::preprocessing::{PreprocessingConfig, BinarizationMethod};
    ///
    /// let ocr_config = OcrConfig::default();
    /// let mut preprocess_config = PreprocessingConfig::default();
    /// preprocess_config.binarize = true;
    /// preprocess_config.binarization_method = BinarizationMethod::Otsu;
    ///
    /// let engine = OcrEngine::with_preprocessing(ocr_config, preprocess_config)
    ///     .expect("Échec initialisation OCR");
    /// ```
    pub fn with_preprocessing(
        config: OcrConfig,
        preprocessing_config: PreprocessingConfig,
    ) -> Result<Self> {
        Ok(Self {
            config,
            preprocessing_config: Some(preprocessing_config),
        })
    }

    /// Détecte l'orientation et le script d'une image via le binaire Tesseract (PSM 0).
    ///
    /// Cette méthode appelle le binaire `tesseract` en ligne de commande avec `--psm 0`
    /// pour obtenir les informations d'orientation et de script sans effectuer d'OCR.
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
    ///
    /// let config = OcrConfig {
    ///     page_seg_mode: PageSegMode::OsdOnly,
    ///     ..OcrConfig::default()
    /// };
    /// let engine = OcrEngine::new(config).expect("Échec initialisation OCR");
    /// let result = engine.detect_orientation(Path::new("image.png")).unwrap();
    /// println!("{}", result);
    /// ```
    ///
    /// # Erreurs
    ///
    /// Retourne une erreur si :
    /// - Le binaire `tesseract` n'est pas installé ou introuvable
    /// - Le fichier image n'existe pas ou est illisible
    /// - La détection échoue (image trop petite, format non supporté, etc.)
    pub fn detect_orientation(&self, path: &Path) -> Result<String> {
        let path_str = path.to_str().context("Chemin invalide")?;

        let output = Command::new("tesseract")
            // OSD requiert obligatoirement le modèle "osd", indépendamment de la langue configurée.
            // Utiliser une autre langue (ex: "fra") échouerait avec une erreur Tesseract.
            .args([path_str, "stdout", "--psm", "0", "-l", "osd"])
            .output()
            .context(
                "Impossible de lancer le binaire tesseract. Est-il installé et dans le PATH ?",
            )?;

        // La sortie utile est sur stdout ; les warnings vont sur stderr
        let stdout = String::from_utf8_lossy(&output.stdout).to_string();

        // Filtrer les lignes pertinentes (ignorer les lignes vides)
        let info: String = stdout
            .lines()
            .filter(|line| !line.trim().is_empty())
            .collect::<Vec<_>>()
            .join("\n");

        if info.is_empty() {
            anyhow::bail!(
                "Aucune information d'orientation retournée par Tesseract. \
                 Assurez-vous que l'image est lisible et que les données linguistiques sont installées."
            );
        }

        Ok(info)
    }

    /// Extrait le texte d'une image.
    ///
    /// Cette méthode charge une image depuis un fichier et utilise Tesseract
    /// pour extraire son contenu textuel. Elle applique automatiquement toutes
    /// les variables de configuration Tesseract définies dans `OcrConfig`.
    ///
    /// En mode `OsdOnly` (PSM 0), délègue vers [`detect_orientation()`](Self::detect_orientation)
    /// et retourne les informations d'orientation et de script.
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

        // En mode OSD uniquement, déléguer vers detect_orientation()
        if matches!(
            self.config.page_seg_mode,
            crate::config::PageSegMode::OsdOnly
        ) {
            return self.detect_orientation(path);
        }

        // Si le prétraitement est activé, charger et prétraiter l'image
        if let Some(ref preprocess_config) = self.preprocessing_config {
            let img = image::open(path)
                .with_context(|| format!("Échec du chargement de l'image '{}'", path.display()))?;

            let preprocessed = preprocess_image(&img, preprocess_config)
                .context("Échec du prétraitement de l'image")?;

            return self.extract_text_from_image(&preprocessed);
        }

        // Sinon, utiliser directement le chemin du fichier
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

    /// Extrait le texte d'une image en mémoire.
    ///
    /// Cette méthode prend une `DynamicImage` et utilise Tesseract pour
    /// extraire son contenu textuel. Utile lorsque l'image a déjà été
    /// chargée ou prétraitée en mémoire.
    ///
    /// # Arguments
    ///
    /// * `image` - L'image à analyser
    ///
    /// # Exemple
    ///
    /// ```no_run
    /// use text_recognition::ocr::OcrEngine;
    /// use text_recognition::config::OcrConfig;
    /// use image::open;
    ///
    /// let config = OcrConfig::default();
    /// let engine = OcrEngine::new(config)?;
    ///
    /// let img = open("document.png")?;
    /// let text = engine.extract_text_from_image(&img)?;
    /// println!("Texte: {}", text);
    /// # Ok::<(), anyhow::Error>(())
    /// ```
    ///
    /// # Erreurs
    ///
    /// Retourne une erreur si :
    /// - Tesseract échoue lors de l'extraction
    /// - L'image ne peut pas être convertie dans un format compatible
    /// - Une variable Tesseract invalide est définie
    pub fn extract_text_from_image(&self, image: &DynamicImage) -> Result<String> {
        // Sauvegarder temporairement l'image pour Tesseract
        // (Tesseract nécessite un chemin de fichier)
        let temp_dir = tempfile::tempdir().context("Échec de création du répertoire temporaire")?;
        let temp_path = temp_dir.path().join("temp_image.png");

        image
            .save(&temp_path)
            .context("Échec de la sauvegarde de l'image temporaire")?;

        let path_str = temp_path.to_str().context("Chemin temporaire invalide")?;

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
