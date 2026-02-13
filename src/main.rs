//! Interface en ligne de commande pour l'extraction de texte depuis des images.
//!
//! Ce binaire fournit une CLI simple pour utiliser le moteur OCR
//! et extraire du texte depuis des images en utilisant Tesseract.

use anyhow::Result;
use clap::Parser;
use std::collections::HashMap;
use std::path::PathBuf;
use text_recognition::{OcrConfig, OcrEngine};

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

    /// Résolution DPI de l'image
    #[arg(short, long, default_value_t = 300)]
    dpi: u32,
}

fn main() -> Result<()> {
    // Parser les arguments de la ligne de commande
    let args = Args::parse();

    // Créer la configuration OCR
    let config = OcrConfig {
        language: args.language,
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
