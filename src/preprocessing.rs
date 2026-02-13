//! Module de prétraitement d'images pour améliorer la qualité OCR.
//!
//! Ce module fournit des fonctions pour préparer les images avant l'extraction de texte
//! avec Tesseract. Les opérations de prétraitement incluent :
//!
//! - Conversion en niveaux de gris
//! - Binarisation (Otsu, seuil fixe, adaptative)
//! - Ajustement de contraste
//! - Débruitage
//! - Correction de l'inclinaison (deskew)
//!
//! # Exemple
//!
//! ```no_run
//! use text_recognition::preprocessing::{PreprocessingConfig, preprocess_image};
//! use image::open;
//!
//! let img = open("input.png").unwrap();
//! let config = PreprocessingConfig::default();
//! let preprocessed = preprocess_image(&img, &config);
//! ```

use anyhow::Result;
use image::{DynamicImage, GrayImage};

/// Configuration pour le prétraitement d'images.
///
/// Cette structure définit les paramètres à appliquer lors du prétraitement
/// d'une image avant l'OCR.
#[derive(Debug, Clone)]
pub struct PreprocessingConfig {
    /// Active la conversion en niveaux de gris
    pub to_grayscale: bool,

    /// Active la binarisation
    pub binarize: bool,

    /// Méthode de binarisation à utiliser
    pub binarization_method: BinarizationMethod,

    /// Active l'ajustement de contraste
    pub adjust_contrast: bool,

    /// Facteur de contraste (1.0 = pas de changement, >1.0 = augmentation)
    pub contrast_factor: f32,

    /// Active le débruitage
    pub denoise: bool,

    /// Active la correction de l'inclinaison
    pub deskew: bool,
}

impl Default for PreprocessingConfig {
    fn default() -> Self {
        Self {
            to_grayscale: true,
            binarize: false,
            binarization_method: BinarizationMethod::Otsu,
            adjust_contrast: false,
            contrast_factor: 1.0,
            denoise: false,
            deskew: false,
        }
    }
}

/// Méthode de binarisation pour convertir une image en noir et blanc.
///
/// La binarisation transforme chaque pixel en noir ou blanc selon un seuil,
/// ce qui peut améliorer la lisibilité du texte pour l'OCR.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BinarizationMethod {
    /// Méthode d'Otsu - calcul automatique du seuil optimal
    Otsu,

    /// Seuil fixe (valeur entre 0 et 255)
    Fixed(u8),

    /// Binarisation adaptative - seuil calculé localement
    Adaptive,
}

/// Applique un pipeline de prétraitement complet à une image.
///
/// Cette fonction est le point d'entrée principal pour préparer une image
/// avant l'OCR. Elle applique les transformations dans l'ordre optimal
/// selon la configuration fournie.
///
/// # Arguments
///
/// * `image` - L'image source à prétraiter
/// * `config` - Configuration du prétraitement
///
/// # Exemple
///
/// ```no_run
/// use text_recognition::preprocessing::{PreprocessingConfig, preprocess_image};
/// use image::open;
///
/// let img = open("document.png").unwrap();
/// let config = PreprocessingConfig::default();
/// let result = preprocess_image(&img, &config).unwrap();
/// ```
///
/// # Erreurs
///
/// Retourne une erreur si une étape du prétraitement échoue.
pub fn preprocess_image(
    image: &DynamicImage,
    config: &PreprocessingConfig,
) -> Result<DynamicImage> {
    let mut img = image.clone();

    // Conversion en niveaux de gris
    if config.to_grayscale {
        img = DynamicImage::ImageLuma8(to_grayscale(&img));
    }

    // Les autres étapes seront ajoutées dans les tâches suivantes

    Ok(img)
}

/// Convertit une image en niveaux de gris.
///
/// Cette conversion simplifie l'image en conservant uniquement l'information
/// de luminance, ce qui réduit le bruit de couleur et améliore la performance OCR.
///
/// # Arguments
///
/// * `image` - L'image source à convertir
///
/// # Exemple
///
/// ```no_run
/// use text_recognition::preprocessing::to_grayscale;
/// use image::open;
///
/// let img = open("color_document.png").unwrap();
/// let gray = to_grayscale(&img);
/// ```
pub fn to_grayscale(image: &DynamicImage) -> GrayImage {
    image.to_luma8()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_preprocessing_config_default() {
        let config = PreprocessingConfig::default();
        assert!(config.to_grayscale);
        assert!(!config.binarize);
        assert_eq!(config.binarization_method, BinarizationMethod::Otsu);
        assert!(!config.adjust_contrast);
        assert_eq!(config.contrast_factor, 1.0);
        assert!(!config.denoise);
        assert!(!config.deskew);
    }

    #[test]
    fn test_binarization_method_equality() {
        assert_eq!(BinarizationMethod::Otsu, BinarizationMethod::Otsu);
        assert_eq!(
            BinarizationMethod::Fixed(128),
            BinarizationMethod::Fixed(128)
        );
        assert_ne!(
            BinarizationMethod::Fixed(100),
            BinarizationMethod::Fixed(128)
        );
        assert_eq!(BinarizationMethod::Adaptive, BinarizationMethod::Adaptive);
    }
}
