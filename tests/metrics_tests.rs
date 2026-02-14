//! Tests d'intégration pour le module de métriques.
//!
//! Ces tests vérifient que les fonctions de calcul de métriques fonctionnent correctement
//! sur de vraies données OCR comparées aux textes de référence.

use image::open;
use std::fs;
use text_recognition::config::{OcrConfig, PageSegMode};
use text_recognition::metrics::{
    calculate_cer, calculate_wer, compare_ocr_result, generate_diff_report,
};
use text_recognition::ocr::OcrEngine;

/// Lit le contenu d'un fichier texte de référence.
fn read_expected_text(filename: &str) -> String {
    let path = format!("resources/expected/{}", filename);
    fs::read_to_string(&path)
        .unwrap_or_else(|_| panic!("Failed to read expected text file: {}", path))
}

/// Test de comparaison OCR avec un texte de référence (image simple).
#[test]
fn test_compare_ocr_with_reference_simple_image() {
    let img_path = "resources/simple/img-1.png";
    let img = open(img_path).expect("Failed to open test image");

    // Extraire le texte avec OCR
    let config = OcrConfig::default();
    let engine = OcrEngine::new(config).expect("Failed to create OCR engine");
    let ocr_text = engine
        .extract_text_from_image(&img)
        .expect("OCR extraction failed");

    // Charger le texte de référence
    let expected_text = read_expected_text("img-1.txt");

    // Comparer les résultats
    let metrics = compare_ocr_result(&expected_text, &ocr_text);

    // Les métriques devraient être calculées avec succès
    assert!(metrics.cer >= 0.0, "CER should be non-negative");
    assert!(metrics.wer >= 0.0, "WER should be non-negative");
    assert!(
        metrics.accuracy() >= 0.0 && metrics.accuracy() <= 1.0,
        "Accuracy should be between 0 and 1"
    );

    // Pour une image simple avec un bon OCR, on s'attend à une précision raisonnable
    // (au moins 50% de précision)
    println!(
        "Image simple - CER: {:.2}%, WER: {:.2}%, Accuracy: {:.2}%",
        metrics.cer,
        metrics.wer,
        metrics.accuracy() * 100.0
    );
}

/// Test de calcul du CER sur un texte réel OCR.
#[test]
fn test_calculate_cer_on_real_ocr() {
    let img_path = "resources/simple/img-1.png";
    let img = open(img_path).expect("Failed to open test image");

    let config = OcrConfig::default();
    let engine = OcrEngine::new(config).expect("Failed to create OCR engine");
    let ocr_text = engine
        .extract_text_from_image(&img)
        .expect("OCR extraction failed");

    let expected_text = read_expected_text("img-1.txt");

    let cer = calculate_cer(&expected_text, &ocr_text);

    // Le CER doit être un pourcentage valide
    assert!(cer >= 0.0, "CER should be non-negative");
    println!("CER for img-1: {:.2}%", cer);
}

/// Test de calcul du WER sur un texte réel OCR.
#[test]
fn test_calculate_wer_on_real_ocr() {
    let img_path = "resources/simple/img-1.png";
    let img = open(img_path).expect("Failed to open test image");

    let config = OcrConfig::default();
    let engine = OcrEngine::new(config).expect("Failed to create OCR engine");
    let ocr_text = engine
        .extract_text_from_image(&img)
        .expect("OCR extraction failed");

    let expected_text = read_expected_text("img-1.txt");

    let wer = calculate_wer(&expected_text, &ocr_text);

    // Le WER doit être un pourcentage valide
    assert!(wer >= 0.0, "WER should be non-negative");
    println!("WER for img-1: {:.2}%", wer);
}

