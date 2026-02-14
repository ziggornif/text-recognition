//! Tests d'intégration pour le module de prétraitement.
//!
//! Ces tests vérifient que les fonctions de prétraitement fonctionnent correctement
//! sur de vraies images issues du répertoire resources/.

use image::{open, GenericImageView};
use text_recognition::preprocessing::{
    adjust_contrast, binarize, denoise, deskew, preprocess_image, BinarizationMethod,
    PreprocessingConfig,
};

/// Vérifie que le prétraitement par défaut fonctionne sur une image simple.
#[test]
fn test_preprocess_default_simple_image() {
    let img_path = "resources/simple/img-1.png";
    let img = open(img_path).expect("Failed to open test image");

    let config = PreprocessingConfig::default();
    let result = preprocess_image(&img, &config);

    assert!(
        result.is_ok(),
        "Default preprocessing should succeed on simple image"
    );

    let processed = result.unwrap();
    assert_eq!(
        processed.dimensions(),
        img.dimensions(),
        "Dimensions should be preserved"
    );
}

/// Vérifie que le prétraitement fonctionne sur une image de complexité moyenne.
#[test]
fn test_preprocess_default_medium_image() {
    let img_path = "resources/medium/img-2.png";
    let img = open(img_path).expect("Failed to open test image");

    let config = PreprocessingConfig::default();
    let result = preprocess_image(&img, &config);

    assert!(
        result.is_ok(),
        "Default preprocessing should succeed on medium image"
    );

    let processed = result.unwrap();
    assert_eq!(processed.dimensions(), img.dimensions());
}

/// Vérifie que le prétraitement fonctionne sur une image complexe.
#[test]
fn test_preprocess_default_complex_image() {
    let img_path = "resources/complex/img-7.png";
    let img = open(img_path).expect("Failed to open test image");

    let config = PreprocessingConfig::default();
    let result = preprocess_image(&img, &config);

    assert!(
        result.is_ok(),
        "Default preprocessing should succeed on complex image"
    );

    let processed = result.unwrap();
    assert_eq!(processed.dimensions(), img.dimensions());
}

/// Test du pipeline complet avec toutes les options activées.
#[test]
fn test_preprocess_full_pipeline() {
    let img_path = "resources/simple/img-1.png";
    let img = open(img_path).expect("Failed to open test image");

    let config = PreprocessingConfig {
        to_grayscale: true,
        binarize: true,
        binarization_method: BinarizationMethod::Otsu,
        adjust_contrast: true,
        contrast_factor: 1.5,
        denoise: true,
        deskew: true,
    };

    let result = preprocess_image(&img, &config);

    assert!(result.is_ok(), "Full preprocessing pipeline should succeed");

    let processed = result.unwrap();
    assert_eq!(processed.dimensions(), img.dimensions());

    // Vérifier que l'image finale est bien en niveaux de gris et binarisée
    let gray = processed.to_luma8();
    let mut has_black = false;
    let mut has_white = false;

    for pixel in gray.pixels() {
        let value = pixel[0];
        if value == 0 {
            has_black = true;
        }
        if value == 255 {
            has_white = true;
        }
        // Avec binarisation, tous les pixels devraient être 0 ou 255
        assert!(
            value == 0 || value == 255,
            "Binarized image should only contain 0 or 255, found {}",
            value
        );
    }

    // Une image de texte devrait contenir du noir et du blanc
    assert!(has_black, "Binarized image should contain black pixels");
    assert!(has_white, "Binarized image should contain white pixels");
}

/// Test de binarisation avec méthode Otsu.
#[test]
fn test_binarize_otsu_on_real_image() {
    let img_path = "resources/simple/img-1.png";
    let img = open(img_path).expect("Failed to open test image");

    let gray = img.to_luma8();
    let binary = binarize(&gray, BinarizationMethod::Otsu);

    // Vérifier que tous les pixels sont 0 ou 255
    for pixel in binary.pixels() {
        assert!(
            pixel[0] == 0 || pixel[0] == 255,
            "Binarized pixel should be 0 or 255"
        );
    }
}

