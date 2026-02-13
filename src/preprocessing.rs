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

    // Ajustement de contraste (doit être fait avant la binarisation)
    if config.adjust_contrast {
        let gray = img.to_luma8();
        let contrasted = adjust_contrast(&gray, config.contrast_factor);
        img = DynamicImage::ImageLuma8(contrasted);
    }

    // Binarisation
    if config.binarize {
        let gray = img.to_luma8();
        let binary = binarize(&gray, config.binarization_method);
        img = DynamicImage::ImageLuma8(binary);
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

/// Ajuste le contraste d'une image en niveaux de gris.
///
/// Cette fonction applique une transformation linéaire aux valeurs des pixels
/// pour augmenter ou diminuer le contraste. Un facteur > 1.0 augmente le contraste,
/// tandis qu'un facteur < 1.0 le diminue.
///
/// La formule utilisée est : `new_value = ((old_value - 128) * factor) + 128`
/// où 128 est la valeur pivot (gris moyen).
///
/// # Arguments
///
/// * `image` - L'image en niveaux de gris à traiter
/// * `factor` - Le facteur de contraste (recommandé: 0.5 à 3.0)
///   - 1.0 = pas de changement
///   - > 1.0 = augmentation du contraste
///   - < 1.0 = diminution du contraste
///
/// # Exemple
///
/// ```no_run
/// use text_recognition::preprocessing::{to_grayscale, adjust_contrast};
/// use image::open;
///
/// let img = open("low_contrast.png").unwrap();
/// let gray = to_grayscale(&img);
/// let enhanced = adjust_contrast(&gray, 1.5); // Augmente le contraste de 50%
/// ```
pub fn adjust_contrast(image: &GrayImage, factor: f32) -> GrayImage {
    let mut output = image.clone();

    for pixel in output.pixels_mut() {
        let value = pixel[0] as f32;
        // Appliquer la transformation de contraste autour du point pivot (128)
        let new_value = ((value - 128.0) * factor) + 128.0;
        // Clamper entre 0 et 255
        pixel[0] = new_value.clamp(0.0, 255.0) as u8;
    }

    output
}

/// Binarise une image en niveaux de gris en noir et blanc pur.
///
/// Cette fonction convertit chaque pixel en noir (0) ou blanc (255) selon
/// la méthode de binarisation spécifiée. La binarisation peut améliorer
/// la qualité OCR en éliminant les variations de gris intermédiaires.
///
/// # Arguments
///
/// * `image` - L'image en niveaux de gris à binariser
/// * `method` - La méthode de binarisation à utiliser
///
/// # Exemple
///
/// ```no_run
/// use text_recognition::preprocessing::{to_grayscale, binarize, BinarizationMethod};
/// use image::open;
///
/// let img = open("document.png").unwrap();
/// let gray = to_grayscale(&img);
/// let binary = binarize(&gray, BinarizationMethod::Otsu);
/// ```
pub fn binarize(image: &GrayImage, method: BinarizationMethod) -> GrayImage {
    match method {
        BinarizationMethod::Otsu => binarize_otsu(image),
        BinarizationMethod::Fixed(threshold) => binarize_fixed(image, threshold),
        BinarizationMethod::Adaptive => binarize_adaptive(image),
    }
}

/// Calcule le seuil optimal avec la méthode d'Otsu.
///
/// La méthode d'Otsu calcule automatiquement le seuil optimal en maximisant
/// la variance inter-classe entre les pixels noirs et blancs.
///
/// # Arguments
///
/// * `image` - L'image en niveaux de gris
///
/// # Retour
///
/// Le seuil optimal (valeur entre 0 et 255)
fn calculate_otsu_threshold(image: &GrayImage) -> u8 {
    // Calculer l'histogramme
    let mut histogram = [0u32; 256];
    for pixel in image.pixels() {
        histogram[pixel[0] as usize] += 1;
    }

    let total_pixels = (image.width() * image.height()) as f64;

    // Calculer la somme totale pondérée
    let mut sum_total = 0.0;
    for (i, &count) in histogram.iter().enumerate() {
        sum_total += i as f64 * count as f64;
    }

    let mut sum_background = 0.0;
    let mut weight_background = 0.0;
    let mut max_variance = 0.0;
    let mut threshold = 0u8;

    // Tester tous les seuils possibles
    for (t, &count) in histogram.iter().enumerate() {
        weight_background += count as f64;

        if weight_background == 0.0 {
            continue;
        }

        let weight_foreground = total_pixels - weight_background;

        if weight_foreground == 0.0 {
            break;
        }

        sum_background += t as f64 * count as f64;

        let mean_background = sum_background / weight_background;
        let mean_foreground = (sum_total - sum_background) / weight_foreground;

        // Calculer la variance inter-classe
        let variance =
            weight_background * weight_foreground * (mean_background - mean_foreground).powi(2);

        if variance > max_variance {
            max_variance = variance;
            threshold = t as u8;
        }
    }

    threshold
}

/// Binarise une image avec la méthode d'Otsu.
///
/// Cette fonction calcule automatiquement le seuil optimal et binarise l'image.
///
/// # Arguments
///
/// * `image` - L'image en niveaux de gris à binariser
fn binarize_otsu(image: &GrayImage) -> GrayImage {
    let threshold = calculate_otsu_threshold(image);
    binarize_fixed(image, threshold)
}

/// Binarise une image avec un seuil fixe.
///
/// Pixels >= threshold deviennent blancs (255), les autres deviennent noirs (0).
///
/// # Arguments
///
/// * `image` - L'image en niveaux de gris à binariser
/// * `threshold` - Le seuil de binarisation (0-255)
fn binarize_fixed(image: &GrayImage, threshold: u8) -> GrayImage {
    let mut output = image.clone();

    for pixel in output.pixels_mut() {
        pixel[0] = if pixel[0] >= threshold { 255 } else { 0 };
    }

    output
}

/// Binarise une image avec une méthode adaptative.
///
/// La méthode adaptative calcule un seuil local pour chaque pixel en fonction
/// de son voisinage, ce qui est utile pour les images avec un éclairage non uniforme.
///
/// Cette implémentation utilise une fenêtre glissante de 15x15 pixels et calcule
/// la moyenne locale comme seuil. Un pixel devient blanc si sa valeur est supérieure
/// à la moyenne locale moins une constante (C=10).
///
/// # Arguments
///
/// * `image` - L'image en niveaux de gris à binariser
fn binarize_adaptive(image: &GrayImage) -> GrayImage {
    const WINDOW_SIZE: u32 = 15;
    const C: i32 = 10; // Constante à soustraire de la moyenne

    let (width, height) = image.dimensions();
    let mut output = GrayImage::new(width, height);

    let half_window = WINDOW_SIZE / 2;

    for y in 0..height {
        for x in 0..width {
            // Calculer les limites de la fenêtre
            let x_start = x.saturating_sub(half_window);
            let x_end = (x + half_window + 1).min(width);
            let y_start = y.saturating_sub(half_window);
            let y_end = (y + half_window + 1).min(height);

            // Calculer la moyenne locale
            let mut sum = 0u32;
            let mut count = 0u32;

            for wy in y_start..y_end {
                for wx in x_start..x_end {
                    sum += image.get_pixel(wx, wy)[0] as u32;
                    count += 1;
                }
            }

            let mean = (sum / count) as i32;
            let threshold = (mean - C).max(0) as u8;

            // Appliquer le seuil local
            let pixel_value = image.get_pixel(x, y)[0];
            output.put_pixel(
                x,
                y,
                image::Luma([if pixel_value >= threshold { 255 } else { 0 }]),
            );
        }
    }

    output
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

    #[test]
    fn test_binarize_fixed() {
        use image::Luma;

        // Créer une image de test 2x2
        let mut img = GrayImage::new(2, 2);
        img.put_pixel(0, 0, Luma([100]));
        img.put_pixel(0, 1, Luma([150]));
        img.put_pixel(1, 0, Luma([200]));
        img.put_pixel(1, 1, Luma([50]));

        // Binariser avec seuil 128
        let binary = binarize(&img, BinarizationMethod::Fixed(128));

        assert_eq!(binary.get_pixel(0, 0)[0], 0); // 100 < 128 -> 0
        assert_eq!(binary.get_pixel(0, 1)[0], 255); // 150 >= 128 -> 255
        assert_eq!(binary.get_pixel(1, 0)[0], 255); // 200 >= 128 -> 255
        assert_eq!(binary.get_pixel(1, 1)[0], 0); // 50 < 128 -> 0
    }

    #[test]
    fn test_calculate_otsu_threshold() {
        use image::Luma;

        // Créer une image bimodale simple (fond clair, texte sombre)
        let mut img = GrayImage::new(10, 10);
        for y in 0..10 {
            for x in 0..10 {
                // Zone sombre (0-50) et zone claire (200-255)
                let value = if x < 5 { 30 } else { 220 };
                img.put_pixel(x, y, Luma([value]));
            }
        }

        let threshold = calculate_otsu_threshold(&img);

        // Le seuil devrait séparer correctement les deux groupes
        // Il devrait être entre les deux pics (de 30 à 220)
        assert!(threshold >= 30, "Threshold {} should be >= 30", threshold);
        assert!(threshold <= 220, "Threshold {} should be <= 220", threshold);
    }

    #[test]
    fn test_binarize_otsu() {
        use image::Luma;

        // Créer une image avec deux niveaux distincts
        let mut img = GrayImage::new(4, 4);
        for y in 0..4 {
            for x in 0..4 {
                let value = if (x + y) % 2 == 0 { 50 } else { 200 };
                img.put_pixel(x, y, Luma([value]));
            }
        }

        let binary = binarize(&img, BinarizationMethod::Otsu);

        // Tous les pixels devraient être soit 0 soit 255
        for pixel in binary.pixels() {
            assert!(
                pixel[0] == 0 || pixel[0] == 255,
                "Pixel value should be 0 or 255, got {}",
                pixel[0]
            );
        }
    }

    #[test]
    fn test_binarize_adaptive() {
        use image::Luma;

        // Créer une image avec éclairage non uniforme (gradient)
        let mut img = GrayImage::new(20, 20);
        for y in 0..20 {
            for x in 0..20 {
                // Gradient de gauche (sombre) à droite (clair)
                // Avec un pattern de texte (alternance)
                let base = 50 + (x * 10); // Gradient 50 -> 240
                let text_offset = if (x + y) % 3 == 0 { 0 } else { 40 };
                let value = (base + text_offset).min(255) as u8;
                img.put_pixel(x, y, Luma([value]));
            }
        }

        let binary = binarize(&img, BinarizationMethod::Adaptive);

        // Tous les pixels devraient être soit 0 soit 255
        for pixel in binary.pixels() {
            assert!(
                pixel[0] == 0 || pixel[0] == 255,
                "Pixel value should be 0 or 255, got {}",
                pixel[0]
            );
        }

        // Vérifier qu'il y a bien un mélange de pixels noirs et blancs
        let mut black_count = 0;
        let mut white_count = 0;
        for pixel in binary.pixels() {
            if pixel[0] == 0 {
                black_count += 1;
            } else {
                white_count += 1;
            }
        }

        assert!(black_count > 0, "Should have some black pixels");
        assert!(white_count > 0, "Should have some white pixels");
    }

    #[test]
    fn test_adjust_contrast_no_change() {
        use image::Luma;

        // Créer une image de test
        let mut img = GrayImage::new(2, 2);
        img.put_pixel(0, 0, Luma([50]));
        img.put_pixel(0, 1, Luma([128]));
        img.put_pixel(1, 0, Luma([200]));
        img.put_pixel(1, 1, Luma([100]));

        // Appliquer un facteur de 1.0 (pas de changement)
        let result = adjust_contrast(&img, 1.0);

        // Les valeurs devraient être identiques
        assert_eq!(result.get_pixel(0, 0)[0], 50);
        assert_eq!(result.get_pixel(0, 1)[0], 128);
        assert_eq!(result.get_pixel(1, 0)[0], 200);
        assert_eq!(result.get_pixel(1, 1)[0], 100);
    }

    #[test]
    fn test_adjust_contrast_increase() {
        use image::Luma;

        // Créer une image avec du gris moyen
        let mut img = GrayImage::new(2, 2);
        img.put_pixel(0, 0, Luma([100])); // Plus sombre que 128
        img.put_pixel(0, 1, Luma([128])); // Point pivot
        img.put_pixel(1, 0, Luma([150])); // Plus clair que 128
        img.put_pixel(1, 1, Luma([180]));

        // Augmenter le contraste (facteur > 1.0)
        let result = adjust_contrast(&img, 2.0);

        // Les valeurs sombres devraient être plus sombres
        assert!(
            result.get_pixel(0, 0)[0] < 100,
            "Dark pixel should become darker"
        );

        // Le point pivot devrait rester à 128
        assert_eq!(result.get_pixel(0, 1)[0], 128);

        // Les valeurs claires devraient être plus claires
        assert!(
            result.get_pixel(1, 0)[0] > 150,
            "Bright pixel should become brighter"
        );
        assert!(
            result.get_pixel(1, 1)[0] > 180,
            "Bright pixel should become brighter"
        );
    }

    #[test]
    fn test_adjust_contrast_decrease() {
        use image::Luma;

        // Créer une image avec des valeurs contrastées
        let mut img = GrayImage::new(2, 2);
        img.put_pixel(0, 0, Luma([50])); // Très sombre
        img.put_pixel(0, 1, Luma([200])); // Très clair

        // Diminuer le contraste (facteur < 1.0)
        let result = adjust_contrast(&img, 0.5);

        // Les valeurs devraient se rapprocher de 128
        assert!(
            result.get_pixel(0, 0)[0] > 50,
            "Dark pixel should become lighter"
        );
        assert!(
            result.get_pixel(0, 1)[0] < 200,
            "Bright pixel should become darker"
        );
    }

    #[test]
    fn test_adjust_contrast_clamping() {
        use image::Luma;

        // Créer une image avec des valeurs extrêmes
        let mut img = GrayImage::new(2, 2);
        img.put_pixel(0, 0, Luma([10])); // Très sombre
        img.put_pixel(0, 1, Luma([240])); // Très clair

        // Augmenter fortement le contraste
        let result = adjust_contrast(&img, 3.0);

        // Avec facteur 3.0:
        // Pixel 0,0: ((10 - 128) * 3.0) + 128 = -354 + 128 = -226 -> clamped to 0
        // Pixel 0,1: ((240 - 128) * 3.0) + 128 = 336 + 128 = 464 -> clamped to 255
        assert_eq!(
            result.get_pixel(0, 0)[0],
            0,
            "Very dark pixel with high contrast should clamp to 0"
        );
        assert_eq!(
            result.get_pixel(0, 1)[0],
            255,
            "Very bright pixel with high contrast should clamp to 255"
        );
    }
}
