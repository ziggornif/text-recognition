//! Module pour l'extraction et la visualisation des bounding boxes au format HOCR.
//!
//! Ce module fournit des structures et méthodes pour extraire les bounding boxes
//! (rectangles délimitant les mots, lignes, paragraphes, etc.) depuis Tesseract
//! au format HOCR (HTML with OCR).

use anyhow::{Context, Result};
use std::path::Path;
use std::process::Command;

/// Représente un rectangle délimitant (bounding box).
///
/// Les coordonnées sont exprimées en pixels depuis le coin supérieur gauche de l'image.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BBox {
    /// Coordonnée X du coin supérieur gauche.
    pub x: u32,
    /// Coordonnée Y du coin supérieur gauche.
    pub y: u32,
    /// Largeur du rectangle.
    pub width: u32,
    /// Hauteur du rectangle.
    pub height: u32,
}

impl BBox {
    /// Crée un nouveau bounding box.
    ///
    /// # Arguments
    ///
    /// * `x` - Coordonnée X du coin supérieur gauche
    /// * `y` - Coordonnée Y du coin supérieur gauche
    /// * `width` - Largeur du rectangle
    /// * `height` - Hauteur du rectangle
    pub fn new(x: u32, y: u32, width: u32, height: u32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    /// Parse un bounding box depuis une chaîne HOCR "bbox x0 y0 x1 y1".
    ///
    /// # Arguments
    ///
    /// * `bbox_str` - Chaîne au format "bbox x0 y0 x1 y1"
    ///
    /// # Exemple
    ///
    /// ```
    /// use text_recognition::hocr::BBox;
    ///
    /// let bbox = BBox::from_hocr_string("bbox 100 200 300 400").unwrap();
    /// assert_eq!(bbox.x, 100);
    /// assert_eq!(bbox.y, 200);
    /// assert_eq!(bbox.width, 200);
    /// assert_eq!(bbox.height, 200);
    /// ```
    pub fn from_hocr_string(bbox_str: &str) -> Result<Self> {
        let parts: Vec<&str> = bbox_str.split_whitespace().collect();
        if parts.len() != 5 || parts[0] != "bbox" {
            anyhow::bail!("Format HOCR bbox invalide: {}", bbox_str);
        }

        let x0: u32 = parts[1].parse().context("X0 invalide dans bbox")?;
        let y0: u32 = parts[2].parse().context("Y0 invalide dans bbox")?;
        let x1: u32 = parts[3].parse().context("X1 invalide dans bbox")?;
        let y1: u32 = parts[4].parse().context("Y1 invalide dans bbox")?;

        let width = x1.saturating_sub(x0);
        let height = y1.saturating_sub(y0);

        Ok(Self::new(x0, y0, width, height))
    }
}

/// Représente un mot avec son bounding box et son texte.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HocrWord {
    /// Rectangle délimitant le mot.
    pub bbox: BBox,
    /// Texte du mot.
    pub text: String,
    /// Niveau de confiance de la reconnaissance (0-100).
    pub confidence: Option<u8>,
}

impl HocrWord {
    /// Crée un nouveau mot HOCR.
    ///
    /// # Arguments
    ///
    /// * `bbox` - Rectangle délimitant le mot
    /// * `text` - Texte du mot
    /// * `confidence` - Niveau de confiance optionnel (0-100)
    pub fn new(bbox: BBox, text: String, confidence: Option<u8>) -> Self {
        Self {
            bbox,
            text,
            confidence,
        }
    }
}

/// Représente une ligne de texte avec son bounding box et ses mots.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HocrLine {
    /// Rectangle délimitant la ligne.
    pub bbox: BBox,
    /// Mots composant la ligne.
    pub words: Vec<HocrWord>,
}

impl HocrLine {
    /// Crée une nouvelle ligne HOCR.
    ///
    /// # Arguments
    ///
    /// * `bbox` - Rectangle délimitant la ligne
    pub fn new(bbox: BBox) -> Self {
        Self {
            bbox,
            words: Vec::new(),
        }
    }

    /// Ajoute un mot à la ligne.
    ///
    /// # Arguments
    ///
    /// * `word` - Mot à ajouter
    pub fn add_word(&mut self, word: HocrWord) {
        self.words.push(word);
    }
}