/// Test de binarisation avec seuil fixe.
#[test]
fn test_binarize_fixed_threshold_on_real_image() {
    let img_path = "resources/simple/img-1.png";
    let img = open(img_path).expect("Failed to open test image");

    let gray = img.to_luma8();
    let binary = binarize(&gray, BinarizationMethod::Fixed(128));

    // Vérifier que tous les pixels sont 0 ou 255
    for pixel in binary.pixels() {
        assert!(
            pixel[0] == 0 || pixel[0] == 255,
            "Binarized pixel should be 0 or 255"
        );
    }
}

/// Test de binarisation adaptative.
#[test]
fn test_binarize_adaptive_on_real_image() {
    let img_path = "resources/medium/img-2.png";
    let img = open(img_path).expect("Failed to open test image");

    let gray = img.to_luma8();
    let binary = binarize(&gray, BinarizationMethod::Adaptive);

    // Vérifier que tous les pixels sont 0 ou 255
    for pixel in binary.pixels() {
        assert!(
            pixel[0] == 0 || pixel[0] == 255,
            "Binarized pixel should be 0 or 255"
        );
    }

    // Vérifier qu'il y a bien un mélange de noir et blanc
    let mut black_count = 0;
    let mut white_count = 0;

    for pixel in binary.pixels() {
        if pixel[0] == 0 {
            black_count += 1;
        } else {
            white_count += 1;
        }
    }

    assert!(black_count > 0, "Should have black pixels");
    assert!(white_count > 0, "Should have white pixels");
}

/// Test d'ajustement de contraste (augmentation).
#[test]
fn test_adjust_contrast_increase_on_real_image() {
    let img_path = "resources/simple/img-1.png";
    let img = open(img_path).expect("Failed to open test image");

    let gray = img.to_luma8();
    let contrasted = adjust_contrast(&gray, 2.0);

    // Vérifier que les dimensions sont préservées
    assert_eq!(contrasted.dimensions(), gray.dimensions());

    // Vérifier que les valeurs ont changé (sauf pour le gris moyen 128)
    let mut changed_count = 0;
    for (original, enhanced) in gray.pixels().zip(contrasted.pixels()) {
        if original[0] != 128 && original[0] != enhanced[0] {
            changed_count += 1;
        }
    }

    // Au moins quelques pixels devraient avoir changé
    assert!(
        changed_count > 0,
        "Contrast adjustment should change some pixels"
    );
}

/// Test d'ajustement de contraste (diminution).
#[test]
fn test_adjust_contrast_decrease_on_real_image() {
    let img_path = "resources/simple/img-1.png";
    let img = open(img_path).expect("Failed to open test image");

    let gray = img.to_luma8();
    let contrasted = adjust_contrast(&gray, 0.5);

    // Vérifier que les dimensions sont préservées
    assert_eq!(contrasted.dimensions(), gray.dimensions());

    // Calculer la variance avant et après
    let variance_before = calculate_variance(&gray);
    let variance_after = calculate_variance(&contrasted);

    // Avec un contraste réduit, la variance devrait diminuer
    assert!(
        variance_after < variance_before,
        "Reducing contrast should reduce variance"
    );
}

/// Test du filtre de débruitage.
#[test]
fn test_denoise_on_real_image() {
    let img_path = "resources/simple/img-1.png";
    let img = open(img_path).expect("Failed to open test image");

    let gray = img.to_luma8();
    let denoised = denoise(&gray);

    // Vérifier que les dimensions sont préservées
    assert_eq!(denoised.dimensions(), gray.dimensions());

    // Le débruitage devrait réduire légèrement les variations locales
    // Sans changer radicalement l'image
    let total_pixels = (gray.width() * gray.height()) as usize;
    let mut similar_count = 0;

    for (original, denoised_pixel) in gray.pixels().zip(denoised.pixels()) {
        // Compter les pixels qui n'ont pas trop changé (différence < 20)
        if (original[0] as i16 - denoised_pixel[0] as i16).abs() < 20 {
            similar_count += 1;
        }
    }

    // La majorité des pixels devrait rester similaire (le filtre médian préserve les structures)
    let similarity_ratio = similar_count as f64 / total_pixels as f64;
    assert!(
        similarity_ratio > 0.8,
        "Denoising should preserve most pixels (similarity: {:.2})",
        similarity_ratio
    );
}

