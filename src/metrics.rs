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

/// Calcule le WER (Word Error Rate) entre le texte OCR et le texte de référence.
///
/// Le WER est le taux d'erreur au niveau des mots, calculé comme le rapport
/// entre la distance de Levenshtein au niveau des mots et le nombre de mots
/// dans le texte de référence.
///
/// **Formule** : WER = distance_levenshtein_mots / nombre_mots_référence
///
/// Les mots sont définis comme des séquences de caractères non-blancs séparées
/// par des espaces blancs.
///
/// # Arguments
///
/// * `ocr_text` - Le texte extrait par OCR
/// * `reference_text` - Le texte de référence attendu
///
/// # Retour
///
/// Un nombre flottant entre 0.0 et potentiellement > 1.0 :
/// - **0.0** : Tous les mots sont identiques
/// - **< 1.0** : Présence d'erreurs, mais moins d'opérations que de mots de référence
/// - **1.0** : Nombre d'erreurs égal au nombre de mots de référence
/// - **> 1.0** : Plus d'erreurs que de mots de référence (cas rare)
///
/// # Cas particuliers
///
/// - Si le texte de référence est vide, retourne 0.0 si l'OCR est aussi vide, sinon 1.0
/// - Si les deux textes sont vides, retourne 0.0
/// - Les espaces multiples sont normalisés (traités comme un seul séparateur)
///
/// # Exemples
///
/// ```
/// use text_recognition::metrics::calculate_wer;
///
/// // Textes identiques
/// let wer = calculate_wer("hello world", "hello world");
/// assert_eq!(wer, 0.0);
///
/// // Un mot différent sur 2
/// let wer = calculate_wer("hello universe", "hello world");
/// assert_eq!(wer, 0.5); // 1 erreur sur 2 mots
///
/// // Un mot manquant
/// let wer = calculate_wer("hello", "hello world");
/// assert_eq!(wer, 0.5); // 1 suppression sur 2 mots
///
/// // Un mot ajouté
/// let wer = calculate_wer("hello big world", "hello world");
/// assert_eq!(wer, 0.5); // 1 insertion sur 2 mots
/// ```
///
/// # Note
///
/// Le WER utilise l'algorithme de Levenshtein au niveau des mots entiers,
/// donc même une petite différence dans un mot (ex: "hello" vs "helo")
/// compte comme une erreur complète.
pub fn calculate_wer(ocr_text: &str, reference_text: &str) -> f64 {
    // Diviser en mots (séquences non-blanches)
    let reference_words: Vec<&str> = reference_text.split_whitespace().collect();
    let ocr_words: Vec<&str> = ocr_text.split_whitespace().collect();

    let reference_word_count = reference_words.len();

    // Cas particulier : texte de référence vide
    if reference_word_count == 0 {
        let ocr_word_count = ocr_words.len();
        return if ocr_word_count == 0 { 0.0 } else { 1.0 };
    }

    // Calculer la distance de Levenshtein au niveau des mots
    let distance = word_levenshtein_distance(&ocr_words, &reference_words);
    distance as f64 / reference_word_count as f64
}

