//! Interface en ligne de commande pour l'extraction de texte depuis des images.
//!
//! Ce binaire fournit une CLI simple pour utiliser le moteur OCR
//! et extraire du texte depuis des images en utilisant Tesseract.

use anyhow::{Context, Result};
use clap::Parser;
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use text_recognition::{
    BinarizationMethod, OcrConfig, OcrEngine, PageSegMode, PreprocessingConfig, compare_ocr_result,
    generate_diff_report, load_config,
};

/// Outil d'extraction de texte depuis des images (OCR).
///
/// Utilise Tesseract OCR pour extraire du texte depuis des images.
/// Supporte les formats d'image courants : PNG, JPG, TIFF, etc.
#[derive(Parser, Debug)]
#[command(name = "text-recognition")]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Chemin vers l'image à analyser (ou pattern glob en mode batch)
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

    /// Fichier de configuration JSON ou TOML
    ///
    /// Permet de charger la configuration OCR et/ou de prétraitement depuis
    /// un fichier externe plutôt que de tout passer en arguments CLI.
    /// Les arguments CLI ont priorité sur les valeurs du fichier de configuration.
    ///
    /// Formats supportés : .json, .toml
    ///
    /// Exemple: --config config.toml
    #[arg(long, value_name = "CONFIG_FILE")]
    config: Option<PathBuf>,

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

    /// Mode batch : traiter plusieurs images
    ///
    /// En mode batch, l'argument IMAGE peut être:
    /// - Un répertoire (tous les fichiers images seront traités)
    /// - Un pattern glob (ex: "images/*.png", "**/*.jpg")
    ///
    /// Le texte extrait de chaque image sera affiché avec son nom de fichier.
    /// Compatible avec toutes les autres options (--preprocess, --expected, etc.)
    ///
    /// Exemple: --batch images/
    /// Exemple avec pattern: --batch "resources/**/*.png"
    #[arg(short, long)]
    batch: bool,

    /// Répertoire de sortie pour les résultats batch
    ///
    /// En mode batch, au lieu d'afficher les résultats dans le terminal,
    /// les sauvegarder dans des fichiers .txt dans ce répertoire.
    /// Le nom de fichier sera: <nom_image_sans_extension>.txt
    ///
    /// Exemple: --batch --output results/
    #[arg(short = 'o', long, requires = "batch")]
    output: Option<PathBuf>,
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

/// Collecte les fichiers images depuis un chemin ou un pattern glob.
///
/// Cette fonction gère trois cas :
/// - Un fichier unique : retourne ce fichier
/// - Un répertoire : trouve tous les fichiers images (png, jpg, jpeg, tiff, bmp, gif)
/// - Un pattern glob : résout le pattern et retourne les fichiers correspondants
///
/// # Arguments
///
/// * `path` - Chemin vers fichier, répertoire, ou pattern glob
///
/// # Erreurs
///
/// Retourne une erreur si :
/// - Le chemin n'existe pas (sauf pour les patterns glob)
/// - Aucun fichier image n'est trouvé
fn collect_image_files(path: &Path) -> Result<Vec<PathBuf>> {
    // Vérifier si c'est un pattern glob (contient *, ?, [, etc.)
    let path_str = path.to_string_lossy();
    let is_glob_pattern =
        path_str.contains('*') || path_str.contains('?') || path_str.contains('[');

    if is_glob_pattern {
        // Résoudre le pattern glob
        let mut files = Vec::new();
        for entry in glob::glob(&path_str).context("Pattern glob invalide")? {
            let entry = entry.context("Erreur lors de la résolution du pattern glob")?;
            if entry.is_file() && is_image_file(&entry) {
                files.push(entry);
            }
        }

        if files.is_empty() {
            anyhow::bail!("Aucun fichier image trouvé pour le pattern '{}'", path_str);
        }

        files.sort();
        Ok(files)
    } else if path.is_file() {
        // Un seul fichier
        if !is_image_file(path) {
            anyhow::bail!(
                "Le fichier '{}' n'est pas une image supportée",
                path.display()
            );
        }
        Ok(vec![path.to_path_buf()])
    } else if path.is_dir() {
        // Répertoire : trouver tous les fichiers images
        let mut files = Vec::new();
        for entry in fs::read_dir(path)
            .with_context(|| format!("Impossible de lire le répertoire '{}'", path.display()))?
        {
            let entry = entry.context("Erreur lors de la lecture d'une entrée du répertoire")?;
            let entry_path = entry.path();
            if entry_path.is_file() && is_image_file(&entry_path) {
                files.push(entry_path);
            }
        }

        if files.is_empty() {
            anyhow::bail!(
                "Aucun fichier image trouvé dans le répertoire '{}'",
                path.display()
            );
        }

        files.sort();
        Ok(files)
    } else {
        anyhow::bail!("Le chemin '{}' n'existe pas", path.display());
    }
}

