//! Interface en ligne de commande pour l'extraction de texte depuis des images.
//!
//! Ce binaire fournit une CLI simple pour utiliser le moteur OCR
//! et extraire du texte depuis des images en utilisant Tesseract.

use anyhow::Result;
use clap::Parser;
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use text_recognition::{
    BinarizationMethod, OcrConfig, OcrEngine, PageSegMode, PreprocessingConfig, compare_ocr_result,
    generate_diff_report,
};

/// Outil d'extraction de texte depuis des images (OCR).
///
/// Utilise Tesseract OCR pour extraire du texte depuis des images.
/// Supporte les formats d'image courants : PNG, JPG, TIFF, etc.
#[derive(Parser, Debug)]
#[command(name = "text-recognition")]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Chemin vers l'image à analyser
    #[arg(value_name = "IMAGE")]
    image: PathBuf,

    /// Langue pour l'OCR
    ///
    /// Langues courantes:
    ///   fra = Français
    ///   eng = Anglais
    ///   deu = Allemand
    ///   spa = Espagnol
    ///   ita = Italien
    ///   por = Portugais
    ///
    /// Plusieurs langues peuvent être combinées avec '+' (ex: "eng+fra")
    ///
    /// Note: Les données linguistiques doivent être installées sur le système.
    /// Sur Debian/Ubuntu: apt-get install tesseract-ocr-fra tesseract-ocr-eng
    #[arg(short, long, alias = "lang", default_value = "fra")]
    language: String,

    /// Mode de segmentation de page (PSM: 0-13)
    ///
    /// Modes disponibles:
    ///   0 = OSD uniquement (orientation/script detection)
    ///   1 = Auto avec OSD
    ///   2 = Auto sans OSD
    ///   3 = Auto (par défaut)
    ///   4 = Colonne unique
    ///   5 = Bloc vertical unique
    ///   6 = Bloc unique
    ///   7 = Ligne unique
    ///   8 = Mot unique
    ///   9 = Mot dans un cercle
    ///  10 = Caractère unique
    ///  11 = Texte épars
    ///  12 = Texte épars avec OSD
    ///  13 = Ligne brute
    #[arg(short = 'p', long, default_value_t = 3, value_parser = clap::value_parser!(i32).range(0..=13))]
    psm: i32,

    /// Résolution DPI de l'image
    #[arg(short, long, default_value_t = 300)]
    dpi: u32,

    /// Activer le prétraitement d'image
    ///
    /// Le prétraitement peut améliorer la qualité OCR en appliquant diverses
    /// transformations à l'image avant l'extraction de texte.
    #[arg(long)]
    preprocess: bool,

    /// Convertir en niveaux de gris (prétraitement)
    #[arg(long, requires = "preprocess")]
    grayscale: bool,

    /// Appliquer la binarisation (prétraitement)
    ///
    /// Convertit l'image en noir et blanc pur (0 ou 255).
    #[arg(long, requires = "preprocess")]
    binarize: bool,

    /// Méthode de binarisation: otsu, fixed, adaptive
    ///
    /// - otsu: Calcul automatique du seuil optimal (recommandé)
    /// - fixed:SEUIL: Seuil fixe (ex: fixed:128)
    /// - adaptive: Seuil adaptatif local
    #[arg(long, default_value = "otsu", requires = "binarize")]
    binarize_method: String,

    /// Appliquer un débruitage (filtre médian 3x3)
    #[arg(long, requires = "preprocess")]
    denoise: bool,

    /// Ajuster le contraste
    ///
    /// Facteur de contraste (1.0 = pas de changement, >1.0 = augmentation).
    /// Exemple: --contrast 1.5
    #[arg(long, requires = "preprocess")]
    contrast: Option<f32>,

    /// Corriger l'inclinaison du document (deskew)
    ///
    /// Détecte et corrige les inclinaisons légères (-20° à +20°) par analyse
    /// de la projection horizontale. Pour les rotations à 90°/180°/270°,
    /// utiliser --auto-rotate.
    #[arg(long, requires = "preprocess")]
    deskew: bool,

    /// Corriger automatiquement l'orientation de l'image
    ///
    /// Utilise Tesseract (PSM 0) pour détecter l'orientation réelle de l'image
    /// (0°, 90°, 180°, 270°) et applique la rotation nécessaire avant l'OCR.
    /// Utile pour les images à l'envers ou pivotées de 90°/270°.
    ///
    /// Compatible avec --preprocess pour combiner correction d'orientation
    /// et prétraitement d'image.
    #[arg(long)]
    auto_rotate: bool,

    /// Fichier contenant le texte de référence attendu
    ///
    /// Si fourni, le programme comparera le résultat OCR avec ce texte
    /// et affichera les métriques de qualité (CER, WER, etc.) au lieu
    /// du texte extrait.
    ///
    /// Exemple: --expected expected_text.txt
    #[arg(short = 'e', long)]
    expected: Option<PathBuf>,

    /// Afficher un rapport détaillé des métriques
    ///
    /// Nécessite l'option --expected. Affiche un rapport complet formaté
    /// incluant les métriques, statistiques, et comparaison des textes.
    ///
    /// Sans cette option, seules les métriques essentielles sont affichées.
    ///
    /// Exemple: --expected expected.txt --metrics
    #[arg(short = 'm', long, requires = "expected")]
    metrics: bool,

    /// Tester tous les modes PSM (0-13) et afficher les résultats
    ///
    /// Cette option teste tous les 14 modes de segmentation de page disponibles
    /// et affiche le texte extrait pour chacun. Si --expected est fourni,
    /// affiche également les métriques de qualité pour chaque mode.
    ///
    /// Utile pour déterminer quel mode PSM donne les meilleurs résultats
    /// pour un type d'image spécifique.
    ///
    /// Note: Cette option ignore l'option --psm.
    ///
    /// Exemple: --test-all-psm
    /// Exemple avec métriques: --test-all-psm --expected expected.txt
    #[arg(long, conflicts_with = "psm")]
    test_all_psm: bool,
}