/// Représente un paragraphe avec son bounding box et ses lignes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HocrParagraph {
    /// Rectangle délimitant le paragraphe.
    pub bbox: BBox,
    /// Lignes composant le paragraphe.
    pub lines: Vec<HocrLine>,
}

impl HocrParagraph {
    /// Crée un nouveau paragraphe HOCR.
    ///
    /// # Arguments
    ///
    /// * `bbox` - Rectangle délimitant le paragraphe
    pub fn new(bbox: BBox) -> Self {
        Self {
            bbox,
            lines: Vec::new(),
        }
    }

    /// Ajoute une ligne au paragraphe.
    ///
    /// # Arguments
    ///
    /// * `line` - Ligne à ajouter
    pub fn add_line(&mut self, line: HocrLine) {
        self.lines.push(line);
    }
}

/// Représente un document HOCR complet.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct HocrDocument {
    /// Paragraphes du document.
    pub paragraphs: Vec<HocrParagraph>,
}

impl HocrDocument {
    /// Crée un nouveau document HOCR vide.
    pub fn new() -> Self {
        Self {
            paragraphs: Vec::new(),
        }
    }

    /// Ajoute un paragraphe au document.
    ///
    /// # Arguments
    ///
    /// * `paragraph` - Paragraphe à ajouter
    pub fn add_paragraph(&mut self, paragraph: HocrParagraph) {
        self.paragraphs.push(paragraph);
    }

    /// Parse un document HOCR depuis une chaîne XML/HTML.
    ///
    /// Cette méthode parse le contenu HOCR généré par Tesseract et extrait
    /// tous les bounding boxes et textes des mots, lignes et paragraphes.
    ///
    /// # Arguments
    ///
    /// * `hocr_content` - Contenu HOCR au format XML/HTML
    ///
    /// # Exemple
    ///
    /// ```no_run
    /// use text_recognition::hocr::HocrDocument;
    ///
    /// let hocr_html = r#"<html>...</html>"#;
    /// let doc = HocrDocument::from_hocr_string(hocr_html).unwrap();
    /// println!("Trouvé {} paragraphes", doc.paragraphs.len());
    /// ```
    pub fn from_hocr_string(hocr_content: &str) -> Result<Self> {
        let mut doc = HocrDocument::new();

        // Parser simple basé sur regex
        // Note: Pour une production robuste, il faudrait utiliser un parser XML/HTML
        // comme `scraper` ou `html5ever`, mais pour l'apprentissage, un parser simple suffit.

        let mut current_paragraph: Option<HocrParagraph> = None;
        let mut current_line: Option<HocrLine> = None;

        for line in hocr_content.lines() {
            let trimmed = line.trim();

            // Détecter les paragraphes
            if trimmed.contains("class='ocr_par'") || trimmed.contains("class=\"ocr_par\"") {
                // Sauvegarder le paragraphe précédent s'il existe
                if let Some(para) = current_paragraph.take() {
                    doc.add_paragraph(para);
                }

                // Extraire le bbox du paragraphe
                if let Some(bbox) = extract_bbox(trimmed) {
                    current_paragraph = Some(HocrParagraph::new(bbox));
                }
            }
            // Détecter les lignes
            else if trimmed.contains("class='ocr_line'") || trimmed.contains("class=\"ocr_line\"")
            {
                // Sauvegarder la ligne précédente s'il existe
                if let Some(line_obj) = current_line.take()
                    && let Some(ref mut para) = current_paragraph
                {
                    para.add_line(line_obj);
                }

                // Extraire le bbox de la ligne
                if let Some(bbox) = extract_bbox(trimmed) {
                    current_line = Some(HocrLine::new(bbox));
                }
            }
            // Détecter les mots
            else if (trimmed.contains("class='ocrx_word'")
                || trimmed.contains("class=\"ocrx_word\""))
                && let Some(bbox) = extract_bbox(trimmed)
                && let Some(text) = extract_word_text(trimmed)
            {
                // Extraire la confiance optionnelle
                let confidence = extract_confidence(trimmed);

                let word = HocrWord::new(bbox, text, confidence);

                if let Some(ref mut line_obj) = current_line {
                    line_obj.add_word(word);
                }
            }
        }

        // Sauvegarder les derniers éléments
        if let Some(line_obj) = current_line
            && let Some(ref mut para) = current_paragraph
        {
            para.add_line(line_obj);
        }
        if let Some(para) = current_paragraph {
            doc.add_paragraph(para);
        }

        Ok(doc)
    }

