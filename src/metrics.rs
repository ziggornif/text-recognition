//! Module de calcul de métriques de qualité OCR.
//!
//! Ce module fournit des outils pour évaluer la qualité des résultats OCR
//! en comparant le texte extrait avec un texte de référence attendu.
//!
//! Les métriques principales incluent :
//! - **CER** (Character Error Rate) : Taux d'erreur au niveau des caractères
//! - **WER** (Word Error Rate) : Taux d'erreur au niveau des mots
//! - **Distance de Levenshtein** : Nombre minimal d'opérations pour transformer un texte en un autre
//!
//! Ces métriques permettent de :
//! - Mesurer l'efficacité de différentes configurations OCR
//! - Comparer l'impact des prétraitements
//! - Identifier les configurations optimales pour différents types d'images

/// Résultats de la comparaison entre le texte OCR et le texte de référence.
///
/// Cette structure contient toutes les métriques calculées lors de la comparaison
/// d'un résultat OCR avec un texte attendu.
///
/// # Exemples
///
/// ```
/// use text_recognition::metrics::OcrMetrics;
///
/// let metrics = OcrMetrics {
///     cer: 0.05,
///     wer: 0.10,
///     levenshtein_distance: 3,
///     reference_char_count: 60,
///     ocr_char_count: 58,
///     reference_word_count: 12,
///     ocr_word_count: 12,
///     exact_match: false,
/// };
///
/// println!("CER: {:.2}%", metrics.cer * 100.0);
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct OcrMetrics {
    /// Character Error Rate : taux d'erreur au niveau des caractères (0.0 = parfait, 1.0 = 100% d'erreurs).
    pub cer: f64,

    /// Word Error Rate : taux d'erreur au niveau des mots (0.0 = parfait, 1.0 = 100% d'erreurs).
    pub wer: f64,

    /// Distance de Levenshtein : nombre minimal d'opérations (insertion, suppression, substitution)
    /// pour transformer le texte OCR en texte de référence.
    pub levenshtein_distance: usize,

    /// Nombre de caractères dans le texte de référence.
    pub reference_char_count: usize,

    /// Nombre de caractères dans le texte extrait par OCR.
    pub ocr_char_count: usize,

    /// Nombre de mots dans le texte de référence.
    pub reference_word_count: usize,

    /// Nombre de mots dans le texte extrait par OCR.
    pub ocr_word_count: usize,

    /// Indique si le texte OCR correspond exactement au texte de référence.
    pub exact_match: bool,
}

impl OcrMetrics {
    /// Crée une instance de `OcrMetrics` avec toutes les valeurs à zéro.
    ///
    /// Utile comme valeur par défaut ou pour initialiser avant calcul.
    ///
    /// # Exemples
    ///
    /// ```
    /// use text_recognition::metrics::OcrMetrics;
    ///
    /// let metrics = OcrMetrics::zero();
    /// assert_eq!(metrics.cer, 0.0);
    /// assert_eq!(metrics.levenshtein_distance, 0);
    /// ```
    pub fn zero() -> Self {
        Self {
            cer: 0.0,
            wer: 0.0,
            levenshtein_distance: 0,
            reference_char_count: 0,
            ocr_char_count: 0,
            reference_word_count: 0,
            ocr_word_count: 0,
            exact_match: true,
        }
    }

    /// Retourne un pourcentage de précision basé sur le CER (1.0 - CER).
    ///
    /// # Exemples
    ///
    /// ```
    /// use text_recognition::metrics::OcrMetrics;
    ///
    /// let metrics = OcrMetrics {
    ///     cer: 0.05,
    ///     wer: 0.10,
    ///     levenshtein_distance: 3,
    ///     reference_char_count: 60,
    ///     ocr_char_count: 58,
    ///     reference_word_count: 12,
    ///     ocr_word_count: 12,
    ///     exact_match: false,
    /// };
    ///
    /// assert_eq!(metrics.accuracy(), 0.95);
    /// ```
    pub fn accuracy(&self) -> f64 {
        (1.0 - self.cer).max(0.0)
    }
}

impl Default for OcrMetrics {
    fn default() -> Self {
        Self::zero()
    }
}