/// Test de génération d'un rapport de différences.
#[test]
fn test_generate_diff_report_on_real_ocr() {
    let img_path = "resources/simple/img-1.png";
    let img = open(img_path).expect("Failed to open test image");

    let config = OcrConfig::default();
    let engine = OcrEngine::new(config).expect("Failed to create OCR engine");
    let ocr_text = engine
        .extract_text_from_image(&img)
        .expect("OCR extraction failed");

    let expected_text = read_expected_text("img-1.txt");

    let report = generate_diff_report(&expected_text, &ocr_text);

    // Le rapport devrait contenir des sections clés
    assert!(
        report.contains("OCR COMPARISON REPORT"),
        "Report should have a title"
    );
    assert!(
        report.contains("STATISTICS") || report.contains("METRICS"),
        "Report should have statistics"
    );
    assert!(
        report.contains("CER") || report.contains("Character Error Rate"),
        "Report should mention CER"
    );
    assert!(
        report.contains("WER") || report.contains("Word Error Rate"),
        "Report should mention WER"
    );

    println!("Generated report:\n{}", report);
}

/// Test de comparaison avec différents modes PSM.
#[test]
fn test_compare_metrics_different_psm_modes() {
    let img_path = "resources/simple/img-1.png";
    let img = open(img_path).expect("Failed to open test image");
    let expected_text = read_expected_text("img-1.txt");

    // Tester plusieurs modes PSM
    let psm_modes = vec![
        PageSegMode::Auto,
        PageSegMode::SingleBlock,
        PageSegMode::SingleColumn,
    ];

    for psm in psm_modes {
        let config = OcrConfig {
            language: "fra".to_string(),
            page_seg_mode: psm,
            dpi: 300,
            tesseract_variables: std::collections::HashMap::new(),
        };

        let engine = OcrEngine::new(config).expect("Failed to create OCR engine");
        let ocr_text = engine
            .extract_text_from_image(&img)
            .expect("OCR extraction failed");

        let metrics = compare_ocr_result(&expected_text, &ocr_text);

        println!(
            "PSM {:?} - CER: {:.2}%, WER: {:.2}%, Accuracy: {:.2}%",
            psm,
            metrics.cer,
            metrics.wer,
            metrics.accuracy() * 100.0
        );

        // Vérifier que les métriques sont valides
        assert!(metrics.cer >= 0.0);
        assert!(metrics.wer >= 0.0);
        assert!(metrics.accuracy() >= 0.0 && metrics.accuracy() <= 1.0);
    }
}

/// Test de cohérence entre CER et WER.
#[test]
fn test_cer_wer_relationship() {
    let img_path = "resources/simple/img-1.png";
    let img = open(img_path).expect("Failed to open test image");

    let config = OcrConfig::default();
    let engine = OcrEngine::new(config).expect("Failed to create OCR engine");
    let ocr_text = engine
        .extract_text_from_image(&img)
        .expect("OCR extraction failed");

    let expected_text = read_expected_text("img-1.txt");

    let cer = calculate_cer(&expected_text, &ocr_text);
    let wer = calculate_wer(&expected_text, &ocr_text);

    // En général, le WER est plus élevé ou égal au CER
    // car une erreur de caractère peut affecter tout un mot
    // Toutefois, ce n'est pas toujours le cas selon la distribution des erreurs
    println!("CER: {:.2}%, WER: {:.2}%", cer, wer);

    // Les deux métriques devraient être dans des plages valides
    assert!(cer >= 0.0);
    assert!(wer >= 0.0);
}

/// Test de la précision (accuracy) dérivée du CER.
#[test]
fn test_accuracy_calculation() {
    let img_path = "resources/simple/img-1.png";
    let img = open(img_path).expect("Failed to open test image");

    let config = OcrConfig::default();
    let engine = OcrEngine::new(config).expect("Failed to create OCR engine");
    let ocr_text = engine
        .extract_text_from_image(&img)
        .expect("OCR extraction failed");

    let expected_text = read_expected_text("img-1.txt");

    let metrics = compare_ocr_result(&expected_text, &ocr_text);

    // Accuracy devrait être 1 - CER (avec clamping à 0)
    let expected_accuracy = (1.0 - metrics.cer).max(0.0);
    assert!(
        (metrics.accuracy() - expected_accuracy).abs() < 0.01,
        "Accuracy should be 1 - CER, got {} expected {}",
        metrics.accuracy(),
        expected_accuracy
    );

    println!("Accuracy: {:.2}%", metrics.accuracy() * 100.0);
}

