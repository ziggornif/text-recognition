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
