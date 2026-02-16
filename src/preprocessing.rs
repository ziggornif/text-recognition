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
use image::{DynamicImage, GrayImage, imageops};
use serde::{Deserialize, Serialize};

/// Configuration pour le prétraitement d'images.
///
/// Cette structure définit les paramètres à appliquer lors du prétraitement
/// d'une image avant l'OCR.
#[derive(Debug, Clone, Serialize, Deserialize)]
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
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum BinarizationMethod {
    /// Méthode d'Otsu - calcul automatique du seuil optimal
    Otsu,

    /// Seuil fixe (valeur entre 0 et 255)
    Fixed(u8),

    /// Binarisation adaptative - seuil calculé localement
    Adaptive,
}

/// Orientation d'une image détectée par Tesseract (PSM 0).
///
/// Tesseract retourne l'orientation en degrés dans le sens horaire.
/// Cette enum représente les quatre orientations possibles.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Orientation {
    /// 0° : image droite, aucune correction nécessaire.
    Upright,
    /// 90° dans le sens horaire : le haut de l'image est à droite.
    Clockwise90,
    /// 180° : image à l'envers.
    UpsideDown,
    /// 270° dans le sens horaire (= 90° antihoraire) : le haut est à gauche.
    CounterClockwise90,
}

impl Orientation {
    /// Crée une `Orientation` depuis le code retourné par Tesseract PSM 0.
    ///
    /// Tesseract retourne la valeur `Orientation in degrees` qui correspond
    /// à la rotation à appliquer pour remettre l'image droite.
    ///
    /// # Arguments
    ///
    /// * `degrees` - Valeur en degrés retournée par Tesseract (0, 90, 180 ou 270)
    pub fn from_tesseract_degrees(degrees: u32) -> Self {
        match degrees {
            90 => Orientation::Clockwise90,
            180 => Orientation::UpsideDown,
            270 => Orientation::CounterClockwise90,
            _ => Orientation::Upright,
        }
    }
}