    /// Génère un rapport texte listant tous les bounding boxes.
    ///
    /// # Exemple
    ///
    /// ```no_run
    /// use text_recognition::hocr::HocrDocument;
    ///
    /// let doc = HocrDocument::new();
    /// let report = doc.generate_report();
    /// println!("{}", report);
    /// ```
    pub fn generate_report(&self) -> String {
        let mut report = String::new();

        report.push_str("=== RAPPORT HOCR - BOUNDING BOXES ===\n\n");

        for (para_idx, para) in self.paragraphs.iter().enumerate() {
            report.push_str(&format!(
                "Paragraphe #{}: bbox({}, {}, {}, {})\n",
                para_idx + 1,
                para.bbox.x,
                para.bbox.y,
                para.bbox.width,
                para.bbox.height
            ));

            for (line_idx, line) in para.lines.iter().enumerate() {
                report.push_str(&format!(
                    "  Ligne #{}: bbox({}, {}, {}, {})\n",
                    line_idx + 1,
                    line.bbox.x,
                    line.bbox.y,
                    line.bbox.width,
                    line.bbox.height
                ));

                for (word_idx, word) in line.words.iter().enumerate() {
                    let conf_str = word
                        .confidence
                        .map(|c| format!(" [confiance: {}%]", c))
                        .unwrap_or_default();

                    report.push_str(&format!(
                        "    Mot #{}: \"{}\" bbox({}, {}, {}, {}){}",
                        word_idx + 1,
                        word.text,
                        word.bbox.x,
                        word.bbox.y,
                        word.bbox.width,
                        word.bbox.height,
                        conf_str
                    ));
                    report.push('\n');
                }
            }

            report.push('\n');
        }

        report
    }
}

impl Default for HocrDocument {
    fn default() -> Self {
        Self::new()
    }
}

/// Extrait un bounding box depuis une ligne HOCR.
///
/// # Arguments
///
/// * `line` - Ligne HTML contenant un attribut title avec bbox
fn extract_bbox(line: &str) -> Option<BBox> {
    // Chercher "title='bbox ..." ou "title=\"bbox ..."
    let start_idx = line.find("title=")?;
    let rest = &line[start_idx + 6..]; // Sauter "title="

    let quote_char = rest.chars().next()?;
    let end_idx = rest[1..].find(quote_char)?;
    let title_content = &rest[1..=end_idx];

    // Chercher "bbox x0 y0 x1 y1"
    if let Some(bbox_start) = title_content.find("bbox ") {
        let bbox_str = &title_content[bbox_start..];
        let bbox_end = bbox_str.find(';').unwrap_or(bbox_str.len());
        let bbox_values = &bbox_str[..bbox_end];

        BBox::from_hocr_string(bbox_values).ok()
    } else {
        None
    }
}

/// Extrait le texte d'un mot depuis une ligne HOCR.
///
/// # Arguments
///
/// * `line` - Ligne HTML contenant le mot
fn extract_word_text(line: &str) -> Option<String> {
    // Chercher le contenu entre > et </span>
    let start_idx = line.find('>')? + 1;
    let end_idx = line.find("</span>")?;

    if start_idx < end_idx {
        let text = line[start_idx..end_idx].trim();
        Some(text.to_string())
    } else {
        None
    }
}

/// Extrait le niveau de confiance depuis une ligne HOCR.
///
/// # Arguments
///
/// * `line` - Ligne HTML contenant l'attribut title avec x_wconf
fn extract_confidence(line: &str) -> Option<u8> {
    // Chercher "x_wconf N" dans l'attribut title
    let start_idx = line.find("x_wconf ")?;
    let rest = &line[start_idx + 8..];
    let end_idx = rest
        .find(|c: char| !c.is_ascii_digit())
        .unwrap_or(rest.len());
    let conf_str = &rest[..end_idx];

    conf_str.parse::<u8>().ok()
}