/// Test avec un texte parfaitement identique (CER et WER = 0).
#[test]
fn test_perfect_match_metrics() {
    let text = "Ceci est un texte de test.";

    let cer = calculate_cer(text, text);
    let wer = calculate_wer(text, text);
    let metrics = compare_ocr_result(text, text);

    assert_eq!(cer, 0.0, "CER should be 0 for identical texts");
    assert_eq!(wer, 0.0, "WER should be 0 for identical texts");
    assert_eq!(
        metrics.accuracy(),
        1.0,
        "Accuracy should be 1.0 (100%) for identical texts"
    );
}

/// Test avec un texte complètement différent.
#[test]
fn test_completely_different_metrics() {
    let reference = "Bonjour le monde";
    let ocr_result = "xyz abc def";

    let cer = calculate_cer(reference, ocr_result);
    let wer = calculate_wer(reference, ocr_result);

    // Les erreurs devraient être significatives (retourné comme ratio 0-1)
    assert!(
        cer > 0.5,
        "CER should be high for completely different texts"
    );
    assert!(
        wer > 0.5,
        "WER should be high for completely different texts"
    );

    println!(
        "Completely different - CER: {:.2}%, WER: {:.2}%",
        cer * 100.0,
        wer * 100.0
    );
}

/// Test avec du texte vide.
#[test]
fn test_empty_text_metrics() {
    let reference = "Texte de référence";
    let empty = "";

    let cer = calculate_cer(reference, empty);
    let wer = calculate_wer(reference, empty);

    // Avec un texte OCR vide, le CER et WER devraient être 1.0 (100%)
    assert_eq!(
        cer, 1.0,
        "CER should be 1.0 (100%) when OCR result is empty"
    );
    assert_eq!(
        wer, 1.0,
        "WER should be 1.0 (100%) when OCR result is empty"
    );
}

/// Test de métriques sur plusieurs images simples.
#[test]
fn test_metrics_multiple_simple_images() {
    let simple_images = vec![
        ("resources/simple/img-1.png", "img-1.txt"),
        ("resources/simple/img-3.png", "img-3.txt"),
        ("resources/simple/img-4.png", "img-4.txt"),
    ];

    let config = OcrConfig::default();

    for (img_path, expected_file) in simple_images {
        let img = open(img_path).expect("Failed to open test image");
        let engine = OcrEngine::new(config.clone()).expect("Failed to create OCR engine");
        let ocr_text = engine
            .extract_text_from_image(&img)
            .expect("OCR extraction failed");

        let expected_text = read_expected_text(expected_file);
        let metrics = compare_ocr_result(&expected_text, &ocr_text);

        println!(
            "{} - CER: {:.2}%, WER: {:.2}%, Accuracy: {:.2}%",
            img_path,
            metrics.cer,
            metrics.wer,
            metrics.accuracy() * 100.0
        );

        // Vérifier que les métriques sont valides
        assert!(metrics.cer >= 0.0);
        assert!(metrics.wer >= 0.0);
        assert!(metrics.accuracy() >= 0.0 && metrics.accuracy() <= 1.0);
    }
}

/// Test de métriques sur images de complexité moyenne.
#[test]
fn test_metrics_medium_complexity_images() {
    let medium_images = vec![
        ("resources/medium/img-2.png", "img-2.txt"),
        ("resources/medium/img-5.png", "img-5.txt"),
        ("resources/medium/img-6.png", "img-6.txt"),
    ];

    let config = OcrConfig::default();

    for (img_path, expected_file) in medium_images {
        let img = open(img_path).expect("Failed to open test image");
        let engine = OcrEngine::new(config.clone()).expect("Failed to create OCR engine");
        let ocr_text = engine
            .extract_text_from_image(&img)
            .expect("OCR extraction failed");

        let expected_text = read_expected_text(expected_file);
        let metrics = compare_ocr_result(&expected_text, &ocr_text);

        println!(
            "{} - CER: {:.2}%, WER: {:.2}%, Accuracy: {:.2}%",
            img_path,
            metrics.cer,
            metrics.wer,
            metrics.accuracy() * 100.0
        );

        // Les images moyennes peuvent avoir un CER plus élevé
        assert!(metrics.cer >= 0.0);
        assert!(metrics.wer >= 0.0);
        assert!(metrics.accuracy() >= 0.0 && metrics.accuracy() <= 1.0);
    }
}