/// Corrige l'orientation d'une image selon l'angle détecté.
///
/// Applique une rotation de 90°, 180° ou 270° pour remettre l'image droite.
/// Utilise les fonctions de rotation sans perte de la bibliothèque `image`.
///
/// # Arguments
///
/// * `image` - L'image à corriger
/// * `orientation` - L'orientation détectée (via Tesseract PSM 0)
///
/// # Exemple
///
/// ```no_run
/// use text_recognition::preprocessing::{Orientation, rotate_orientation};
/// use image::open;
///
/// let img = open("upside_down.png").unwrap();
/// let corrected = rotate_orientation(&img, Orientation::UpsideDown);
/// ```
pub fn rotate_orientation(image: &DynamicImage, orientation: Orientation) -> DynamicImage {
    match orientation {
        Orientation::Upright => image.clone(),
        Orientation::Clockwise90 => DynamicImage::ImageRgba8(imageops::rotate90(&image.to_rgba8())),
        Orientation::UpsideDown => DynamicImage::ImageRgba8(imageops::rotate180(&image.to_rgba8())),
        Orientation::CounterClockwise90 => {
            DynamicImage::ImageRgba8(imageops::rotate270(&image.to_rgba8()))
        }
    }
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

    // Correction de l'inclinaison (deskew - avant les autres traitements)
    if config.deskew {
        let gray = img.to_luma8();
        let deskewed = deskew(&gray);
        img = DynamicImage::ImageLuma8(deskewed);
    }

    // Débruitage (avant ajustement de contraste et binarisation)
    if config.denoise {
        let gray = img.to_luma8();
        let denoised = denoise(&gray);
        img = DynamicImage::ImageLuma8(denoised);
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

    // Pipeline de prétraitement terminé

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

/// Applique un filtre de débruitage à une image en niveaux de gris.
///
/// Cette fonction utilise un filtre médian 3x3 pour réduire le bruit salt-and-pepper
/// (poivre et sel) tout en préservant les contours. Le filtre médian remplace chaque
/// pixel par la valeur médiane de son voisinage.
///
/// Le filtre médian est particulièrement efficace pour :
/// - Réduire le bruit impulsionnel (pixels isolés noirs ou blancs)
/// - Préserver les contours et les détails du texte
/// - Améliorer la qualité avant binarisation
///
/// # Arguments
///
/// * `image` - L'image en niveaux de gris à débruiter
///
/// # Exemple
///
/// ```no_run
/// use text_recognition::preprocessing::{to_grayscale, denoise};
/// use image::open;
///
/// let img = open("noisy_document.png").unwrap();
/// let gray = to_grayscale(&img);
/// let denoised = denoise(&gray);
/// ```
pub fn denoise(image: &GrayImage) -> GrayImage {
    let (width, height) = image.dimensions();
    let mut output = image.clone();

    // Appliquer un filtre médian 3x3
    for y in 1..height - 1 {
        for x in 1..width - 1 {
            // Collecter les valeurs du voisinage 3x3
            let mut neighbors = [0u8; 9];
            let mut idx = 0;

            for dy in 0..3 {
                for dx in 0..3 {
                    neighbors[idx] = image.get_pixel(x + dx - 1, y + dy - 1)[0];
                    idx += 1;
                }
            }

            // Trier et prendre la médiane
            neighbors.sort_unstable();
            let median = neighbors[4]; // Élément du milieu (index 4 sur 9)

            output.put_pixel(x, y, image::Luma([median]));
        }
    }

    output
}

/// Corrige l'inclinaison d'une image (deskew).
///
/// Cette fonction détecte et corrige l'inclinaison d'un document scanné ou photographié
/// en utilisant la méthode de projection horizontale.
///
/// # Algorithme
///
/// 1. **Détection d'angle** : teste des angles de -20° à +20° par pas de 0.5°.
///    Pour chaque angle candidat, l'image est virtuellement projetée sur l'axe horizontal
///    et la variance des sommes de lignes est calculée. Un texte bien aligné produit des
///    lignes alternant entre zones denses (texte) et zones vides (interlignes), ce qui
///    maximise la variance. L'angle donnant la variance maximale est retenu.
///
/// 2. **Rotation** : l'image est pivotée de l'angle opposé avec interpolation bilinéaire
///    pour éviter les artefacts. Les pixels hors image sont remplis en blanc (255).
///
/// # Arguments
///
/// * `image` - L'image en niveaux de gris à corriger
///
/// # Exemple
///
/// ```no_run
/// use text_recognition::preprocessing::{to_grayscale, deskew};
/// use image::open;
///
/// let img = open("skewed_document.png").unwrap();
/// let gray = to_grayscale(&img);
/// let deskewed = deskew(&gray);
/// ```
pub fn deskew(image: &GrayImage) -> GrayImage {
    let angle = detect_skew_angle(image);
    if angle.abs() < 0.1 {
        // Angle négligeable, pas de rotation nécessaire
        return image.clone();
    }
    rotate_image(image, -angle)
}

/// Détecte l'angle d'inclinaison d'une image par projection horizontale.
///
/// Teste des angles de -20° à +20° par pas de 0.5° et retourne l'angle
/// qui maximise la variance des projections horizontales.
///
/// # Arguments
///
/// * `image` - L'image en niveaux de gris à analyser
///
/// # Retour
///
/// L'angle d'inclinaison estimé en degrés (valeur positive = sens horaire).
fn detect_skew_angle(image: &GrayImage) -> f64 {
    let (width, height) = image.dimensions();
    let cx = width as f64 / 2.0;
    let cy = height as f64 / 2.0;

    let mut best_angle = 0.0f64;
    let mut best_variance = 0.0f64;

    // Tester des angles de -20° à +20° par pas de 0.5°
    let mut angle = -20.0f64;
    while angle <= 20.0 {
        let rad = angle.to_radians();
        let cos_a = rad.cos();
        let sin_a = rad.sin();

        // Calculer la projection horizontale pour cet angle
        let mut row_sums = vec![0u64; height as usize];

        for y in 0..height {
            for x in 0..width {
                // Coordonnées relatives au centre
                let dx = x as f64 - cx;
                let dy = y as f64 - cy;

                // Pixel source après rotation inverse
                let src_x = dx * cos_a + dy * sin_a + cx;
                let src_y = -dx * sin_a + dy * cos_a + cy;

                if src_x >= 0.0
                    && src_x < width as f64 - 1.0
                    && src_y >= 0.0
                    && src_y < height as f64 - 1.0
                {
                    // Interpolation bilinéaire pour la valeur du pixel source
                    let sx = src_x as u32;
                    let sy = src_y as u32;
                    let fx = src_x - sx as f64;
                    let fy = src_y - sy as f64;

                    let p00 = image.get_pixel(sx, sy)[0] as f64;
                    let p10 = image.get_pixel(sx + 1, sy)[0] as f64;
                    let p01 = image.get_pixel(sx, sy + 1)[0] as f64;
                    let p11 = image.get_pixel(sx + 1, sy + 1)[0] as f64;

                    let val = p00 * (1.0 - fx) * (1.0 - fy)
                        + p10 * fx * (1.0 - fy)
                        + p01 * (1.0 - fx) * fy
                        + p11 * fx * fy;

                    // Pixel sombre = texte (valeur basse = contribution forte)
                    row_sums[y as usize] += (255.0 - val) as u64;
                }
            }
        }

        // Calculer la variance des sommes de lignes
        let n = row_sums.len() as f64;
        let mean = row_sums.iter().sum::<u64>() as f64 / n;
        let variance = row_sums
            .iter()
            .map(|&s| {
                let diff = s as f64 - mean;
                diff * diff
            })
            .sum::<f64>()
            / n;

        if variance > best_variance {
            best_variance = variance;
            best_angle = angle;
        }

        angle += 0.5;
    }

    best_angle
}

/// Fait pivoter une image en niveaux de gris d'un angle donné avec interpolation bilinéaire.
///
/// La rotation est effectuée autour du centre de l'image. Les pixels hors image
/// après rotation sont remplis en blanc (255).
///
/// # Arguments
///
/// * `image` - L'image en niveaux de gris à faire pivoter
/// * `angle_deg` - L'angle de rotation en degrés (positif = sens antihoraire)
///
/// # Retour
///
/// Une nouvelle image pivotée de même taille que l'originale.
fn rotate_image(image: &GrayImage, angle_deg: f64) -> GrayImage {
    let (width, height) = image.dimensions();
    let cx = width as f64 / 2.0;
    let cy = height as f64 / 2.0;

    let rad = angle_deg.to_radians();
    let cos_a = rad.cos();
    let sin_a = rad.sin();

    let mut output = GrayImage::new(width, height);

    for y in 0..height {
        for x in 0..width {
            // Coordonnées relatives au centre
            let dx = x as f64 - cx;
            let dy = y as f64 - cy;

            // Coordonnées dans l'image source (rotation inverse)
            let src_x = dx * cos_a + dy * sin_a + cx;
            let src_y = -dx * sin_a + dy * cos_a + cy;

            if src_x >= 0.0
                && src_x < width as f64 - 1.0
                && src_y >= 0.0
                && src_y < height as f64 - 1.0
            {
                // Interpolation bilinéaire
                let sx = src_x as u32;
                let sy = src_y as u32;
                let fx = src_x - sx as f64;
                let fy = src_y - sy as f64;

                let p00 = image.get_pixel(sx, sy)[0] as f64;
                let p10 = image.get_pixel(sx + 1, sy)[0] as f64;
                let p01 = image.get_pixel(sx, sy + 1)[0] as f64;
                let p11 = image.get_pixel(sx + 1, sy + 1)[0] as f64;

                let val = p00 * (1.0 - fx) * (1.0 - fy)
                    + p10 * fx * (1.0 - fy)
                    + p01 * (1.0 - fx) * fy
                    + p11 * fx * fy;

                output.put_pixel(x, y, image::Luma([val.round() as u8]));
            } else {
                // Remplir les bords avec du blanc
                output.put_pixel(x, y, image::Luma([255u8]));
            }
        }
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

    #[test]
    fn test_denoise_removes_salt_and_pepper() {
        use image::Luma;

        // Créer une image 5x5 avec bruit salt-and-pepper
        let mut img = GrayImage::new(5, 5);

        // Remplir avec une valeur uniforme
        for y in 0..5 {
            for x in 0..5 {
                img.put_pixel(x, y, Luma([128]));
            }
        }

        // Ajouter du bruit (pixels isolés)
        img.put_pixel(2, 2, Luma([0])); // Pepper (noir)
        img.put_pixel(1, 1, Luma([255])); // Salt (blanc)
        img.put_pixel(3, 3, Luma([255])); // Salt (blanc)

        let denoised = denoise(&img);

        // Les pixels bruités au centre devraient être corrigés
        // Le filtre médian remplace les valeurs aberrantes par la médiane du voisinage
        assert_ne!(
            denoised.get_pixel(2, 2)[0],
            0,
            "Pepper noise should be removed"
        );
        assert_ne!(
            denoised.get_pixel(1, 1)[0],
            255,
            "Salt noise should be removed"
        );

        // Les pixels corrigés devraient être proches de 128
        assert!(
            (denoised.get_pixel(2, 2)[0] as i16 - 128).abs() < 10,
            "Denoised pixel should be close to 128"
        );
    }

    #[test]
    fn test_denoise_preserves_edges() {
        use image::Luma;

        // Créer une image avec un contour net (moitié noire, moitié blanche)
        let mut img = GrayImage::new(5, 5);

        for y in 0..5 {
            for x in 0..5 {
                let value = if x < 2 { 50 } else { 200 };
                img.put_pixel(x, y, Luma([value]));
            }
        }

        let denoised = denoise(&img);

        // Les zones uniformes devraient rester similaires
        assert_eq!(
            denoised.get_pixel(1, 2)[0],
            50,
            "Dark area should be preserved"
        );
        assert_eq!(
            denoised.get_pixel(3, 2)[0],
            200,
            "Bright area should be preserved"
        );
    }

    #[test]
    fn test_denoise_median_calculation() {
        use image::Luma;

        // Créer une image de test 3x3 avec des valeurs connues
        let mut img = GrayImage::new(3, 3);
        let values = [
            [10, 20, 30],
            [40, 100, 60], // Centre = 100, médiane du voisinage devrait être calculée
            [70, 80, 90],
        ];

        for y in 0..3 {
            for x in 0..3 {
                img.put_pixel(x, y, Luma([values[y as usize][x as usize]]));
            }
        }

        let denoised = denoise(&img);

        // Le pixel central devrait être la médiane de [10,20,30,40,100,60,70,80,90]
        // Trié: [10,20,30,40,60,70,80,90,100]
        // Médiane (index 4): 60
        assert_eq!(
            denoised.get_pixel(1, 1)[0],
            60,
            "Center pixel should be the median of neighborhood"
        );
    }

    #[test]
    fn test_deskew_preserves_dimensions() {
        use image::Luma;

        // Créer une image uniforme (angle nul attendu)
        let mut img = GrayImage::new(20, 20);
        for y in 0..20 {
            for x in 0..20 {
                img.put_pixel(x, y, Luma([200]));
            }
        }

        let deskewed = deskew(&img);

        // Les dimensions doivent être conservées
        assert_eq!(deskewed.dimensions(), img.dimensions());
    }

    #[test]
    fn test_deskew_uniform_image_unchanged() {
        use image::Luma;

        // Une image uniforme n'a pas d'inclinaison détectable
        // -> deskew doit retourner l'image quasi inchangée
        let mut img = GrayImage::new(30, 30);
        for y in 0..30 {
            for x in 0..30 {
                img.put_pixel(x, y, Luma([200]));
            }
        }

        let deskewed = deskew(&img);
        assert_eq!(deskewed.dimensions(), (30, 30));
    }

    #[test]
    fn test_detect_skew_angle_horizontal_lines() {
        use image::Luma;

        // Créer une image avec des lignes horizontales (texte simulé)
        // -> l'angle détecté doit être proche de 0°
        let width = 60u32;
        let height = 40u32;
        let mut img = GrayImage::new(width, height);

        // Fond blanc
        for y in 0..height {
            for x in 0..width {
                img.put_pixel(x, y, Luma([255]));
            }
        }

        // Lignes sombres horizontales (simulation de texte)
        for row in [8u32, 18, 28] {
            for x in 5..55 {
                img.put_pixel(x, row, Luma([30]));
            }
        }

        let angle = detect_skew_angle(&img);

        // L'angle détecté doit être proche de 0° (lignes déjà horizontales)
        assert!(
            angle.abs() <= 2.0,
            "Angle détecté {} devrait être proche de 0°",
            angle
        );
    }

    #[test]
    fn test_rotate_image_zero_angle() {
        use image::Luma;

        // Une rotation de 0° doit retourner une image très proche de l'originale
        let mut img = GrayImage::new(10, 10);
        for y in 0..10 {
            for x in 0..10 {
                img.put_pixel(x, y, Luma([(x * 25) as u8]));
            }
        }

        let rotated = rotate_image(&img, 0.0);
        assert_eq!(rotated.dimensions(), img.dimensions());

        // Les pixels centraux (hors bords) doivent être quasi identiques
        for y in 1..9 {
            for x in 1..9 {
                let orig = img.get_pixel(x, y)[0] as i16;
                let rot = rotated.get_pixel(x, y)[0] as i16;
                assert!(
                    (orig - rot).abs() <= 2,
                    "Pixel ({},{}) : orig={} rot={}",
                    x,
                    y,
                    orig,
                    rot
                );
            }
        }
    }

    #[test]
    fn test_preprocess_pipeline_order() {
        use image::{GenericImageView, Luma};

        // Créer une image de test
        let mut img = GrayImage::new(10, 10);
        for y in 0..10 {
            for x in 0..10 {
                img.put_pixel(x, y, Luma([128]));
            }
        }

        let dynamic_img = DynamicImage::ImageLuma8(img);

        // Tester avec toutes les options activées
        let config = PreprocessingConfig {
            to_grayscale: true,
            binarize: true,
            binarization_method: BinarizationMethod::Fixed(128),
            adjust_contrast: true,
            contrast_factor: 1.5,
            denoise: true,
            deskew: true,
        };

        let result = preprocess_image(&dynamic_img, &config);

        // Le pipeline devrait réussir sans erreur
        assert!(result.is_ok(), "Preprocessing pipeline should succeed");

        let processed = result.unwrap();
        assert_eq!(
            processed.dimensions(),
            (10, 10),
            "Dimensions should be preserved"
        );
    }

    #[test]
    fn test_to_grayscale_from_rgb() {
        use image::{Rgb, RgbImage};

        // Créer une image RGB de test
        let mut rgb_img = RgbImage::new(3, 3);
        rgb_img.put_pixel(0, 0, Rgb([255, 0, 0])); // Rouge
        rgb_img.put_pixel(1, 1, Rgb([0, 255, 0])); // Vert
        rgb_img.put_pixel(2, 2, Rgb([0, 0, 255])); // Bleu

        let dynamic_img = DynamicImage::ImageRgb8(rgb_img);

        // Convertir en niveaux de gris
        let gray = to_grayscale(&dynamic_img);

        // Vérifier que l'image est bien en niveaux de gris
        assert_eq!(gray.dimensions(), (3, 3));

        // Vérifier que la conversion a réussi et que les pixels ont des valeurs valides
        // (les pixels u8 sont automatiquement dans [0, 255])
        assert_eq!(gray.pixels().count(), 9, "Should have 9 pixels");
    }

    #[test]
    fn test_preprocess_with_minimal_config() {
        use image::{GenericImageView, Luma};

        // Créer une image de test
        let mut img = GrayImage::new(5, 5);
        for y in 0..5 {
            for x in 0..5 {
                img.put_pixel(x, y, Luma([150]));
            }
        }

        let dynamic_img = DynamicImage::ImageLuma8(img);

        // Configuration minimale : seulement grayscale
        let config = PreprocessingConfig {
            to_grayscale: true,
            binarize: false,
            binarization_method: BinarizationMethod::Otsu,
            adjust_contrast: false,
            contrast_factor: 1.0,
            denoise: false,
            deskew: false,
        };

        let result = preprocess_image(&dynamic_img, &config);

        assert!(result.is_ok(), "Minimal preprocessing should succeed");

        let processed = result.unwrap();
        assert_eq!(processed.dimensions(), (5, 5));
    }

    #[test]
    fn test_preprocess_only_binarization() {
        use image::Luma;

        // Créer une image de test avec des valeurs variées
        let mut img = GrayImage::new(4, 4);
        for y in 0..4 {
            for x in 0..4 {
                let value = if (x + y) % 2 == 0 { 50 } else { 200 };
                img.put_pixel(x, y, Luma([value]));
            }
        }

        let dynamic_img = DynamicImage::ImageLuma8(img);

        // Configuration : seulement binarisation
        let config = PreprocessingConfig {
            to_grayscale: false,
            binarize: true,
            binarization_method: BinarizationMethod::Fixed(100),
            adjust_contrast: false,
            contrast_factor: 1.0,
            denoise: false,
            deskew: false,
        };

        let result = preprocess_image(&dynamic_img, &config);

        assert!(
            result.is_ok(),
            "Binarization-only preprocessing should succeed"
        );

        let processed = result.unwrap();

        // Vérifier que l'image est bien binarisée
        let gray_result = processed.to_luma8();
        for pixel in gray_result.pixels() {
            assert!(
                pixel[0] == 0 || pixel[0] == 255,
                "Binarized pixel should be 0 or 255, got {}",
                pixel[0]
            );
        }
    }

    #[test]
    fn test_preprocess_contrast_then_binarize() {
        use image::Luma;

        // Créer une image avec faible contraste
        let mut img = GrayImage::new(4, 4);
        for y in 0..4 {
            for x in 0..4 {
                let value = if (x + y) % 2 == 0 { 100 } else { 140 };
                img.put_pixel(x, y, Luma([value]));
            }
        }

        let dynamic_img = DynamicImage::ImageLuma8(img);

        // Configuration : augmenter le contraste puis binariser
        let config = PreprocessingConfig {
            to_grayscale: false,
            binarize: true,
            binarization_method: BinarizationMethod::Otsu,
            adjust_contrast: true,
            contrast_factor: 2.0,
            denoise: false,
            deskew: false,
        };

        let result = preprocess_image(&dynamic_img, &config);

        assert!(
            result.is_ok(),
            "Contrast + binarization preprocessing should succeed"
        );

        let processed = result.unwrap();

        // Vérifier que le résultat est binarisé
        let gray_result = processed.to_luma8();
        for pixel in gray_result.pixels() {
            assert!(
                pixel[0] == 0 || pixel[0] == 255,
                "Final image should be binarized"
            );
        }
    }

    #[test]
    fn test_preprocess_denoise_then_binarize() {
        use image::{GenericImageView, Luma};

        // Créer une image avec du bruit
        let mut img = GrayImage::new(5, 5);
        for y in 0..5 {
            for x in 0..5 {
                img.put_pixel(x, y, Luma([128]));
            }
        }
        // Ajouter des pixels bruités
        img.put_pixel(2, 2, Luma([0]));
        img.put_pixel(1, 1, Luma([255]));

        let dynamic_img = DynamicImage::ImageLuma8(img);

        // Configuration : débruiter puis binariser
        let config = PreprocessingConfig {
            to_grayscale: false,
            binarize: true,
            binarization_method: BinarizationMethod::Fixed(128),
            adjust_contrast: false,
            contrast_factor: 1.0,
            denoise: true,
            deskew: false,
        };

        let result = preprocess_image(&dynamic_img, &config);

        assert!(
            result.is_ok(),
            "Denoise + binarization preprocessing should succeed"
        );

        let processed = result.unwrap();
        assert_eq!(processed.dimensions(), (5, 5));
    }

    #[test]
    fn test_binarization_method_clone() {
        let method1 = BinarizationMethod::Otsu;
        let method2 = method1;

        assert_eq!(method1, method2);

        let method3 = BinarizationMethod::Fixed(150);
        let method4 = method3;

        assert_eq!(method3, method4);
    }

    #[test]
    fn test_preprocessing_config_clone() {
        let config1 = PreprocessingConfig::default();
        let config2 = config1.clone();

        assert_eq!(config1.to_grayscale, config2.to_grayscale);
        assert_eq!(config1.binarize, config2.binarize);
        assert_eq!(config1.binarization_method, config2.binarization_method);
        assert_eq!(config1.adjust_contrast, config2.adjust_contrast);
        assert_eq!(config1.contrast_factor, config2.contrast_factor);
        assert_eq!(config1.denoise, config2.denoise);
        assert_eq!(config1.deskew, config2.deskew);
    }

    #[test]
    fn test_binarize_all_methods() {
        use image::Luma;

        // Créer une image de test
        let mut img = GrayImage::new(10, 10);
        for y in 0..10 {
            for x in 0..10 {
                let value = if x < 5 { 60 } else { 180 };
                img.put_pixel(x, y, Luma([value]));
            }
        }

        // Tester chaque méthode de binarisation
        let methods = vec![
            BinarizationMethod::Otsu,
            BinarizationMethod::Fixed(120),
            BinarizationMethod::Adaptive,
        ];

        for method in methods {
            let binary = binarize(&img, method);

            // Vérifier que tous les pixels sont 0 ou 255
            for pixel in binary.pixels() {
                assert!(
                    pixel[0] == 0 || pixel[0] == 255,
                    "Binarization method {:?} should produce only 0 or 255",
                    method
                );
            }
        }
    }
}