/// Vérifie si un fichier est une image supportée (par extension).
///
/// Extensions supportées : png, jpg, jpeg, tiff, tif, bmp, gif
fn is_image_file(path: &Path) -> bool {
    if let Some(ext) = path.extension() {
        let ext = ext.to_string_lossy().to_lowercase();
        matches!(
            ext.as_str(),
            "png" | "jpg" | "jpeg" | "tiff" | "tif" | "bmp" | "gif"
        )
    } else {
        false
    }
}

/// Traite plusieurs images en mode batch.
///
/// Cette fonction collecte les fichiers images selon le chemin fourni
/// (fichier unique, répertoire, ou pattern glob), puis extrait le texte
/// de chaque image avec la configuration fournie.
///
/// Les résultats peuvent être affichés dans le terminal ou sauvegardés
/// dans des fichiers si un répertoire de sortie est spécifié.
///
/// # Arguments
///
/// * `args` - Arguments de la ligne de commande
/// * `engine` - Moteur OCR configuré
///
/// # Erreurs
///
/// Retourne une erreur si :
/// - Aucun fichier image n'est trouvé
/// - Le répertoire de sortie ne peut pas être créé
/// - Une erreur d'écriture survient
fn process_batch(args: &Args, engine: &OcrEngine) -> Result<()> {
    // Collecter les fichiers images
    let image_files = collect_image_files(&args.image)?;

    println!("═══════════════════════════════════════════════════════════");
    println!("              MODE BATCH - TRAITEMENT MULTIPLE");
    println!("═══════════════════════════════════════════════════════════");
    println!();
    println!("Nombre d'images à traiter: {}", image_files.len());
    println!();

    // Créer le répertoire de sortie si nécessaire
    if let Some(ref output_dir) = args.output {
        fs::create_dir_all(output_dir).with_context(|| {
            format!(
                "Impossible de créer le répertoire de sortie '{}'",
                output_dir.display()
            )
        })?;
        println!("Répertoire de sortie: {}", output_dir.display());
        println!();
    }

    // Statistiques globales
    let mut success_count = 0;
    let mut error_count = 0;

    // Traiter chaque image
    for (index, image_path) in image_files.iter().enumerate() {
        let file_num = index + 1;
        println!("───────────────────────────────────────────────────────────");
        println!(
            "[{}/{}] Traitement: {}",
            file_num,
            image_files.len(),
            image_path.display()
        );
        println!("───────────────────────────────────────────────────────────");

        // Extraire le texte (avec correction d'orientation si demandée)
        let extraction_result = if args.auto_rotate {
            let helper = OcrEngine::new(OcrConfig::default())?;
            let corrected = helper.detect_and_correct_orientation(image_path)?;
            engine.extract_text_from_image(&corrected)
        } else {
            engine.extract_text_from_file(image_path)
        };

        match extraction_result {
            Ok(text) => {
                success_count += 1;

                // Afficher ou sauvegarder le résultat
                if let Some(ref output_dir) = args.output {
                    // Sauvegarder dans un fichier
                    let output_filename = image_path
                        .file_stem()
                        .context("Impossible d'extraire le nom du fichier")?
                        .to_string_lossy()
                        .to_string()
                        + ".txt";
                    let output_path = output_dir.join(output_filename);

                    fs::write(&output_path, &text).with_context(|| {
                        format!(
                            "Impossible d'écrire le fichier de sortie '{}'",
                            output_path.display()
                        )
                    })?;

                    println!("✓ Succès - Résultat sauvegardé: {}", output_path.display());
                } else {
                    // Afficher dans le terminal
                    let trimmed_text = text.trim();
                    if trimmed_text.is_empty() {
                        println!("⚠ Aucun texte extrait");
                    } else {
                        // Limiter l'affichage pour ne pas surcharger
                        let preview = if trimmed_text.len() > 300 {
                            format!(
                                "{}... ({} caractères)",
                                &trimmed_text[..300],
                                trimmed_text.len()
                            )
                        } else {
                            trimmed_text.to_string()
                        };
                        println!("Texte extrait:");
                        println!("{}", preview);
                    }
                    println!("✓ Succès");
                }
            }
            Err(e) => {
                error_count += 1;
                println!("✗ Erreur: {}", e);
            }
        }

        println!();
    }

    // Afficher le résumé
    println!("═══════════════════════════════════════════════════════════");
    println!("                   RÉSUMÉ DU TRAITEMENT");
    println!("═══════════════════════════════════════════════════════════");
    println!("Total:     {} images", image_files.len());
    println!("Succès:    {} images", success_count);
    println!("Erreurs:   {} images", error_count);
    println!(
        "Taux de réussite: {:.1}%",
        (success_count as f64 / image_files.len() as f64) * 100.0
    );
    println!("═══════════════════════════════════════════════════════════");

    if error_count > 0 {
        anyhow::bail!("{} image(s) n'ont pas pu être traitées", error_count);
    }

    Ok(())
}

