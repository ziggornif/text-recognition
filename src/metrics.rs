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

/// Type d'erreur identifié lors de la comparaison de textes.
///
/// Cette enum catégorise les différentes erreurs qui peuvent survenir
/// lors de l'analyse de la différence entre le texte OCR et le texte de référence.
///
/// # Exemples
///
/// ```
/// use text_recognition::metrics::TextError;
///
/// let error = TextError::Substitution {
///     position: 5,
///     expected: 'a',
///     found: 'o',
/// };
///
/// match error {
///     TextError::Substitution { position, expected, found } => {
///         println!("Caractère '{}' remplacé par '{}' à la position {}", expected, found, position);
///     }
///     _ => {}
/// }
/// ```
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TextError {
    /// Un caractère a été substitué par un autre.
    ///
    /// Par exemple : "chat" → "chot" (a → o)
    Substitution {
        /// Position du caractère dans le texte de référence.
        position: usize,
        /// Caractère attendu.
        expected: char,
        /// Caractère trouvé dans le texte OCR.
        found: char,
    },

    /// Un caractère manque dans le texte OCR.
    ///
    /// Par exemple : "chat" → "cht" (manque 'a')
    Deletion {
        /// Position du caractère manquant dans le texte de référence.
        position: usize,
        /// Caractère qui manque.
        expected: char,
    },

    /// Un caractère supplémentaire a été ajouté dans le texte OCR.
    ///
    /// Par exemple : "chat" → "chaat" (ajout d'un 'a')
    Insertion {
        /// Position de l'insertion dans le texte OCR.
        position: usize,
        /// Caractère inséré à tort.
        found: char,
    },

    /// Un mot entier est incorrect.
    ///
    /// Cette variante est utilisée pour les erreurs au niveau des mots
    /// lors du calcul du WER.
    WordError {
        /// Position du mot dans le texte de référence.
        word_position: usize,
        /// Mot attendu.
        expected: String,
        /// Mot trouvé dans le texte OCR.
        found: String,
    },
}

impl TextError {
    /// Retourne la position de l'erreur.
    ///
    /// Pour les erreurs de caractères (Substitution, Deletion, Insertion),
    /// retourne la position du caractère. Pour WordError, retourne la position du mot.
    ///
    /// # Exemples
    ///
    /// ```
    /// use text_recognition::metrics::TextError;
    ///
    /// let error = TextError::Substitution {
    ///     position: 5,
    ///     expected: 'a',
    ///     found: 'o',
    /// };
    ///
    /// assert_eq!(error.position(), 5);
    /// ```
    pub fn position(&self) -> usize {
        match self {
            TextError::Substitution { position, .. } => *position,
            TextError::Deletion { position, .. } => *position,
            TextError::Insertion { position, .. } => *position,
            TextError::WordError { word_position, .. } => *word_position,
        }
    }

    /// Retourne une description textuelle de l'erreur.
    ///
    /// # Exemples
    ///
    /// ```
    /// use text_recognition::metrics::TextError;
    ///
    /// let error = TextError::Substitution {
    ///     position: 5,
    ///     expected: 'a',
    ///     found: 'o',
    /// };
    ///
    /// assert_eq!(error.description(), "Substitution: 'a' → 'o' at position 5");
    /// ```
    pub fn description(&self) -> String {
        match self {
            TextError::Substitution {
                position,
                expected,
                found,
            } => format!(
                "Substitution: '{}' → '{}' at position {}",
                expected, found, position
            ),
            TextError::Deletion { position, expected } => {
                format!("Deletion: '{}' missing at position {}", expected, position)
            }
            TextError::Insertion { position, found } => {
                format!("Insertion: '{}' added at position {}", found, position)
            }
            TextError::WordError {
                word_position,
                expected,
                found,
            } => format!(
                "Word error: '{}' → '{}' at word position {}",
                expected, found, word_position
            ),
        }
    }
}

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