/// Convertit un code PSM numérique en PageSegMode.
fn psm_from_int(psm: i32) -> PageSegMode {
    match psm {
        0 => PageSegMode::OsdOnly,
        1 => PageSegMode::AutoOsd,
        2 => PageSegMode::AutoOnly,
        3 => PageSegMode::Auto,
        4 => PageSegMode::SingleColumn,
        5 => PageSegMode::SingleBlockVertText,
        6 => PageSegMode::SingleBlock,
        7 => PageSegMode::SingleLine,
        8 => PageSegMode::SingleWord,
        9 => PageSegMode::CircleWord,
        10 => PageSegMode::SingleChar,
        11 => PageSegMode::SparseText,
        12 => PageSegMode::SparseTextOsd,
        13 => PageSegMode::RawLine,
        _ => PageSegMode::Auto, // Fallback (ne devrait jamais arriver grâce au value_parser)
    }
}

/// Parse la méthode de binarisation depuis une chaîne.
///
/// Formats supportés:
/// - "otsu" -> BinarizationMethod::Otsu
/// - "fixed:128" -> BinarizationMethod::Fixed(128)
/// - "adaptive" -> BinarizationMethod::Adaptive
fn parse_binarization_method(method: &str) -> Result<BinarizationMethod> {
    if method == "otsu" {
        Ok(BinarizationMethod::Otsu)
    } else if method == "adaptive" {
        Ok(BinarizationMethod::Adaptive)
    } else if let Some(threshold_str) = method.strip_prefix("fixed:") {
        let threshold = threshold_str.parse::<u8>().map_err(|_| {
            anyhow::anyhow!(
                "Seuil invalide: '{}'. Doit être entre 0 et 255",
                threshold_str
            )
        })?;
        Ok(BinarizationMethod::Fixed(threshold))
    } else {
        anyhow::bail!(
            "Méthode de binarisation invalide: '{}'. Utilisez 'otsu', 'adaptive', ou 'fixed:SEUIL'",
            method
        )
    }
}