/// Génère un fichier HOCR depuis une image en utilisant le binaire Tesseract.
///
/// Cette fonction appelle directement le binaire `tesseract` en ligne de commande
/// pour générer la sortie HOCR, qui contient tous les bounding boxes et le texte.
///
/// # Arguments
///
/// * `image_path` - Chemin vers l'image à analyser
/// * `language` - Code langue Tesseract (ex: "fra", "eng")
/// * `psm` - Mode de segmentation de page (0-13)
///
/// # Exemple
///
/// ```no_run
/// use text_recognition::hocr::generate_hocr;
/// use std::path::Path;
///
/// let hocr = generate_hocr(Path::new("image.png"), "eng", 3).unwrap();
/// println!("HOCR généré: {} octets", hocr.len());
/// ```
///
/// # Erreurs
///
/// Retourne une erreur si :
/// - Le binaire `tesseract` n'est pas installé ou introuvable
/// - Le fichier image n'existe pas ou est illisible
/// - La génération HOCR échoue
pub fn generate_hocr(image_path: &Path, language: &str, psm: u8) -> Result<String> {
    let path_str = image_path.to_str().context("Chemin invalide")?;

    // Créer un répertoire temporaire pour la sortie
    let temp_dir = tempfile::tempdir().context("Échec de création du répertoire temporaire")?;
    let output_base = temp_dir.path().join("output");
    let output_base_str = output_base.to_str().context("Chemin temporaire invalide")?;

    // Appeler tesseract avec l'option hocr
    let status = Command::new("tesseract")
        .args([
            path_str,
            output_base_str,
            "-l",
            language,
            "--psm",
            &psm.to_string(),
            "hocr",
        ])
        .status()
        .context("Impossible de lancer le binaire tesseract")?;

    if !status.success() {
        anyhow::bail!("Tesseract a échoué lors de la génération HOCR");
    }

    // Lire le fichier HOCR généré (extension .hocr)
    let hocr_path = temp_dir.path().join("output.hocr");
    let hocr_content = std::fs::read_to_string(&hocr_path)
        .context("Échec de la lecture du fichier HOCR généré")?;

    Ok(hocr_content)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bbox_from_hocr_string() {
        let bbox = BBox::from_hocr_string("bbox 100 200 300 400").unwrap();
        assert_eq!(bbox.x, 100);
        assert_eq!(bbox.y, 200);
        assert_eq!(bbox.width, 200);
        assert_eq!(bbox.height, 200);
    }

    #[test]
    fn test_bbox_from_hocr_string_invalid() {
        let result = BBox::from_hocr_string("invalid");
        assert!(result.is_err());
    }

    #[test]
    fn test_extract_bbox() {
        let line = r#"<span class="ocr_line" title="bbox 100 200 300 400; baseline 0 -5">"#;
        let bbox = extract_bbox(line).unwrap();
        assert_eq!(bbox.x, 100);
        assert_eq!(bbox.y, 200);
        assert_eq!(bbox.width, 200);
        assert_eq!(bbox.height, 200);
    }

    #[test]
    fn test_extract_word_text() {
        let line = r#"<span class="ocrx_word">Hello</span>"#;
        let text = extract_word_text(line).unwrap();
        assert_eq!(text, "Hello");
    }

    #[test]
    fn test_extract_confidence() {
        let line = r#"<span title="bbox 100 200 300 400; x_wconf 95">"#;
        let conf = extract_confidence(line).unwrap();
        assert_eq!(conf, 95);
    }

    #[test]
    fn test_hocr_document_new() {
        let doc = HocrDocument::new();
        assert_eq!(doc.paragraphs.len(), 0);
    }

    #[test]
    fn test_hocr_word_new() {
        let bbox = BBox::new(10, 20, 30, 40);
        let word = HocrWord::new(bbox.clone(), "test".to_string(), Some(95));
        assert_eq!(word.bbox, bbox);
        assert_eq!(word.text, "test");
        assert_eq!(word.confidence, Some(95));
    }

    #[test]
    fn test_hocr_line_add_word() {
        let mut line = HocrLine::new(BBox::new(0, 0, 100, 50));
        let word = HocrWord::new(BBox::new(10, 10, 30, 40), "test".to_string(), None);
        line.add_word(word);
        assert_eq!(line.words.len(), 1);
    }

    #[test]
    fn test_hocr_paragraph_add_line() {
        let mut para = HocrParagraph::new(BBox::new(0, 0, 200, 100));
        let line = HocrLine::new(BBox::new(10, 10, 180, 40));
        para.add_line(line);
        assert_eq!(para.lines.len(), 1);
    }
}