fn main() -> Result<()> {
    // Parser les arguments de la ligne de commande
    let args = Args::parse();

    // Mode spécial: tester tous les PSM
    if args.test_all_psm {
        return test_all_psm_modes(&args);
    }

    // Mode batch : traiter plusieurs images
    if args.batch {
        // Validation: en mode batch, --expected et --metrics ne sont pas supportés
        if args.expected.is_some() {
            anyhow::bail!(
                "L'option --expected n'est pas supportée en mode batch. \
                 Traitez les images individuellement pour comparer avec des textes de référence."
            );
        }
    }

    // Charger la configuration depuis un fichier si --config est fourni
    let file_config = if let Some(ref config_path) = args.config {
        let app_config = load_config(config_path)
            .with_context(|| format!("Impossible de charger '{}'", config_path.display()))?;
        Some(app_config)
    } else {
        None
    };

    // Créer la configuration OCR (fichier de config en base, arguments CLI en surcharge)
    let config = {
        let base = file_config
            .as_ref()
            .and_then(|c| c.ocr.clone())
            .unwrap_or_default();

        OcrConfig {
            // Les arguments CLI ont priorité sur le fichier (valeurs non-défaut)
            language: if args.language != "fra" {
                args.language.clone()
            } else {
                base.language
            },
            page_seg_mode: if args.psm != 3 {
                psm_from_int(args.psm)
            } else {
                base.page_seg_mode
            },
            dpi: if args.dpi != 300 { args.dpi } else { base.dpi },
            tesseract_variables: base.tesseract_variables,
        }
    };

    // Créer le moteur OCR avec ou sans prétraitement
    let engine = if args.batch {
        // En mode batch, créer le moteur une seule fois et le réutiliser
        if args.preprocess {
            let base_prep = file_config
                .as_ref()
                .and_then(|c| c.preprocessing.clone())
                .unwrap_or_default();

            let binarization_method = parse_binarization_method(&args.binarize_method)?;

            let preprocess_config = PreprocessingConfig {
                to_grayscale: args.grayscale || base_prep.to_grayscale,
                binarize: args.binarize || base_prep.binarize,
                binarization_method: if args.binarize {
                    binarization_method
                } else {
                    base_prep.binarization_method
                },
                adjust_contrast: args.contrast.is_some() || base_prep.adjust_contrast,
                contrast_factor: args.contrast.unwrap_or(base_prep.contrast_factor),
                denoise: args.denoise || base_prep.denoise,
                deskew: args.deskew || base_prep.deskew,
            };

            OcrEngine::with_preprocessing(config, preprocess_config)?
        } else if let Some(prep) = file_config.as_ref().and_then(|c| c.preprocessing.clone()) {
            OcrEngine::with_preprocessing(config, prep)?
        } else {
            OcrEngine::new(config)?
        }
    } else if args.preprocess {
        // Construire la configuration de prétraitement (fichier en base, CLI en surcharge)
        let base_prep = file_config
            .as_ref()
            .and_then(|c| c.preprocessing.clone())
            .unwrap_or_default();

        let binarization_method = parse_binarization_method(&args.binarize_method)?;

        let preprocess_config = PreprocessingConfig {
            to_grayscale: args.grayscale || base_prep.to_grayscale,
            binarize: args.binarize || base_prep.binarize,
            binarization_method: if args.binarize {
                binarization_method
            } else {
                base_prep.binarization_method
            },
            adjust_contrast: args.contrast.is_some() || base_prep.adjust_contrast,
            contrast_factor: args.contrast.unwrap_or(base_prep.contrast_factor),
            denoise: args.denoise || base_prep.denoise,
            deskew: args.deskew || base_prep.deskew,
        };

        OcrEngine::with_preprocessing(config, preprocess_config)?
    } else if let Some(prep) = file_config.as_ref().and_then(|c| c.preprocessing.clone()) {
        // Pas de --preprocess CLI mais le fichier de config a une section preprocessing
        OcrEngine::with_preprocessing(config, prep)?
    } else {
        OcrEngine::new(config)?
    };

    // En mode batch, traiter toutes les images et terminer
    if args.batch {
        return process_batch(&args, &engine);
    }

    // Mode normal: traiter une seule image

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