/// Teste tous les modes PSM (0-13) sur une image et affiche les résultats.
///
/// Cette fonction itère sur tous les modes de segmentation de page disponibles,
/// extrait le texte avec chaque mode, et affiche les résultats.
///
/// Si un fichier de référence est fourni (--expected), affiche également
/// les métriques de qualité (CER, WER) pour chaque mode.
fn test_all_psm_modes(args: &Args) -> Result<()> {
    println!("═══════════════════════════════════════════════════════════");
    println!("         TEST DE TOUS LES MODES PSM (0-13)");
    println!("═══════════════════════════════════════════════════════════");
    println!();
    println!("Image: {}", args.image.display());
    println!("Langue: {}", args.language);
    println!("DPI: {}", args.dpi);
    println!();

    // Charger le texte de référence si fourni
    let expected_text = if let Some(expected_path) = &args.expected {
        let text = fs::read_to_string(expected_path).map_err(|e| {
            anyhow::anyhow!(
                "Impossible de lire le fichier de référence '{}': {}",
                expected_path.display(),
                e
            )
        })?;
        println!("Texte de référence chargé: {} caractères", text.len());
        println!();
        Some(text)
    } else {
        None
    };

    // Liste de tous les modes PSM avec leurs noms
    let all_psm_modes = [
        (0, "OSD Only", PageSegMode::OsdOnly),
        (1, "Auto with OSD", PageSegMode::AutoOsd),
        (2, "Auto without OSD", PageSegMode::AutoOnly),
        (3, "Auto (default)", PageSegMode::Auto),
        (4, "Single column", PageSegMode::SingleColumn),
        (5, "Single vertical block", PageSegMode::SingleBlockVertText),
        (6, "Single block", PageSegMode::SingleBlock),
        (7, "Single line", PageSegMode::SingleLine),
        (8, "Single word", PageSegMode::SingleWord),
        (9, "Circle word", PageSegMode::CircleWord),
        (10, "Single char", PageSegMode::SingleChar),
        (11, "Sparse text", PageSegMode::SparseText),
        (12, "Sparse text with OSD", PageSegMode::SparseTextOsd),
        (13, "Raw line", PageSegMode::RawLine),
    ];

    // Construire la configuration de prétraitement si nécessaire
    let preprocess_config = if args.preprocess {
        let binarization_method = parse_binarization_method(&args.binarize_method)?;
        Some(PreprocessingConfig {
            to_grayscale: args.grayscale,
            binarize: args.binarize,
            binarization_method,
            adjust_contrast: args.contrast.is_some(),
            contrast_factor: args.contrast.unwrap_or(1.0),
            denoise: args.denoise,
            deskew: args.deskew,
        })
    } else {
        None
    };

    // Tester chaque mode PSM
    for (psm_num, psm_name, psm_mode) in &all_psm_modes {
        println!("───────────────────────────────────────────────────────────");
        println!("PSM {} - {}", psm_num, psm_name);
        println!("───────────────────────────────────────────────────────────");

        // Créer la configuration avec le PSM actuel
        let config = OcrConfig {
            language: args.language.clone(),
            page_seg_mode: *psm_mode,
            dpi: args.dpi,
            tesseract_variables: HashMap::new(),
        };

        // Créer le moteur OCR
        let engine = if let Some(ref prep_config) = preprocess_config {
            OcrEngine::with_preprocessing(config, prep_config.clone())?
        } else {
            OcrEngine::new(config)?
        };

        // Extraire le texte (avec correction d'orientation si demandée)
        let extraction_result = if args.auto_rotate {
            let helper = OcrEngine::new(OcrConfig::default())?;
            let corrected = helper.detect_and_correct_orientation(&args.image)?;
            engine.extract_text_from_image(&corrected)
        } else {
            engine.extract_text_from_file(&args.image)
        };

        match extraction_result {
            Ok(text) => {
                // Afficher le texte extrait
                let trimmed_text = text.trim();

                if trimmed_text.is_empty() {
                    println!("⚠ Aucun texte extrait");
                } else {
                    // Limiter l'affichage pour ne pas surcharger le terminal
                    let preview = if trimmed_text.len() > 200 {
                        format!(
                            "{}... ({} caractères)",
                            &trimmed_text[..200],
                            trimmed_text.len()
                        )
                    } else {
                        trimmed_text.to_string()
                    };
                    println!("Texte extrait:");
                    println!("{}", preview);
                }

                // Si un texte de référence est fourni, calculer les métriques
                if let Some(ref expected) = expected_text {
                    let metrics = compare_ocr_result(&text, expected);
                    println!();
                    println!("Métriques:");
                    println!("  CER:       {:.2}%", metrics.cer * 100.0);
                    println!("  WER:       {:.2}%", metrics.wer * 100.0);
                    println!("  Précision: {:.2}%", metrics.accuracy() * 100.0);

                    // Indicateur visuel de qualité
                    let quality = if metrics.cer < 0.05 {
                        "★★★★★ Excellent"
                    } else if metrics.cer < 0.15 {
                        "★★★★☆ Bon"
                    } else if metrics.cer < 0.30 {
                        "★★★☆☆ Moyen"
                    } else if metrics.cer < 0.50 {
                        "★★☆☆☆ Faible"
                    } else {
                        "★☆☆☆☆ Très faible"
                    };
                    println!("  Qualité:   {}", quality);
                }
            }
            Err(e) => {
                println!("✗ Erreur lors de l'extraction: {}", e);
            }
        }

        println!();
    }

    println!("═══════════════════════════════════════════════════════════");
    println!("Test terminé. {} modes testés.", all_psm_modes.len());
    println!("═══════════════════════════════════════════════════════════");

    Ok(())
}

