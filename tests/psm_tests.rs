//! Tests des différents modes de segmentation de page (PSM).
//!
//! Ces tests vérifient que tous les modes PSM de Tesseract fonctionnent
//! correctement et peuvent être utilisés sans erreur.

use std::path::Path;
use text_recognition::{OcrConfig, OcrEngine, PageSegMode};

/// Teste le mode PSM OsdOnly (Orientation and Script Detection only).
#[test]
fn test_psm_osd_only() {
    let config = OcrConfig {
        language: "fra".to_string(),
        page_seg_mode: PageSegMode::OsdOnly,
        dpi: 300,
        tesseract_variables: Default::default(),
    };

    let engine = OcrEngine::new(config);
    assert!(
        engine.is_ok(),
        "Échec de création du moteur avec PSM OsdOnly: {:?}",
        engine.err()
    );
}

/// Teste le mode PSM AutoOsd (Automatic page segmentation with OSD).
#[test]
fn test_psm_auto_osd() {
    let config = OcrConfig {
        language: "fra".to_string(),
        page_seg_mode: PageSegMode::AutoOsd,
        dpi: 300,
        tesseract_variables: Default::default(),
    };

    let engine = OcrEngine::new(config);
    assert!(engine.is_ok(), "Échec de création avec PSM AutoOsd");

    if let Ok(engine) = engine {
        let image_path = Path::new("resources/simple/img-4.png");
        if image_path.exists() {
            let result = engine.extract_text_from_file(image_path);
            // Ce mode peut échouer sur certaines images, on vérifie juste qu'il ne panic pas
            let _ = result;
        }
    }
}

/// Teste le mode PSM AutoOnly (Automatic page segmentation without OSD).
#[test]
fn test_psm_auto_only() {
    let config = OcrConfig {
        language: "fra".to_string(),
        page_seg_mode: PageSegMode::AutoOnly,
        dpi: 300,
        tesseract_variables: Default::default(),
    };

    let engine = OcrEngine::new(config);
    assert!(engine.is_ok(), "Échec de création avec PSM AutoOnly");

    if let Ok(engine) = engine {
        let image_path = Path::new("resources/simple/img-4.png");
        if image_path.exists() {
            let result = engine.extract_text_from_file(image_path);
            assert!(
                result.is_ok(),
                "Échec extraction avec AutoOnly: {:?}",
                result.err()
            );
        }
    }
}

/// Teste le mode PSM Auto (Fully automatic page segmentation - default).
#[test]
fn test_psm_auto() {
    let config = OcrConfig {
        language: "fra".to_string(),
        page_seg_mode: PageSegMode::Auto,
        dpi: 300,
        tesseract_variables: Default::default(),
    };

    let engine = OcrEngine::new(config).expect("Échec de création avec PSM Auto");
    let image_path = Path::new("resources/simple/img-4.png");

    assert!(image_path.exists(), "L'image de test n'existe pas");

    let result = engine.extract_text_from_file(image_path);
    assert!(
        result.is_ok(),
        "Échec extraction avec Auto: {:?}",
        result.err()
    );

    let text = result.unwrap();
    assert!(!text.trim().is_empty(), "Aucun texte extrait avec Auto");
}

/// Teste le mode PSM SingleColumn (Single column of text).
#[test]
fn test_psm_single_column() {
    let config = OcrConfig {
        language: "fra".to_string(),
        page_seg_mode: PageSegMode::SingleColumn,
        dpi: 300,
        tesseract_variables: Default::default(),
    };

    let engine = OcrEngine::new(config).expect("Échec de création avec PSM SingleColumn");
    let image_path = Path::new("resources/simple/img-3.png");

    if image_path.exists() {
        let result = engine.extract_text_from_file(image_path);
        assert!(
            result.is_ok(),
            "Échec extraction avec SingleColumn: {:?}",
            result.err()
        );
    }
}