/// Calcule la distance de Levenshtein entre deux chaînes de caractères.
///
/// La distance de Levenshtein est le nombre minimal d'opérations nécessaires
/// pour transformer une chaîne en une autre. Les opérations autorisées sont :
/// - **Insertion** d'un caractère
/// - **Suppression** d'un caractère
/// - **Substitution** d'un caractère par un autre
///
/// # Arguments
///
/// * `source` - La chaîne source (texte OCR)
/// * `target` - La chaîne cible (texte de référence)
///
/// # Retour
///
/// Le nombre minimal d'opérations nécessaires pour transformer `source` en `target`.
///
/// # Algorithme
///
/// Utilise la programmation dynamique avec une matrice de taille (n+1) × (m+1)
/// où n et m sont les longueurs des deux chaînes.
///
/// # Exemples
///
/// ```
/// use text_recognition::metrics::levenshtein_distance;
///
/// // Chaînes identiques
/// assert_eq!(levenshtein_distance("chat", "chat"), 0);
///
/// // Une substitution
/// assert_eq!(levenshtein_distance("chat", "chot"), 1);
///
/// // Une insertion
/// assert_eq!(levenshtein_distance("chat", "chaat"), 1);
///
/// // Une suppression
/// assert_eq!(levenshtein_distance("chat", "cht"), 1);
///
/// // Opérations multiples
/// assert_eq!(levenshtein_distance("kitten", "sitting"), 3);
/// ```
///
/// # Complexité
///
/// - **Temps** : O(n × m) où n et m sont les longueurs des chaînes
/// - **Espace** : O(n × m)
pub fn levenshtein_distance(source: &str, target: &str) -> usize {
    let source_chars: Vec<char> = source.chars().collect();
    let target_chars: Vec<char> = target.chars().collect();

    let source_len = source_chars.len();
    let target_len = target_chars.len();

    // Cas de base : si une des chaînes est vide
    if source_len == 0 {
        return target_len;
    }
    if target_len == 0 {
        return source_len;
    }

    // Créer une matrice (source_len + 1) × (target_len + 1)
    let mut matrix = vec![vec![0usize; target_len + 1]; source_len + 1];

    // Initialiser la première colonne (suppressions depuis source)
    #[allow(clippy::needless_range_loop)]
    for i in 0..=source_len {
        matrix[i][0] = i;
    }

    // Initialiser la première ligne (insertions pour atteindre target)
    #[allow(clippy::needless_range_loop)]
    for j in 0..=target_len {
        matrix[0][j] = j;
    }

    // Remplir la matrice
    for i in 1..=source_len {
        for j in 1..=target_len {
            // Coût de substitution : 0 si les caractères sont identiques, 1 sinon
            let substitution_cost = if source_chars[i - 1] == target_chars[j - 1] {
                0
            } else {
                1
            };

            matrix[i][j] = std::cmp::min(
                std::cmp::min(
                    matrix[i - 1][j] + 1, // Suppression
                    matrix[i][j - 1] + 1, // Insertion
                ),
                matrix[i - 1][j - 1] + substitution_cost, // Substitution
            );
        }
    }

    // La distance est dans la dernière cellule
    matrix[source_len][target_len]
}