fn main() -> Result<()> {
    // Parser les arguments de la ligne de commande
    let args = Args::parse();

    // Mode spécial: tester tous les PSM
    if args.test_all_psm {
        return test_all_psm_modes(&args);
    }

    // Créer la configuration OCR
    let config = OcrConfig {
        language: args.language,
        page_seg_mode: psm_from_int(args.psm),
        dpi: args.dpi,
        tesseract_variables: HashMap::new(),
    };

    // Créer le moteur OCR avec ou sans prétraitement
    let engine = if args.preprocess {
        // Construire la configuration de prétraitement
        let binarization_method = parse_binarization_method(&args.binarize_method)?;

        let preprocess_config = PreprocessingConfig {
            to_grayscale: args.grayscale,
            binarize: args.binarize,
            binarization_method,
            adjust_contrast: args.contrast.is_some(),
            contrast_factor: args.contrast.unwrap_or(1.0),
            denoise: args.denoise,
            deskew: args.deskew,
        };

        OcrEngine::with_preprocessing(config, preprocess_config)?
    } else {
        OcrEngine::new(config)?
    };

    // Extraire le texte (avec correction d'orientation si demandée)
    let text = if args.auto_rotate {
        // Détecter et corriger l'orientation via Tesseract PSM 0
        let helper = OcrEngine::new(OcrConfig::default())?;
        let corrected_image = helper.detect_and_correct_orientation(&args.image)?;
        engine.extract_text_from_image(&corrected_image)?
    } else {
        engine.extract_text_from_file(&args.image)?
    };

    // Si un fichier de référence est fourni, comparer et afficher les métriques
    if let Some(expected_path) = args.expected {
        let expected_text = fs::read_to_string(&expected_path).map_err(|e| {
            anyhow::anyhow!(
                "Impossible de lire le fichier de référence '{}': {}",
                expected_path.display(),
                e
            )
        })?;

        // Afficher les métriques (format détaillé ou simple)
        if args.metrics {
            // Rapport détaillé avec generate_diff_report()
            let report = generate_diff_report(&text, &expected_text);
            println!("{}", report);
        } else {
            // Affichage simple des métriques essentielles
            let metrics = compare_ocr_result(&text, &expected_text);

            println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
            println!("               RÉSULTATS DE LA COMPARAISON OCR");
            println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
            println!();
            println!("MÉTRIQUES:");
            println!(
                "  • CER (Character Error Rate):  {:.2}%",
                metrics.cer * 100.0
            );
            println!(
                "  • WER (Word Error Rate):       {:.2}%",
                metrics.wer * 100.0
            );
            println!(
                "  • Distance de Levenshtein:     {}",
                metrics.levenshtein_distance
            );
            println!(
                "  • Précision:                   {:.2}%",
                metrics.accuracy() * 100.0
            );
            println!();
            println!("STATISTIQUES:");
            println!(
                "  • Référence:  {} caractères, {} mots",
                metrics.reference_char_count, metrics.reference_word_count
            );
            println!(
                "  • OCR:        {} caractères, {} mots",
                metrics.ocr_char_count, metrics.ocr_word_count
            );
            println!();
            println!(
                "  • Match exact: {}",
                if metrics.exact_match {
                    "Oui ✓"
                } else {
                    "Non ✗"
                }
            );
            println!();
            println!("━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━");
        }
    } else {
        // Afficher le résultat normalement
        println!("{}", text);
    }

    Ok(())
}