/// Teste le mode PSM SingleBlockVertText (Single vertical block of text).
#[test]
fn test_psm_single_block_vert_text() {
    let config = OcrConfig {
        language: "fra".to_string(),
        page_seg_mode: PageSegMode::SingleBlockVertText,
        dpi: 300,
        tesseract_variables: Default::default(),
    };

    let engine = OcrEngine::new(config).expect("Échec de création avec PSM SingleBlockVertText");
    let image_path = Path::new("resources/medium/img-5.png");

    if image_path.exists() {
        let result = engine.extract_text_from_file(image_path);
        // Le texte vertical peut être difficile, on vérifie juste que ça ne plante pas
        let _ = result;
    }
}

/// Teste le mode PSM SingleBlock (Single block of text).
#[test]
fn test_psm_single_block() {
    let config = OcrConfig {
        language: "fra".to_string(),
        page_seg_mode: PageSegMode::SingleBlock,
        dpi: 300,
        tesseract_variables: Default::default(),
    };

    let engine = OcrEngine::new(config).expect("Échec de création avec PSM SingleBlock");
    let image_path = Path::new("resources/simple/img-4.png");

    assert!(image_path.exists(), "L'image de test n'existe pas");

    let result = engine.extract_text_from_file(image_path);
    assert!(
        result.is_ok(),
        "Échec extraction avec SingleBlock: {:?}",
        result.err()
    );

    let text = result.unwrap();
    assert!(
        !text.trim().is_empty(),
        "Aucun texte extrait avec SingleBlock"
    );
}

/// Teste le mode PSM SingleLine (Single line of text).
#[test]
fn test_psm_single_line() {
    let config = OcrConfig {
        language: "fra".to_string(),
        page_seg_mode: PageSegMode::SingleLine,
        dpi: 300,
        tesseract_variables: Default::default(),
    };

    let engine = OcrEngine::new(config).expect("Échec de création avec PSM SingleLine");
    let image_path = Path::new("resources/simple/img-4.png");

    if image_path.exists() {
        let result = engine.extract_text_from_file(image_path);
        assert!(
            result.is_ok(),
            "Échec extraction avec SingleLine: {:?}",
            result.err()
        );
    }
}

/// Teste le mode PSM SingleWord (Single word).
#[test]
fn test_psm_single_word() {
    let config = OcrConfig {
        language: "fra".to_string(),
        page_seg_mode: PageSegMode::SingleWord,
        dpi: 300,
        tesseract_variables: Default::default(),
    };

    let engine = OcrEngine::new(config).expect("Échec de création avec PSM SingleWord");
    let image_path = Path::new("resources/simple/img-4.png");

    if image_path.exists() {
        let result = engine.extract_text_from_file(image_path);
        // SingleWord peut donner des résultats partiels sur une image multi-mots
        let _ = result;
    }
}

/// Teste le mode PSM CircleWord (Single word in a circle).
#[test]
fn test_psm_circle_word() {
    let config = OcrConfig {
        language: "fra".to_string(),
        page_seg_mode: PageSegMode::CircleWord,
        dpi: 300,
        tesseract_variables: Default::default(),
    };

    let engine = OcrEngine::new(config).expect("Échec de création avec PSM CircleWord");
    let image_path = Path::new("resources/simple/img-4.png");

    if image_path.exists() {
        let result = engine.extract_text_from_file(image_path);
        // Mode spécialisé, peut ne pas fonctionner sur toutes les images
        let _ = result;
    }
}

/// Teste le mode PSM SingleChar (Single character).
#[test]
fn test_psm_single_char() {
    let config = OcrConfig {
        language: "fra".to_string(),
        page_seg_mode: PageSegMode::SingleChar,
        dpi: 300,
        tesseract_variables: Default::default(),
    };

    let engine = OcrEngine::new(config).expect("Échec de création avec PSM SingleChar");
    let image_path = Path::new("resources/simple/img-4.png");

    if image_path.exists() {
        let result = engine.extract_text_from_file(image_path);
        // SingleChar extraira seulement un caractère
        let _ = result;
    }
}

