//! Tests d'intégration basiques pour le moteur OCR.
//!
//! Ces tests vérifient le fonctionnement de base de l'extraction de texte
//! avec différentes configurations et images de test.

use std::path::Path;
use text_recognition::{OcrConfig, OcrEngine};

/// Teste l'extraction de texte sur une image simple avec configuration par défaut.
#[test]
fn test_extract_text_simple_image() {
    let config = OcrConfig::default();
    let engine = OcrEngine::new(config).expect("Échec de création du moteur OCR");

    let image_path = Path::new("resources/simple/img-4.png");

    // Vérifier que l'image existe
    assert!(
        image_path.exists(),
        "L'image de test {} n'existe pas",
        image_path.display()
    );

    let result = engine.extract_text_from_file(image_path);

    assert!(
        result.is_ok(),
        "L'extraction de texte a échoué : {:?}",
        result.err()
    );

    let text = result.unwrap();

    // Vérifier que du texte a été extrait
    assert!(!text.trim().is_empty(), "Aucun texte extrait de l'image");

    // Vérifier que le texte contient des mots attendus
    assert!(
        text.contains("phrase") || text.contains("test"),
        "Le texte extrait ne contient pas les mots attendus. Texte: {}",
        text
    );
}

/// Teste que le moteur OCR peut être créé avec différentes configurations.
#[test]
fn test_create_engine_with_different_configs() {
    // Configuration par défaut
    let config_default = OcrConfig::default();
    let engine_default = OcrEngine::new(config_default);
    assert!(engine_default.is_ok(), "Échec avec config par défaut");

    // Preset document
    let config_doc = OcrConfig::document_preset();
    let engine_doc = OcrEngine::new(config_doc);
    assert!(engine_doc.is_ok(), "Échec avec preset document");

    // Preset screenshot
    let config_screenshot = OcrConfig::screenshot_preset();
    let engine_screenshot = OcrEngine::new(config_screenshot);
    assert!(engine_screenshot.is_ok(), "Échec avec preset screenshot");

    // Preset single line
    let config_line = OcrConfig::single_line_preset();
    let engine_line = OcrEngine::new(config_line);
    assert!(engine_line.is_ok(), "Échec avec preset single line");
}

/// Teste l'extraction sur une image inexistante (doit échouer proprement).
#[test]
fn test_extract_text_nonexistent_file() {
    let config = OcrConfig::default();
    let engine = OcrEngine::new(config).expect("Échec de création du moteur OCR");

    let image_path = Path::new("resources/nonexistent_image.png");
    let result = engine.extract_text_from_file(image_path);

    assert!(
        result.is_err(),
        "L'extraction aurait dû échouer pour un fichier inexistant"
    );
}

/// Teste l'extraction sur les trois images simples.
#[test]
fn test_extract_text_all_simple_images() {
    let config = OcrConfig::default();
    let engine = OcrEngine::new(config).expect("Échec de création du moteur OCR");

    let simple_images = [
        "resources/simple/img-1.png",
        "resources/simple/img-3.png",
        "resources/simple/img-4.png",
    ];

    for image_path in &simple_images {
        let path = Path::new(image_path);

        assert!(path.exists(), "L'image {} n'existe pas", image_path);

        let result = engine.extract_text_from_file(path);

        assert!(
            result.is_ok(),
            "L'extraction a échoué pour {} : {:?}",
            image_path,
            result.err()
        );

        let text = result.unwrap();

        assert!(
            !text.trim().is_empty(),
            "Aucun texte extrait de {}",
            image_path
        );
    }
}

/// Teste l'extraction avec le preset document sur une image documentaire.
#[test]
fn test_extract_with_document_preset() {
    let config = OcrConfig::document_preset();
    let engine = OcrEngine::new(config).expect("Échec de création du moteur OCR");

    let image_path = Path::new("resources/simple/img-1.png");

    assert!(image_path.exists(), "L'image de test n'existe pas");

    let result = engine.extract_text_from_file(image_path);

    assert!(
        result.is_ok(),
        "L'extraction avec preset document a échoué : {:?}",
        result.err()
    );

    let text = result.unwrap();

    assert!(
        !text.trim().is_empty(),
        "Aucun texte extrait avec preset document"
    );
}

/// Teste que le moteur peut traiter plusieurs images successivement.
#[test]
fn test_multiple_extractions_same_engine() {
    let config = OcrConfig::default();
    let engine = OcrEngine::new(config).expect("Échec de création du moteur OCR");

    let images = ["resources/simple/img-3.png", "resources/simple/img-4.png"];

    for image_path in &images {
        let path = Path::new(image_path);
        let result = engine.extract_text_from_file(path);

        assert!(
            result.is_ok(),
            "L'extraction a échoué pour {} lors du traitement multiple",
            image_path
        );
    }
}

/// Teste l'extraction avec prétraitement activé.
#[test]
fn test_extract_with_preprocessing() {
    use text_recognition::preprocessing::PreprocessingConfig;

    let ocr_config = OcrConfig::default();
    let preproc_config = PreprocessingConfig::default();

    let engine = OcrEngine::with_preprocessing(ocr_config, preproc_config)
        .expect("Échec de création du moteur OCR");

    let image_path = Path::new("resources/simple/img-4.png");

    assert!(image_path.exists(), "L'image de test n'existe pas");

    let result = engine.extract_text_from_file(image_path);

    assert!(
        result.is_ok(),
        "L'extraction avec prétraitement a échoué : {:?}",
        result.err()
    );

    let text = result.unwrap();

    assert!(
        !text.trim().is_empty(),
        "Aucun texte extrait avec prétraitement"
    );
}