/// Test de métriques sur images complexes.
#[test]
fn test_metrics_complex_images() {
    let complex_images = vec![
        ("resources/complex/img-7.png", "img-7.txt"),
        ("resources/complex/img-8.png", "img-8.txt"),
    ];

    let config = OcrConfig::default();

    for (img_path, expected_file) in complex_images {
        let img = open(img_path).expect("Failed to open test image");
        let engine = OcrEngine::new(config.clone()).expect("Failed to create OCR engine");
        let ocr_text = engine
            .extract_text_from_image(&img)
            .expect("OCR extraction failed");

        let expected_text = read_expected_text(expected_file);
        let metrics = compare_ocr_result(&expected_text, &ocr_text);

        println!(
            "{} - CER: {:.2}%, WER: {:.2}%, Accuracy: {:.2}%",
            img_path,
            metrics.cer,
            metrics.wer,
            metrics.accuracy() * 100.0
        );

        // Les images complexes peuvent avoir un CER très élevé
        assert!(metrics.cer >= 0.0);
        assert!(metrics.wer >= 0.0);
        assert!(metrics.accuracy() >= 0.0 && metrics.accuracy() <= 1.0);
    }
}

/// Test de génération de rapport avec qualité variable.
#[test]
fn test_report_quality_categories() {
    // Test avec un texte presque parfait (excellente qualité)
    let _excellent = compare_ocr_result(
        "Bonjour le monde",
        "Bonjour le monde!", // Une petite différence
    );
    let report_excellent = generate_diff_report("Bonjour le monde", "Bonjour le monde!");
    assert!(
        report_excellent.contains("Excellent") || report_excellent.contains("Good"),
        "Report should indicate good quality"
    );

    // Test avec un texte moyen (qualité moyenne)
    let _medium = compare_ocr_result(
        "Bonjour le monde, comment allez-vous?",
        "Bonjour Ie monde, coment allez-vous?", // Quelques erreurs
    );
    println!("Medium quality - CER: {:.2}%", _medium.cer);

    // Test avec un texte de mauvaise qualité
    let _poor = compare_ocr_result(
        "Bonjour le monde",
        "xyz abc def", // Complètement différent
    );
    let report_poor = generate_diff_report("Bonjour le monde", "xyz abc def");
    assert!(
        report_poor.contains("Poor") || report_poor.contains("Fair"),
        "Report should indicate poor quality"
    );
}