/// Calcule le CER (Character Error Rate) entre le texte OCR et le texte de référence.
///
/// Le CER est le taux d'erreur au niveau des caractères, calculé comme le rapport
/// entre la distance de Levenshtein et le nombre de caractères dans le texte de référence.
///
/// **Formule** : CER = distance_levenshtein / nombre_caractères_référence
///
/// # Arguments
///
/// * `ocr_text` - Le texte extrait par OCR
/// * `reference_text` - Le texte de référence attendu
///
/// # Retour
///
/// Un nombre flottant entre 0.0 et potentiellement > 1.0 :
/// - **0.0** : Textes identiques (aucune erreur)
/// - **< 1.0** : Présence d'erreurs, mais moins d'opérations que de caractères de référence
/// - **1.0** : Nombre d'erreurs égal au nombre de caractères de référence
/// - **> 1.0** : Plus d'erreurs que de caractères de référence (cas rare, nombreuses insertions)
///
/// # Cas particuliers
///
/// - Si le texte de référence est vide, retourne 0.0 si l'OCR est aussi vide, sinon 1.0
/// - Si les deux textes sont vides, retourne 0.0 (considéré comme une correspondance parfaite)
///
/// # Exemples
///
/// ```
/// use text_recognition::metrics::calculate_cer;
///
/// // Textes identiques
/// let cer = calculate_cer("hello world", "hello world");
/// assert_eq!(cer, 0.0);
///
/// // Une erreur sur 11 caractères
/// let cer = calculate_cer("hallo world", "hello world");
/// assert!((cer - 0.0909).abs() < 0.001); // ≈ 1/11 = 0.0909
///
/// // Texte complètement différent
/// let cer = calculate_cer("abc", "xyz");
/// assert_eq!(cer, 1.0); // 3 erreurs sur 3 caractères
/// ```
pub fn calculate_cer(ocr_text: &str, reference_text: &str) -> f64 {
    let reference_len = reference_text.chars().count();

    // Cas particulier : texte de référence vide
    if reference_len == 0 {
        let ocr_len = ocr_text.chars().count();
        return if ocr_len == 0 { 0.0 } else { 1.0 };
    }

    let distance = levenshtein_distance(ocr_text, reference_text);
    distance as f64 / reference_len as f64
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_levenshtein_identical_strings() {
        assert_eq!(levenshtein_distance("hello", "hello"), 0);
        assert_eq!(levenshtein_distance("", ""), 0);
        assert_eq!(levenshtein_distance("a", "a"), 0);
    }

    #[test]
    fn test_levenshtein_empty_strings() {
        assert_eq!(levenshtein_distance("", "hello"), 5);
        assert_eq!(levenshtein_distance("hello", ""), 5);
        assert_eq!(levenshtein_distance("", ""), 0);
    }

    #[test]
    fn test_levenshtein_single_substitution() {
        assert_eq!(levenshtein_distance("chat", "chot"), 1);
        assert_eq!(levenshtein_distance("hello", "hallo"), 1);
    }

    #[test]
    fn test_levenshtein_single_insertion() {
        assert_eq!(levenshtein_distance("chat", "chaat"), 1);
        assert_eq!(levenshtein_distance("helo", "hello"), 1);
    }

    #[test]
    fn test_levenshtein_single_deletion() {
        assert_eq!(levenshtein_distance("chat", "cht"), 1);
        assert_eq!(levenshtein_distance("hello", "hllo"), 1);
    }

    #[test]
    fn test_levenshtein_multiple_operations() {
        // kitten → sitting : 3 opérations
        // k → s (substitution)
        // e → i (substitution)
        // + t + g (2 insertions)
        assert_eq!(levenshtein_distance("kitten", "sitting"), 3);

        assert_eq!(levenshtein_distance("saturday", "sunday"), 3);
    }

    #[test]
    fn test_levenshtein_completely_different() {
        assert_eq!(levenshtein_distance("abc", "xyz"), 3);
    }

    #[test]
    fn test_levenshtein_unicode() {
        assert_eq!(levenshtein_distance("café", "café"), 0);
        assert_eq!(levenshtein_distance("café", "cafe"), 1);
        assert_eq!(levenshtein_distance("🐱", "🐶"), 1);
    }

    #[test]
    fn test_levenshtein_case_sensitive() {
        assert_eq!(levenshtein_distance("Hello", "hello"), 1);
        assert_eq!(levenshtein_distance("HELLO", "hello"), 5);
    }

    #[test]
    fn test_calculate_cer_identical_texts() {
        assert_eq!(calculate_cer("hello world", "hello world"), 0.0);
        assert_eq!(calculate_cer("", ""), 0.0);
        assert_eq!(calculate_cer("test", "test"), 0.0);
    }

    #[test]
    fn test_calculate_cer_empty_reference() {
        // Référence vide, OCR vide : match parfait
        assert_eq!(calculate_cer("", ""), 0.0);

        // Référence vide, OCR non vide : erreur complète
        assert_eq!(calculate_cer("something", ""), 1.0);
    }

    #[test]
    fn test_calculate_cer_empty_ocr() {
        // OCR vide, référence non vide : 100% d'erreur
        let cer = calculate_cer("", "hello");
        assert_eq!(cer, 1.0); // 5 suppressions sur 5 caractères
    }

    #[test]
    fn test_calculate_cer_single_error() {
        // 1 erreur sur 11 caractères
        let cer = calculate_cer("hallo world", "hello world");
        assert!((cer - 1.0 / 11.0).abs() < 0.001);
    }

    #[test]
    fn test_calculate_cer_multiple_errors() {
        // "kitten" (OCR) vs "sitting" (référence) : 3 erreurs sur 7 caractères
        let cer = calculate_cer("kitten", "sitting");
        assert!((cer - 3.0 / 7.0).abs() < 0.001); // ≈ 0.4286
    }

    #[test]
    fn test_calculate_cer_completely_wrong() {
        // Texte complètement différent : 100% d'erreur
        let cer = calculate_cer("abc", "xyz");
        assert_eq!(cer, 1.0); // 3 erreurs sur 3 caractères
    }

    #[test]
    fn test_calculate_cer_more_than_100_percent() {
        // OCR beaucoup plus long que la référence : CER > 1.0
        let cer = calculate_cer("aaaaaaaaaa", "a");
        assert_eq!(cer, 9.0); // 9 insertions sur 1 caractère de référence
    }

    #[test]
    fn test_calculate_cer_unicode() {
        // Test avec caractères Unicode
        let cer = calculate_cer("café", "café");
        assert_eq!(cer, 0.0);

        // 1 erreur (é → e) sur 4 caractères
        let cer = calculate_cer("cafe", "café");
        assert_eq!(cer, 0.25); // 1/4
    }

    #[test]
    fn test_calculate_cer_case_sensitive() {
        // La casse compte : "Hello" vs "hello" = 1 erreur sur 5 caractères
        let cer = calculate_cer("Hello", "hello");
        assert_eq!(cer, 0.2); // 1/5
    }

    #[test]
    fn test_calculate_cer_whitespace() {
        // Les espaces comptent
        let cer = calculate_cer("helloworld", "hello world");
        assert!((cer - 1.0 / 11.0).abs() < 0.001); // 1 suppression d'espace
    }
}