/// Teste le mode PSM SparseText (Sparse text).
#[test]
fn test_psm_sparse_text() {
    let config = OcrConfig {
        language: "fra".to_string(),
        page_seg_mode: PageSegMode::SparseText,
        dpi: 300,
        tesseract_variables: Default::default(),
    };

    let engine = OcrEngine::new(config).expect("Échec de création avec PSM SparseText");
    let image_path = Path::new("resources/simple/img-4.png");

    if image_path.exists() {
        let result = engine.extract_text_from_file(image_path);
        assert!(
            result.is_ok(),
            "Échec extraction avec SparseText: {:?}",
            result.err()
        );
    }
}

/// Teste le mode PSM SparseTextOsd (Sparse text with OSD).
#[test]
fn test_psm_sparse_text_osd() {
    let config = OcrConfig {
        language: "fra".to_string(),
        page_seg_mode: PageSegMode::SparseTextOsd,
        dpi: 300,
        tesseract_variables: Default::default(),
    };

    let engine = OcrEngine::new(config).expect("Échec de création avec PSM SparseTextOsd");
    let image_path = Path::new("resources/simple/img-4.png");

    if image_path.exists() {
        let result = engine.extract_text_from_file(image_path);
        // Ce mode peut échouer sur certaines images
        let _ = result;
    }
}

/// Teste le mode PSM RawLine (Raw line - bypass all processing).
#[test]
fn test_psm_raw_line() {
    let config = OcrConfig {
        language: "fra".to_string(),
        page_seg_mode: PageSegMode::RawLine,
        dpi: 300,
        tesseract_variables: Default::default(),
    };

    let engine = OcrEngine::new(config).expect("Échec de création avec PSM RawLine");
    let image_path = Path::new("resources/simple/img-4.png");

    if image_path.exists() {
        let result = engine.extract_text_from_file(image_path);
        // RawLine est un mode très spécialisé
        let _ = result;
    }
}

/// Teste la conversion de tous les modes PSM vers les valeurs Tesseract.
#[test]
fn test_all_psm_conversions() {
    let modes = [
        (PageSegMode::OsdOnly, 0),
        (PageSegMode::AutoOsd, 1),
        (PageSegMode::AutoOnly, 2),
        (PageSegMode::Auto, 3),
        (PageSegMode::SingleColumn, 4),
        (PageSegMode::SingleBlockVertText, 5),
        (PageSegMode::SingleBlock, 6),
        (PageSegMode::SingleLine, 7),
        (PageSegMode::SingleWord, 8),
        (PageSegMode::CircleWord, 9),
        (PageSegMode::SingleChar, 10),
        (PageSegMode::SparseText, 11),
        (PageSegMode::SparseTextOsd, 12),
        (PageSegMode::RawLine, 13),
    ];

    for (mode, expected_value) in modes.iter() {
        assert_eq!(
            mode.to_tesseract_psm(),
            *expected_value,
            "Conversion incorrecte pour {:?}",
            mode
        );
    }
}

/// Teste que différents modes PSM donnent des résultats (même si différents).
#[test]
fn test_psm_modes_produce_results() {
    let image_path = Path::new("resources/simple/img-4.png");
    assert!(image_path.exists(), "L'image de test n'existe pas");

    let modes_to_test = [PageSegMode::Auto, PageSegMode::SingleBlock];

    for mode in modes_to_test.iter() {
        let config = OcrConfig {
            language: "fra".to_string(),
            page_seg_mode: *mode,
            dpi: 300,
            tesseract_variables: Default::default(),
        };

        let engine = OcrEngine::new(config)
            .unwrap_or_else(|_| panic!("Échec de création avec mode {:?}", mode));

        let result = engine.extract_text_from_file(image_path);
        assert!(
            result.is_ok(),
            "Échec extraction avec mode {:?}: {:?}",
            mode,
            result.err()
        );

        let text = result.unwrap();
        assert!(
            !text.trim().is_empty(),
            "Aucun texte extrait avec mode {:?}",
            mode
        );
    }
}