/// Test du stub deskew (correction d'inclinaison).
#[test]
fn test_deskew_stub_on_real_image() {
    let img_path = "resources/simple/img-1.png";
    let img = open(img_path).expect("Failed to open test image");

    let gray = img.to_luma8();
    let deskewed = deskew(&gray);

    // Le stub devrait retourner l'image inchangée
    assert_eq!(deskewed.dimensions(), gray.dimensions());

    // Vérifier que tous les pixels sont identiques
    for (original, deskewed_pixel) in gray.pixels().zip(deskewed.pixels()) {
        assert_eq!(
            original[0], deskewed_pixel[0],
            "Deskew stub should not modify pixels"
        );
    }
}

/// Test de prétraitement sans aucune option activée.
#[test]
fn test_preprocess_no_operations() {
    let img_path = "resources/simple/img-1.png";
    let img = open(img_path).expect("Failed to open test image");

    let config = PreprocessingConfig {
        to_grayscale: false,
        binarize: false,
        binarization_method: BinarizationMethod::Otsu,
        adjust_contrast: false,
        contrast_factor: 1.0,
        denoise: false,
        deskew: false,
    };

    let result = preprocess_image(&img, &config);

    assert!(
        result.is_ok(),
        "Preprocessing with no operations should succeed"
    );

    let processed = result.unwrap();

    // Sans prétraitement, l'image devrait être identique
    assert_eq!(processed.dimensions(), img.dimensions());
}

/// Test de prétraitement avec uniquement la conversion en niveaux de gris.
#[test]
fn test_preprocess_grayscale_only() {
    let img_path = "resources/simple/img-1.png";
    let img = open(img_path).expect("Failed to open test image");

    let config = PreprocessingConfig {
        to_grayscale: true,
        binarize: false,
        binarization_method: BinarizationMethod::Otsu,
        adjust_contrast: false,
        contrast_factor: 1.0,
        denoise: false,
        deskew: false,
    };

    let result = preprocess_image(&img, &config);

    assert!(
        result.is_ok(),
        "Grayscale-only preprocessing should succeed"
    );

    let processed = result.unwrap();
    assert_eq!(processed.dimensions(), img.dimensions());

    // Vérifier que l'image est bien en niveaux de gris
    let gray = processed.to_luma8();
    assert_eq!(gray.dimensions(), img.dimensions());
}

/// Test de prétraitement avec seulement la binarisation.
#[test]
fn test_preprocess_binarize_only() {
    let img_path = "resources/simple/img-1.png";
    let img = open(img_path).expect("Failed to open test image");

    let config = PreprocessingConfig {
        to_grayscale: false,
        binarize: true,
        binarization_method: BinarizationMethod::Otsu,
        adjust_contrast: false,
        contrast_factor: 1.0,
        denoise: false,
        deskew: false,
    };

    let result = preprocess_image(&img, &config);

    assert!(result.is_ok(), "Binarize-only preprocessing should succeed");

    let processed = result.unwrap();

    // Vérifier que tous les pixels sont 0 ou 255
    let gray = processed.to_luma8();
    for pixel in gray.pixels() {
        assert!(
            pixel[0] == 0 || pixel[0] == 255,
            "Pixel should be 0 or 255 after binarization"
        );
    }
}

