//! Interface en ligne de commande pour l'extraction de texte depuis des images.
//!
//! Ce binaire fournit une CLI simple pour utiliser le moteur OCR
//! et extraire du texte depuis des images en utilisant Tesseract.

use anyhow::Result;
use clap::Parser;
use std::collections::HashMap;
use std::path::PathBuf;
use text_recognition::{OcrConfig, OcrEngine, PageSegMode};

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

    /// Langue pour l'OCR (ex: "fra", "eng", "eng+fra")
    #[arg(short, long, default_value = "fra")]
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

fn main() -> Result<()> {
    // Parser les arguments de la ligne de commande
    let args = Args::parse();

    // Créer la configuration OCR
    let config = OcrConfig {
        language: args.language,
        page_seg_mode: psm_from_int(args.psm),
        dpi: args.dpi,
        tesseract_variables: HashMap::new(),
    };

    // Créer le moteur OCR
    let engine = OcrEngine::new(config)?;

    // Extraire le texte
    let text = engine.extract_text_from_file(&args.image)?;

    // Afficher le résultat
    println!("{}", text);

    Ok(())
}