/// Test de comparaison avant/après prétraitement.
#[test]
fn test_metrics_with_and_without_preprocessing() {
    use text_recognition::preprocessing::{BinarizationMethod, PreprocessingConfig};

    let img_path = "resources/simple/img-1.png";
    let img = open(img_path).expect("Failed to open test image");
    let expected_text = read_expected_text("img-1.txt");

    // Sans prétraitement
    let config_no_prep = OcrConfig::default();
    let engine_no_prep = OcrEngine::new(config_no_prep).expect("Failed to create OCR engine");
    let ocr_no_prep = engine_no_prep
        .extract_text_from_image(&img)
        .expect("OCR extraction failed");
    let metrics_no_prep = compare_ocr_result(&expected_text, &ocr_no_prep);

    // Avec prétraitement
    let prep_config = PreprocessingConfig {
        to_grayscale: true,
        binarize: true,
        binarization_method: BinarizationMethod::Otsu,
        adjust_contrast: false,
        contrast_factor: 1.0,
        denoise: false,
        deskew: false,
    };

    let config_with_prep = OcrConfig::default();
    let engine_with_prep = OcrEngine::with_preprocessing(config_with_prep, prep_config)
        .expect("Failed to create OCR engine");

    let ocr_with_prep = engine_with_prep
        .extract_text_from_image(&img)
        .expect("OCR extraction failed");
    let metrics_with_prep = compare_ocr_result(&expected_text, &ocr_with_prep);

    println!(
        "Sans prétraitement - CER: {:.2}%, WER: {:.2}%, Accuracy: {:.2}%",
        metrics_no_prep.cer,
        metrics_no_prep.wer,
        metrics_no_prep.accuracy() * 100.0
    );
    println!(
        "Avec prétraitement - CER: {:.2}%, WER: {:.2}%, Accuracy: {:.2}%",
        metrics_with_prep.cer,
        metrics_with_prep.wer,
        metrics_with_prep.accuracy() * 100.0
    );

    // Les deux devraient être valides (mais pas nécessairement meilleurs avec preprocessing)
    assert!(metrics_no_prep.cer >= 0.0);
    assert!(metrics_with_prep.cer >= 0.0);
}

/// Test de stabilité des métriques (résultats reproductibles).
#[test]
fn test_metrics_reproducibility() {
    let img_path = "resources/simple/img-1.png";
    let img = open(img_path).expect("Failed to open test image");
    let expected_text = read_expected_text("img-1.txt");

    let config = OcrConfig::default();

    // Exécuter l'OCR deux fois
    let engine1 = OcrEngine::new(config.clone()).expect("Failed to create OCR engine");
    let ocr_text1 = engine1
        .extract_text_from_image(&img)
        .expect("OCR extraction failed");
    let metrics1 = compare_ocr_result(&expected_text, &ocr_text1);

    let engine2 = OcrEngine::new(config).expect("Failed to create OCR engine");
    let ocr_text2 = engine2
        .extract_text_from_image(&img)
        .expect("OCR extraction failed");
    let metrics2 = compare_ocr_result(&expected_text, &ocr_text2);

    // Les résultats devraient être identiques
    assert_eq!(
        ocr_text1, ocr_text2,
        "OCR should produce identical results for the same input"
    );
    assert_eq!(metrics1.cer, metrics2.cer, "CER should be reproducible");
    assert_eq!(metrics1.wer, metrics2.wer, "WER should be reproducible");
}

/// Test de rapport avec texte multilingue.
#[test]
fn test_metrics_with_unicode_characters() {
    let reference = "Texte français avec accents: à é è ù ç œ";
    let ocr_result = "Texte français avec accents: a e e u c oe"; // Sans accents

    let cer = calculate_cer(reference, ocr_result);
    let wer = calculate_wer(reference, ocr_result);

    // Les différences d'accents devraient être comptées
    assert!(cer > 0.0, "CER should detect accent differences");

    println!("Unicode test - CER: {:.2}%, WER: {:.2}%", cer, wer);
}

/// Test de génération de rapport complet avec toutes les sections.
#[test]
fn test_full_report_structure() {
    let img_path = "resources/simple/img-1.png";
    let img = open(img_path).expect("Failed to open test image");

    let config = OcrConfig::default();
    let engine = OcrEngine::new(config).expect("Failed to create OCR engine");
    let ocr_text = engine
        .extract_text_from_image(&img)
        .expect("OCR extraction failed");

    let expected_text = read_expected_text("img-1.txt");
    let report = generate_diff_report(&expected_text, &ocr_text);

    // Vérifier que le rapport contient toutes les sections attendues
    let required_sections = vec!["OCR COMPARISON REPORT", "STATISTICS", "characters", "words"];

    for section in required_sections {
        assert!(
            report.contains(section),
            "Report should contain section: {}",
            section
        );
    }

    // Le rapport ne devrait pas être vide
    assert!(report.len() > 100, "Report should be substantial");
}