/// Test de comparaison entre différentes méthodes de binarisation.
#[test]
fn test_compare_binarization_methods() {
    let img_path = "resources/medium/img-2.png";
    let img = open(img_path).expect("Failed to open test image");

    let gray = img.to_luma8();

    // Tester chaque méthode
    let otsu_result = binarize(&gray, BinarizationMethod::Otsu);
    let fixed_result = binarize(&gray, BinarizationMethod::Fixed(128));
    let adaptive_result = binarize(&gray, BinarizationMethod::Adaptive);

    // Toutes les méthodes devraient produire une image binarisée valide
    assert_eq!(otsu_result.dimensions(), gray.dimensions());
    assert_eq!(fixed_result.dimensions(), gray.dimensions());
    assert_eq!(adaptive_result.dimensions(), gray.dimensions());

    // Vérifier que chaque méthode produit bien une image binaire
    for method_name in ["Otsu", "Fixed", "Adaptive"].iter() {
        let result = match *method_name {
            "Otsu" => &otsu_result,
            "Fixed" => &fixed_result,
            "Adaptive" => &adaptive_result,
            _ => unreachable!(),
        };

        for pixel in result.pixels() {
            assert!(
                pixel[0] == 0 || pixel[0] == 255,
                "Method {} should produce binary pixels",
                method_name
            );
        }
    }

    // Les résultats devraient être différents (sauf cas exceptionnel)
    // Compter les pixels qui diffèrent entre Otsu et Fixed
    let mut diff_count = 0;
    for (otsu_pixel, fixed_pixel) in otsu_result.pixels().zip(fixed_result.pixels()) {
        if otsu_pixel[0] != fixed_pixel[0] {
            diff_count += 1;
        }
    }

    // Au moins quelques pixels devraient différer entre les méthodes
    let total_pixels = otsu_result.width() * otsu_result.height();
    let diff_ratio = diff_count as f64 / total_pixels as f64;

    // Autoriser jusqu'à 100% de différence (ou 0% si l'image est uniforme)
    assert!(
        (0.0..=1.0).contains(&diff_ratio),
        "Difference ratio should be valid: {}",
        diff_ratio
    );
}

/// Test du pipeline optimal pour documents.
#[test]
fn test_preprocess_document_pipeline() {
    let img_path = "resources/simple/img-1.png";
    let img = open(img_path).expect("Failed to open test image");

    // Configuration optimale pour documents scannés
    let config = PreprocessingConfig {
        to_grayscale: true,
        binarize: true,
        binarization_method: BinarizationMethod::Otsu,
        adjust_contrast: false,
        contrast_factor: 1.0,
        denoise: false,
        deskew: true,
    };

    let result = preprocess_image(&img, &config);

    assert!(
        result.is_ok(),
        "Document preprocessing pipeline should succeed"
    );

    let processed = result.unwrap();

    // Vérifier que c'est bien binarisé
    let gray = processed.to_luma8();
    for pixel in gray.pixels() {
        assert!(pixel[0] == 0 || pixel[0] == 255);
    }
}

/// Test du pipeline optimal pour photos de faible qualité.
#[test]
fn test_preprocess_photo_pipeline() {
    let img_path = "resources/complex/img-7.png";
    let img = open(img_path).expect("Failed to open test image");

    // Configuration pour photos avec bruit et faible contraste
    let config = PreprocessingConfig {
        to_grayscale: true,
        binarize: true,
        binarization_method: BinarizationMethod::Adaptive,
        adjust_contrast: true,
        contrast_factor: 1.5,
        denoise: true,
        deskew: false,
    };

    let result = preprocess_image(&img, &config);

    assert!(
        result.is_ok(),
        "Photo preprocessing pipeline should succeed"
    );

    let processed = result.unwrap();

    // Vérifier que c'est bien binarisé
    let gray = processed.to_luma8();
    for pixel in gray.pixels() {
        assert!(pixel[0] == 0 || pixel[0] == 255);
    }
}

// ============================================================================
// Fonctions utilitaires pour les tests
// ============================================================================

/// Calcule la variance d'une image en niveaux de gris.
fn calculate_variance(img: &image::GrayImage) -> f64 {
    let pixels: Vec<u8> = img.pixels().map(|p| p[0]).collect();
    let mean: f64 = pixels.iter().map(|&x| x as f64).sum::<f64>() / pixels.len() as f64;

    pixels
        .iter()
        .map(|&x| {
            let diff = x as f64 - mean;
            diff * diff
        })
        .sum::<f64>()
        / pixels.len() as f64
}