/// Calcule la distance de Levenshtein entre deux séquences de mots.
///
/// Similaire à `levenshtein_distance` mais opère sur des mots entiers
/// plutôt que sur des caractères individuels.
///
/// # Arguments
///
/// * `source` - Séquence de mots source (texte OCR)
/// * `target` - Séquence de mots cible (texte de référence)
///
/// # Retour
///
/// Le nombre minimal d'opérations (insertion, suppression, substitution de mots)
/// nécessaires pour transformer `source` en `target`.
fn word_levenshtein_distance(source: &[&str], target: &[&str]) -> usize {
    let source_len = source.len();
    let target_len = target.len();

    // Cas de base : si une des séquences est vide
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
            // Coût de substitution : 0 si les mots sont identiques, 1 sinon
            let substitution_cost = if source[i - 1] == target[j - 1] { 0 } else { 1 };

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

/// Compare un résultat OCR avec un texte de référence et calcule toutes les métriques.
///
/// Cette fonction effectue une analyse complète de la qualité d'un résultat OCR
/// en calculant le CER, le WER, la distance de Levenshtein, et en comptant les
/// caractères et mots dans les deux textes.
///
/// # Arguments
///
/// * `ocr_text` - Le texte extrait par OCR
/// * `reference_text` - Le texte de référence attendu
///
/// # Retour
///
/// Une structure `OcrMetrics` contenant toutes les métriques calculées :
/// - `cer` : Character Error Rate
/// - `wer` : Word Error Rate
/// - `levenshtein_distance` : Distance de Levenshtein au niveau des caractères
/// - `reference_char_count` : Nombre de caractères dans la référence
/// - `ocr_char_count` : Nombre de caractères dans le texte OCR
/// - `reference_word_count` : Nombre de mots dans la référence
/// - `ocr_word_count` : Nombre de mots dans le texte OCR
/// - `exact_match` : `true` si les textes sont identiques
///
/// # Exemples
///
/// ```
/// use text_recognition::metrics::compare_ocr_result;
///
/// // Textes identiques
/// let metrics = compare_ocr_result("hello world", "hello world");
/// assert_eq!(metrics.cer, 0.0);
/// assert_eq!(metrics.wer, 0.0);
/// assert!(metrics.exact_match);
///
/// // Texte avec une erreur
/// let metrics = compare_ocr_result("helo world", "hello world");
/// assert!(metrics.cer > 0.0);
/// assert!(metrics.wer > 0.0);
/// assert!(!metrics.exact_match);
/// assert_eq!(metrics.levenshtein_distance, 1);
/// ```
///
/// # Utilisation
///
/// Cette fonction est typiquement utilisée après une extraction OCR pour évaluer
/// la qualité du résultat par rapport à un texte de référence connu :
///
/// ```no_run
/// use text_recognition::ocr::OcrEngine;
/// use text_recognition::config::OcrConfig;
/// use text_recognition::metrics::compare_ocr_result;
/// use std::path::Path;
///
/// # fn main() -> anyhow::Result<()> {
/// let mut engine = OcrEngine::new(OcrConfig::default())?;
/// let ocr_text = engine.extract_text_from_file(Path::new("test.png"))?;
/// let reference = "Expected text content";
///
/// let metrics = compare_ocr_result(&ocr_text, reference);
/// println!("CER: {:.2}%", metrics.cer * 100.0);
/// println!("WER: {:.2}%", metrics.wer * 100.0);
/// println!("Accuracy: {:.2}%", metrics.accuracy() * 100.0);
/// # Ok(())
/// # }
/// ```
pub fn compare_ocr_result(ocr_text: &str, reference_text: &str) -> OcrMetrics {
    // Calculer la distance de Levenshtein
    let levenshtein_distance = levenshtein_distance(ocr_text, reference_text);

    // Compter les caractères
    let reference_char_count = reference_text.chars().count();
    let ocr_char_count = ocr_text.chars().count();

    // Compter les mots
    let reference_word_count = reference_text.split_whitespace().count();
    let ocr_word_count = ocr_text.split_whitespace().count();

    // Calculer le CER
    let cer = calculate_cer(ocr_text, reference_text);

    // Calculer le WER
    let wer = calculate_wer(ocr_text, reference_text);

    // Vérifier si c'est un match exact
    let exact_match = ocr_text == reference_text;

    OcrMetrics {
        cer,
        wer,
        levenshtein_distance,
        reference_char_count,
        ocr_char_count,
        reference_word_count,
        ocr_word_count,
        exact_match,
    }
}

/// Génère un rapport détaillé des différences entre le texte OCR et le texte de référence.
///
/// Cette fonction produit un rapport formaté en texte qui présente :
/// - Les métriques globales (CER, WER, distance de Levenshtein)
/// - Les statistiques de caractères et de mots
/// - Une comparaison côte à côte des textes
/// - Un résumé de la qualité
///
/// # Arguments
///
/// * `ocr_text` - Le texte extrait par OCR
/// * `reference_text` - Le texte de référence attendu
///
/// # Retour
///
/// Une chaîne de caractères contenant le rapport formaté, prêt à être affiché
/// ou écrit dans un fichier.
///
/// # Format du rapport
///
/// Le rapport contient les sections suivantes :
/// 1. **En-tête** : Titre du rapport
/// 2. **Métriques** : CER, WER, distance de Levenshtein, précision
/// 3. **Statistiques** : Nombre de caractères et mots dans chaque texte
/// 4. **Comparaison** : Affichage des deux textes pour comparaison visuelle
/// 5. **Résumé** : Évaluation qualitative du résultat (Excellent, Bon, Moyen, Faible)
///
/// # Exemples
///
/// ```
/// use text_recognition::metrics::generate_diff_report;
///
/// let ocr = "hello world";
/// let reference = "hello world";
/// let report = generate_diff_report(ocr, reference);
/// println!("{}", report);
/// ```
///
/// Exemple de sortie pour un texte avec erreurs :
///
/// ```text
/// ═══════════════════════════════════════════════════════════
///                    OCR COMPARISON REPORT
/// ═══════════════════════════════════════════════════════════
///
/// METRICS:
/// --------
/// Character Error Rate (CER): 9.09%
/// Word Error Rate (WER):      50.00%
/// Levenshtein Distance:       1
/// Accuracy:                   90.91%
///
/// STATISTICS:
/// -----------
/// Reference: 11 characters, 2 words
/// OCR:       10 characters, 2 words
///
/// COMPARISON:
/// -----------
/// Reference: "hello world"
/// OCR:       "helo world"
///
/// SUMMARY:
/// --------
/// Quality: Good (minor errors)
/// Match:   Not exact
/// ```
///
/// # Utilisation
///
/// Cette fonction est utile pour :
/// - Déboguer les problèmes d'OCR
/// - Générer des rapports de test
/// - Comparer différentes configurations
/// - Documenter la qualité des résultats
///
/// ```no_run
/// use text_recognition::ocr::OcrEngine;
/// use text_recognition::config::OcrConfig;
/// use text_recognition::metrics::generate_diff_report;
/// use std::path::Path;
/// use std::fs;
///
/// # fn main() -> anyhow::Result<()> {
/// let mut engine = OcrEngine::new(OcrConfig::default())?;
/// let ocr_text = engine.extract_text_from_file(Path::new("test.png"))?;
/// let reference = fs::read_to_string("test_expected.txt")?;
///
/// let report = generate_diff_report(&ocr_text, &reference);
/// fs::write("report.txt", report)?;
/// # Ok(())
/// # }
/// ```
pub fn generate_diff_report(ocr_text: &str, reference_text: &str) -> String {
    // Calculer les métriques
    let metrics = compare_ocr_result(ocr_text, reference_text);

    // Déterminer la qualité du résultat
    let quality = if metrics.exact_match {
        "Perfect (exact match)"
    } else if metrics.cer < 0.05 {
        "Excellent (< 5% error)"
    } else if metrics.cer < 0.15 {
        "Good (< 15% error)"
    } else if metrics.cer < 0.30 {
        "Fair (< 30% error)"
    } else {
        "Poor (≥ 30% error)"
    };

    // Construire le rapport
    let mut report = String::new();

    // En-tête
    report.push_str("═══════════════════════════════════════════════════════════\n");
    report.push_str("                   OCR COMPARISON REPORT\n");
    report.push_str("═══════════════════════════════════════════════════════════\n\n");

    // Métriques
    report.push_str("METRICS:\n");
    report.push_str("--------\n");
    report.push_str(&format!(
        "Character Error Rate (CER): {:.2}%\n",
        metrics.cer * 100.0
    ));
    report.push_str(&format!(
        "Word Error Rate (WER):      {:.2}%\n",
        metrics.wer * 100.0
    ));
    report.push_str(&format!(
        "Levenshtein Distance:       {}\n",
        metrics.levenshtein_distance
    ));
    report.push_str(&format!(
        "Accuracy:                   {:.2}%\n",
        metrics.accuracy() * 100.0
    ));

    // Statistiques
    report.push_str("\nSTATISTICS:\n");
    report.push_str("-----------\n");
    report.push_str(&format!(
        "Reference: {} characters, {} words\n",
        metrics.reference_char_count, metrics.reference_word_count
    ));
    report.push_str(&format!(
        "OCR:       {} characters, {} words\n",
        metrics.ocr_char_count, metrics.ocr_word_count
    ));

    // Comparaison
    report.push_str("\nCOMPARISON:\n");
    report.push_str("-----------\n");

    // Limiter la longueur des textes affichés pour la lisibilité
    let max_display_len = 200;
    let ref_display = if reference_text.len() > max_display_len {
        format!("{}... (truncated)", &reference_text[..max_display_len])
    } else {
        reference_text.to_string()
    };
    let ocr_display = if ocr_text.len() > max_display_len {
        format!("{}... (truncated)", &ocr_text[..max_display_len])
    } else {
        ocr_text.to_string()
    };

    report.push_str(&format!("Reference: \"{}\"\n", ref_display));
    report.push_str(&format!("OCR:       \"{}\"\n", ocr_display));

    // Résumé
    report.push_str("\nSUMMARY:\n");
    report.push_str("--------\n");
    report.push_str(&format!("Quality: {}\n", quality));
    report.push_str(&format!(
        "Match:   {}\n",
        if metrics.exact_match {
            "Exact"
        } else {
            "Not exact"
        }
    ));

    report.push_str("\n═══════════════════════════════════════════════════════════\n");

    report
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

    #[test]
    fn test_calculate_wer_identical_texts() {
        assert_eq!(calculate_wer("hello world", "hello world"), 0.0);
        assert_eq!(calculate_wer("", ""), 0.0);
        assert_eq!(calculate_wer("one two three", "one two three"), 0.0);
    }

    #[test]
    fn test_calculate_wer_empty_reference() {
        // Référence vide, OCR vide : match parfait
        assert_eq!(calculate_wer("", ""), 0.0);

        // Référence vide, OCR non vide : erreur complète
        assert_eq!(calculate_wer("hello world", ""), 1.0);
    }

    #[test]
    fn test_calculate_wer_empty_ocr() {
        // OCR vide, référence non vide : 100% d'erreur
        let wer = calculate_wer("", "hello world");
        assert_eq!(wer, 1.0); // 2 suppressions de mots sur 2 mots
    }

    #[test]
    fn test_calculate_wer_single_word_substitution() {
        // 1 mot différent sur 2
        let wer = calculate_wer("hello universe", "hello world");
        assert_eq!(wer, 0.5); // 1 erreur sur 2 mots
    }

    #[test]
    fn test_calculate_wer_word_deletion() {
        // Un mot manquant
        let wer = calculate_wer("hello", "hello world");
        assert_eq!(wer, 0.5); // 1 suppression sur 2 mots
    }

    #[test]
    fn test_calculate_wer_word_insertion() {
        // Un mot ajouté
        let wer = calculate_wer("hello big world", "hello world");
        assert_eq!(wer, 0.5); // 1 insertion sur 2 mots
    }

    #[test]
    fn test_calculate_wer_multiple_errors() {
        // Plusieurs erreurs
        let wer = calculate_wer("hello big universe", "hello world");
        assert_eq!(wer, 1.0); // 2 erreurs sur 2 mots
    }

    #[test]
    fn test_calculate_wer_completely_wrong() {
        // Tous les mots sont différents
        let wer = calculate_wer("one two three", "four five six");
        assert_eq!(wer, 1.0); // 3 erreurs sur 3 mots
    }

    #[test]
    fn test_calculate_wer_character_difference_in_word() {
        // Une petite différence dans un mot compte comme erreur complète au niveau WER
        let wer = calculate_wer("helo world", "hello world");
        assert_eq!(wer, 0.5); // 1 mot différent sur 2
    }

    #[test]
    fn test_calculate_wer_extra_whitespace() {
        // Les espaces multiples sont normalisés
        let wer = calculate_wer("hello    world", "hello world");
        assert_eq!(wer, 0.0); // Même mots après normalisation
    }

    #[test]
    fn test_calculate_wer_case_sensitive() {
        // La casse compte au niveau des mots
        let wer = calculate_wer("Hello world", "hello world");
        assert_eq!(wer, 0.5); // 1 mot différent sur 2
    }

    #[test]
    fn test_calculate_wer_more_than_100_percent() {
        // OCR beaucoup plus long que la référence : WER > 1.0
        let wer = calculate_wer("one two three four five", "one");
        assert_eq!(wer, 4.0); // 4 insertions sur 1 mot de référence
    }

    #[test]
    fn test_word_levenshtein_distance() {
        let source = vec!["hello", "world"];
        let target = vec!["hello", "world"];
        assert_eq!(word_levenshtein_distance(&source, &target), 0);

        let source = vec!["hello", "big", "world"];
        let target = vec!["hello", "world"];
        assert_eq!(word_levenshtein_distance(&source, &target), 1);

        let source = vec!["hello"];
        let target = vec!["hello", "world"];
        assert_eq!(word_levenshtein_distance(&source, &target), 1);
    }

    #[test]
    fn test_compare_ocr_result_identical_texts() {
        let metrics = compare_ocr_result("hello world", "hello world");
        assert_eq!(metrics.cer, 0.0);
        assert_eq!(metrics.wer, 0.0);
        assert_eq!(metrics.levenshtein_distance, 0);
        assert_eq!(metrics.reference_char_count, 11);
        assert_eq!(metrics.ocr_char_count, 11);
        assert_eq!(metrics.reference_word_count, 2);
        assert_eq!(metrics.ocr_word_count, 2);
        assert!(metrics.exact_match);
        assert_eq!(metrics.accuracy(), 1.0);
    }

    #[test]
    fn test_compare_ocr_result_empty_texts() {
        let metrics = compare_ocr_result("", "");
        assert_eq!(metrics.cer, 0.0);
        assert_eq!(metrics.wer, 0.0);
        assert_eq!(metrics.levenshtein_distance, 0);
        assert_eq!(metrics.reference_char_count, 0);
        assert_eq!(metrics.ocr_char_count, 0);
        assert_eq!(metrics.reference_word_count, 0);
        assert_eq!(metrics.ocr_word_count, 0);
        assert!(metrics.exact_match);
    }

    #[test]
    fn test_compare_ocr_result_single_character_error() {
        let metrics = compare_ocr_result("helo world", "hello world");
        assert!((metrics.cer - 1.0 / 11.0).abs() < 0.001); // 1 erreur sur 11 caractères
        assert_eq!(metrics.wer, 0.5); // 1 mot différent sur 2
        assert_eq!(metrics.levenshtein_distance, 1);
        assert_eq!(metrics.reference_char_count, 11);
        assert_eq!(metrics.ocr_char_count, 10);
        assert_eq!(metrics.reference_word_count, 2);
        assert_eq!(metrics.ocr_word_count, 2);
        assert!(!metrics.exact_match);
    }

    #[test]
    fn test_compare_ocr_result_multiple_word_errors() {
        let metrics = compare_ocr_result("helo wrld", "hello world");
        assert!((metrics.cer - 2.0 / 11.0).abs() < 0.001); // 2 erreurs sur 11 caractères
        assert_eq!(metrics.wer, 1.0); // 2 mots différents sur 2
        assert_eq!(metrics.levenshtein_distance, 2);
        assert_eq!(metrics.reference_char_count, 11);
        assert_eq!(metrics.ocr_char_count, 9);
        assert_eq!(metrics.reference_word_count, 2);
        assert_eq!(metrics.ocr_word_count, 2);
        assert!(!metrics.exact_match);
    }

    #[test]
    fn test_compare_ocr_result_missing_word() {
        let metrics = compare_ocr_result("hello", "hello world");
        assert!((metrics.cer - 6.0 / 11.0).abs() < 0.001); // 6 caractères manquants
        assert_eq!(metrics.wer, 0.5); // 1 mot manquant sur 2
        assert_eq!(metrics.levenshtein_distance, 6); // " world" = 6 caractères
        assert_eq!(metrics.reference_char_count, 11);
        assert_eq!(metrics.ocr_char_count, 5);
        assert_eq!(metrics.reference_word_count, 2);
        assert_eq!(metrics.ocr_word_count, 1);
        assert!(!metrics.exact_match);
    }

    #[test]
    fn test_compare_ocr_result_extra_word() {
        let metrics = compare_ocr_result("hello big world", "hello world");
        assert!((metrics.cer - 4.0 / 11.0).abs() < 0.001); // 4 caractères en trop
        assert_eq!(metrics.wer, 0.5); // 1 mot en trop sur 2
        assert_eq!(metrics.levenshtein_distance, 4); // "big " = 4 caractères
        assert_eq!(metrics.reference_char_count, 11);
        assert_eq!(metrics.ocr_char_count, 15);
        assert_eq!(metrics.reference_word_count, 2);
        assert_eq!(metrics.ocr_word_count, 3);
        assert!(!metrics.exact_match);
    }

    #[test]
    fn test_compare_ocr_result_completely_different() {
        let metrics = compare_ocr_result("abc def", "xyz uvw");
        assert!((metrics.cer - 6.0 / 7.0).abs() < 0.001); // 6 erreurs sur 7 caractères
        assert_eq!(metrics.wer, 1.0); // 2 mots différents sur 2
        assert_eq!(metrics.levenshtein_distance, 6);
        assert_eq!(metrics.reference_char_count, 7);
        assert_eq!(metrics.ocr_char_count, 7);
        assert_eq!(metrics.reference_word_count, 2);
        assert_eq!(metrics.ocr_word_count, 2);
        assert!(!metrics.exact_match);
    }

    #[test]
    fn test_compare_ocr_result_empty_ocr() {
        let metrics = compare_ocr_result("", "hello world");
        assert_eq!(metrics.cer, 1.0); // 100% d'erreur
        assert_eq!(metrics.wer, 1.0); // 100% d'erreur
        assert_eq!(metrics.levenshtein_distance, 11);
        assert_eq!(metrics.reference_char_count, 11);
        assert_eq!(metrics.ocr_char_count, 0);
        assert_eq!(metrics.reference_word_count, 2);
        assert_eq!(metrics.ocr_word_count, 0);
        assert!(!metrics.exact_match);
    }

    #[test]
    fn test_compare_ocr_result_empty_reference() {
        let metrics = compare_ocr_result("hello world", "");
        assert_eq!(metrics.cer, 1.0); // 100% d'erreur (par convention)
        assert_eq!(metrics.wer, 1.0); // 100% d'erreur
        assert_eq!(metrics.levenshtein_distance, 11);
        assert_eq!(metrics.reference_char_count, 0);
        assert_eq!(metrics.ocr_char_count, 11);
        assert_eq!(metrics.reference_word_count, 0);
        assert_eq!(metrics.ocr_word_count, 2);
        assert!(!metrics.exact_match);
    }

    #[test]
    fn test_compare_ocr_result_unicode() {
        let metrics = compare_ocr_result("café", "café");
        assert_eq!(metrics.cer, 0.0);
        assert_eq!(metrics.wer, 0.0);
        assert_eq!(metrics.levenshtein_distance, 0);
        assert_eq!(metrics.reference_char_count, 4);
        assert_eq!(metrics.ocr_char_count, 4);
        assert!(metrics.exact_match);

        let metrics = compare_ocr_result("cafe", "café");
        assert_eq!(metrics.cer, 0.25); // 1 erreur sur 4 caractères
        assert_eq!(metrics.wer, 1.0); // 1 mot différent sur 1
        assert_eq!(metrics.levenshtein_distance, 1);
        assert!(!metrics.exact_match);
    }

    #[test]
    fn test_compare_ocr_result_multiline_text() {
        let reference = "First line\nSecond line\nThird line";
        let ocr = "First line\nSecond line\nThird line";
        let metrics = compare_ocr_result(ocr, reference);
        assert_eq!(metrics.cer, 0.0);
        assert_eq!(metrics.wer, 0.0);
        assert!(metrics.exact_match);
        assert_eq!(metrics.reference_word_count, 6);
    }

    #[test]
    fn test_compare_ocr_result_accuracy() {
        let metrics = compare_ocr_result("hello world", "hello world");
        assert_eq!(metrics.accuracy(), 1.0); // 100% précis

        let metrics = compare_ocr_result("helo world", "hello world");
        assert!((metrics.accuracy() - 10.0 / 11.0).abs() < 0.001); // ~90.9% précis
    }

    #[test]
    fn test_generate_diff_report_perfect_match() {
        let report = generate_diff_report("hello world", "hello world");

        // Vérifier que le rapport contient les sections clés
        assert!(report.contains("OCR COMPARISON REPORT"));
        assert!(report.contains("METRICS:"));
        assert!(report.contains("STATISTICS:"));
        assert!(report.contains("COMPARISON:"));
        assert!(report.contains("SUMMARY:"));

        // Vérifier les métriques
        assert!(report.contains("Character Error Rate (CER): 0.00%"));
        assert!(report.contains("Word Error Rate (WER):      0.00%"));
        assert!(report.contains("Levenshtein Distance:       0"));
        assert!(report.contains("Accuracy:                   100.00%"));

        // Vérifier la qualité
        assert!(report.contains("Quality: Perfect (exact match)"));
        assert!(report.contains("Match:   Exact"));
    }

    #[test]
    fn test_generate_diff_report_excellent_quality() {
        // Texte long pour avoir < 5% d'erreur : 1 erreur sur 25 caractères = 4%
        let reference = "This is a test sentence."; // 24 caractères
        let ocr = "This is a tast sentence."; // 1 erreur : e -> a (4.16%)
        let report = generate_diff_report(ocr, reference);

        // Vérifier la classification de qualité (< 5% erreur = Excellent)
        assert!(report.contains("Quality: Excellent (< 5% error)"));
        assert!(report.contains("Match:   Not exact"));

        // Vérifier que les métriques sont présentes
        assert!(report.contains("Character Error Rate (CER):"));
        assert!(report.contains("Word Error Rate (WER):"));
        assert!(report.contains("Levenshtein Distance:       1"));
    }

    #[test]
    fn test_generate_diff_report_good_quality() {
        // 1 erreur sur 11 caractères = ~9% (< 15% = Good)
        let report = generate_diff_report("helo world", "hello world");

        // ~9% d'erreur devrait être "Good"
        assert!(report.contains("Quality: Good (< 15% error)"));
        assert!(report.contains("Match:   Not exact"));
    }

    #[test]
    fn test_generate_diff_report_fair_quality() {
        // 2 erreurs sur 11 caractères = ~18% (< 30% = Fair)
        let report = generate_diff_report("helo wrld", "hello world");

        // ~18% d'erreur devrait être "Fair"
        assert!(report.contains("Quality: Fair (< 30% error)"));
    }

    #[test]
    fn test_generate_diff_report_poor_quality() {
        // Texte très différent (≥ 30% erreur)
        let report = generate_diff_report("abc def", "hello world");

        assert!(report.contains("Quality: Poor (≥ 30% error)"));
        assert!(report.contains("Match:   Not exact"));
    }

    #[test]
    fn test_generate_diff_report_statistics() {
        let report = generate_diff_report("hello world", "hello world");

        // Vérifier les statistiques
        assert!(report.contains("Reference: 11 characters, 2 words"));
        assert!(report.contains("OCR:       11 characters, 2 words"));
    }

    #[test]
    fn test_generate_diff_report_comparison_section() {
        let report = generate_diff_report("hello world", "goodbye world");

        // Vérifier que les deux textes sont affichés
        assert!(report.contains("Reference: \"goodbye world\""));
        assert!(report.contains("OCR:       \"hello world\""));
    }

    #[test]
    fn test_generate_diff_report_truncation() {
        // Créer un texte très long pour tester la troncature
        let long_text = "a".repeat(250);
        let report = generate_diff_report(&long_text, &long_text);

        // Vérifier que le texte est tronqué
        assert!(report.contains("... (truncated)"));
        assert!(report.contains("250 characters"));
    }

    #[test]
    fn test_generate_diff_report_empty_texts() {
        let report = generate_diff_report("", "");

        // Devrait être un match parfait
        assert!(report.contains("Quality: Perfect (exact match)"));
        assert!(report.contains("Match:   Exact"));
        assert!(report.contains("Reference: 0 characters, 0 words"));
        assert!(report.contains("OCR:       0 characters, 0 words"));
    }

    #[test]
    fn test_generate_diff_report_format() {
        let report = generate_diff_report("test", "test");

        // Vérifier le format avec les bordures
        assert!(report.starts_with("═══════════════════════════════════════════════════════════"));
        assert!(report.ends_with("═══════════════════════════════════════════════════════════\n"));

        // Vérifier les sections avec tirets
        assert!(report.contains("--------"));
        assert!(report.contains("-----------"));
    }

    // ============================================================
    // Tests avec du texte français
    // ============================================================

    #[test]
    fn test_french_text_perfect_match() {
        let reference = "Bonjour, comment allez-vous aujourd'hui ?";
        let ocr = "Bonjour, comment allez-vous aujourd'hui ?";

        let metrics = compare_ocr_result(ocr, reference);
        assert_eq!(metrics.cer, 0.0);
        assert_eq!(metrics.wer, 0.0);
        assert!(metrics.exact_match);
        assert_eq!(metrics.reference_char_count, 41);
        assert_eq!(metrics.reference_word_count, 5);
    }

    #[test]
    fn test_french_text_with_accents() {
        let reference = "Le café est très délicieux et coûte cher.";
        let ocr = "Le cafe est tres delicieux et coute cher.";

        let metrics = compare_ocr_result(ocr, reference);
        // 4 accents : café→cafe (é→e), très→tres (è→e), délicieux→delicieux (é→e), coûte→coute (û→u)
        assert_eq!(metrics.levenshtein_distance, 4);
        assert!((metrics.cer - 4.0 / 41.0).abs() < 0.001); // ~9.75%
        assert_eq!(metrics.reference_char_count, 41);
        assert_eq!(metrics.ocr_char_count, 41);
        assert!(!metrics.exact_match);
    }

    #[test]
    fn test_french_text_accent_errors_cer() {
        let reference = "école";
        let ocr = "ecole";

        let cer = calculate_cer(ocr, reference);
        assert_eq!(cer, 0.2); // 1 erreur sur 5 caractères
    }

    #[test]
    fn test_french_text_cedilla() {
        let reference = "Le garçon reçoit un reçu.";
        let ocr = "Le garcon recoit un recu.";

        let metrics = compare_ocr_result(ocr, reference);
        // 3 cédilles : garçon→garcon, reçoit→recoit, reçu→recu
        assert_eq!(metrics.levenshtein_distance, 3);
        assert_eq!(metrics.reference_char_count, 25);
        assert_eq!(metrics.ocr_char_count, 25);
    }

    #[test]
    fn test_french_text_ligature_oe() {
        let reference = "Un bœuf et un œuf dans le cœur.";
        let ocr = "Un boeuf et un oeuf dans le coeur.";

        let metrics = compare_ocr_result(ocr, reference);
        // 3 ligatures œ→oe (chacune compte comme 1 suppression + 2 insertions = 2 opérations)
        // En réalité: bœuf→boeuf (2), œuf→oeuf (2), cœur→coeur (2) = 6 opérations
        assert_eq!(metrics.levenshtein_distance, 6);
        assert_eq!(metrics.reference_char_count, 31);
        assert_eq!(metrics.ocr_char_count, 34);
    }

    #[test]
    fn test_french_text_apostrophe() {
        let reference = "L'école d'été qu'il a visitée.";
        let ocr = "L'ecole d'ete qu'il a visitee.";

        let metrics = compare_ocr_result(ocr, reference);
        // 4 accents : école→ecole, été→ete, visitée→visitee (2 accents)
        assert_eq!(metrics.levenshtein_distance, 4);
        assert_eq!(metrics.reference_word_count, 5);
        assert_eq!(metrics.reference_char_count, 30);
    }

    #[test]
    fn test_french_text_complex_sentence() {
        let reference = "L'été dernier, j'ai visité la côte méditerranéenne.";
        let ocr = "L'ete dernier, j'ai visite la cote mediterraneenne.";

        let metrics = compare_ocr_result(ocr, reference);
        // Accents manquants : été→ete, visité→visite, côte→cote, méditerranéenne→mediterraneenne (2 accents)
        // Total: 6 erreurs
        assert_eq!(metrics.levenshtein_distance, 6);
        assert!((metrics.cer - 6.0 / 51.0).abs() < 0.001); // ~11.76%
        assert_eq!(metrics.reference_char_count, 51);
    }

    #[test]
    fn test_french_generate_report() {
        let reference = "Le développement logiciel nécessite de la rigueur.";
        let ocr = "Le developpement logiciel necessite de la rigueur.";

        let report = generate_diff_report(ocr, reference);

        // Vérifier que le rapport est bien généré
        assert!(report.contains("OCR COMPARISON REPORT"));
        assert!(report.contains("Character Error Rate (CER):"));
        assert!(report.contains("COMPARISON:"));

        // 2 accents : développement→developpement, nécessite→necessite
        // 2 erreurs sur 50 caractères = 4% → Excellent
        assert!(report.contains("Quality: Excellent (< 5% error)"));
    }

    #[test]
    fn test_french_multiline() {
        let reference =
            "Première ligne avec des accents.\nDeuxième ligne très longue.\nTroisième ligne.";
        let ocr = "Premiere ligne avec des accents.\nDeuxieme ligne tres longue.\nTroisieme ligne.";

        let metrics = compare_ocr_result(ocr, reference);
        // 4 accents : Première→Premiere, Deuxième→Deuxieme, très→tres, Troisième→Troisieme
        assert_eq!(metrics.levenshtein_distance, 4);
        assert_eq!(metrics.reference_word_count, 11);
        assert_eq!(metrics.reference_char_count, 77);
    }

    #[test]
    fn test_french_proper_nouns() {
        let reference = "François habite à Paris près de l'Élysée.";
        let ocr = "Francois habite a Paris pres de l'Elysee.";

        let metrics = compare_ocr_result(ocr, reference);
        // 5 accents : François→Francois, à→a, près→pres, Élysée→Elysee (2 accents)
        assert_eq!(metrics.levenshtein_distance, 5);
        assert!((metrics.cer - 5.0 / 41.0).abs() < 0.001); // ~12.2%
        assert_eq!(metrics.reference_char_count, 41);
    }
}
