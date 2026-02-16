# Guide d'Utilisation pour Claude Code

Ce document contient les instructions spécifiques pour l'agent Claude lors du travail sur ce projet.

## 🎯 Objectif du Projet

Projet éducatif pour apprendre à paramétrer Tesseract OCR avec Rust, en testant différentes configurations, prétraitements et en mesurant la qualité des résultats.

---

## ⚠️ Contraintes Strictes d'Exécution

### Règle #1 : Une Tâche à la Fois

**IMPORTANT** : Ne traiter qu'**UNE SEULE** tâche de la TODO list par session.

- ✅ Prendre la prochaine tâche non complétée dans `TODO.md`
- ✅ L'implémenter complètement
- ✅ Valider avec les commandes de vérification (voir Règle #2)
- ✅ Marquer la tâche comme complétée dans `TODO.md`
- ❌ **NE PAS** enchaîner plusieurs tâches d'affilée
- ❌ **NE PAS** anticiper les tâches suivantes
- ❌ **NE PAS** implémenter des fonctionnalités hors scope de la tâche

**Raison** : Minimiser la consommation de tokens et maintenir un contexte léger.

---

### Règle #2 : Validation Avant Commit

**IMPORTANT** : **Toujours utiliser le Makefile** pour lancer les commandes de validation.

Le Makefile configure automatiquement l'environnement selon l'OS (macOS/Linux) et résout les problèmes de compilation avec bindgen/leptonica.

**Avant CHAQUE commit**, exécuter systématiquement dans cet ordre :

```bash
# 1. Formatage du code
make fmt

# 2. Linting avec Clippy (corriger tous les warnings)
make clippy

# 3. Compilation
make build

# 4. Tests (si des tests existent)
make test
```

**OU utiliser la commande tout-en-un pour la validation complète** :

```bash
make validate
```

Cette commande exécute automatiquement : `fmt`, `clippy`, `build`, et `test` dans l'ordre.

**Tous les checks doivent passer** avant de créer un commit. Si une erreur survient :
- Corriger le problème immédiatement
- Relancer les vérifications
- Ne committer que si tout est vert ✅

**Exception** : Si la tâche consiste à créer une structure vide ou des répertoires, `cargo test` peut échouer temporairement. Dans ce cas, s'assurer au minimum que `cargo build` passe.

**⚠️ NE JAMAIS utiliser directement les commandes `cargo` pour la validation** car elles échoueront sur Linux à cause de problèmes d'environnement bindgen. Le Makefile résout ce problème automatiquement.

---

### Règle #3 : Messages de Commit (Conventional Commits)

**IMPORTANT** : Utiliser le format **Conventional Commits** pour tous les commits.

Format des commits :
```
<type>(<scope>): <description>

[corps optionnel]

[pied de page optionnel]
```

**Types principaux** :
- `feat` : Nouvelle fonctionnalité
- `fix` : Correction de bug
- `docs` : Documentation uniquement
- `style` : Formatage, points-virgules manquants, etc. (pas de changement de code)
- `refactor` : Refactoring (ni nouvelle fonctionnalité ni correction)
- `test` : Ajout ou correction de tests
- `chore` : Tâches de maintenance (dépendances, configuration, etc.)
- `build` : Changements du système de build ou dépendances externes
- `ci` : Changements de configuration CI

**Scope** : Identifiant de phase (phase-1.1, phase-1.2, etc.) ou module (ocr, config, preprocessing, etc.)

**Exemples** :
```
chore(phase-1.1): initialiser projet Cargo

- Création du projet en tant que bibliothèque
- Génération de Cargo.toml avec configuration de base
- Génération de src/lib.rs avec fonction de test
```

```
feat(config): ajouter structure OcrConfig

- Ajout de la structure OcrConfig avec champs de base
- Implémentation du trait Default
- Documentation rustdoc
```

```
feat(ocr): implémenter extraction de texte depuis fichier

- Ajout de OcrEngine::extract_text_from_file()
- Gestion des erreurs avec Result
- Tests unitaires
```

---

## 📋 Workflow Type

Pour chaque tâche :

1. **Lire** `TODO.md` et identifier la prochaine tâche non complétée
2. **Annoncer** la tâche à l'utilisateur
3. **Implémenter** la tâche (code, tests, documentation selon besoin)
4. **Vérifier** avec les commandes de validation via le Makefile :
   - `make fmt`
   - `make clippy`
   - `make build`
   - `make test` (si applicable)
   - OU simplement `make validate` pour tout exécuter
5. **Corriger** les éventuels problèmes jusqu'à ce que tout passe
6. **Committer** avec un message de commit approprié
7. **Marquer** la tâche comme complétée dans `TODO.md` (changer `[ ]` en `[x]`)
8. **S'arrêter** et attendre la prochaine instruction de l'utilisateur

---

## 🛠️ Commandes Utiles

**IMPORTANT** : Toujours utiliser le **Makefile** pour les opérations de build, test et validation.

### Makefile - Commandes Principales

```bash
# Afficher la configuration détectée (macOS/Linux)
make info

# Formatage du code
make fmt

# Lint avec Clippy (échoue sur les warnings)
make clippy

# Compiler en mode debug
make build

# Compiler en mode release
make release

# Lancer les tests
make test

# Validation complète avant commit (fmt + clippy + build + test)
make validate

# Exécuter le binaire
make run ARGS="image.png --lang fra"

# Générer la documentation
make doc

# Nettoyer les artefacts de build
make clean

# Afficher l'aide
make help
```

### Développement

```bash
# Exemples d'utilisation de la CLI

# OCR simple
make run ARGS="resources/simple/img-1.png"

# OCR avec langue spécifique
make run ARGS="resources/simple/img-1.png --lang eng"

# OCR avec mode PSM spécifique
make run ARGS="resources/simple/img-1.png --psm 6"

# OCR avec prétraitement
make run ARGS="resources/simple/img-1.png --preprocess --grayscale --binarize"

# OCR avec comparaison de référence
make run ARGS="resources/simple/img-1.png --expected resources/expected/img-1.txt"

# OCR avec export CSV
make run ARGS="resources/simple/img-1.png --expected resources/expected/img-1.txt --csv-export metrics.csv"

# Tester tous les modes PSM
make run ARGS="resources/simple/img-1.png --test-all-psm"

# Tester tous les modes PSM avec export CSV
make run ARGS="resources/simple/img-1.png --expected resources/expected/img-1.txt --test-all-psm --csv-export all_psm.csv"

# Mode batch
make run ARGS="--batch resources/simple/"

# Mode batch avec sortie dans un répertoire
make run ARGS="--batch resources/simple/ --output results/"
```

### Pourquoi utiliser le Makefile ?

Le Makefile détecte automatiquement l'OS et configure les variables d'environnement nécessaires :

- **macOS** : Configure `SDKROOT` pour Xcode
- **Linux** : Configure `BINDGEN_EXTRA_CLANG_ARGS=--target=x86_64-unknown-linux-gnu`

Sans le Makefile, la compilation échouera sur Linux avec une erreur bindgen/leptonica.

### Commandes Cargo Directes (À ÉVITER pour la validation)

⚠️ **Ces commandes peuvent échouer sur Linux**. Utilisez le Makefile à la place.

```bash
# Vérifier compilation rapide (peut échouer sur Linux)
cargo check

# Lancer un test spécifique
cargo test test_name

# Lancer tests avec sortie détaillée
cargo test -- --nocapture

# Mettre à jour les dépendances
cargo update
```

---

## 📁 Organisation du Code

- `src/lib.rs` : Point d'entrée de la bibliothèque, exports publics
- `src/main.rs` : CLI, point d'entrée du binaire
- `src/ocr.rs` : Logique OCR, wrapper Tesseract
- `src/config.rs` : Structures de configuration
- `src/config_file.rs` : Chargement de configuration depuis fichiers JSON/TOML
- `src/preprocessing.rs` : Prétraitement d'images
- `src/metrics.rs` : Calcul de métriques de qualité (CER, WER, export CSV)
- `tests/` : Tests d'intégration
- `resources/` : Images de test et textes de référence
  - `resources/simple/` : Images simples (texte clair)
  - `resources/medium/` : Images moyennes (qualité variable)
  - `resources/complex/` : Images complexes (difficiles)
  - `resources/expected/` : Textes de référence pour validation
- `docs/` : Documentation approfondie
- `Makefile` : Configuration multi-OS pour build (IMPORTANT !)

---

## 🎨 Style de Code

### Formatage
- **Respecter** `rustfmt` (automatique avec `cargo fmt`)
- Utiliser 4 espaces pour l'indentation (standard Rust)
- Longueur de ligne max : 100 caractères (par défaut rustfmt)

### Nommage
- `snake_case` pour fonctions et variables : `extract_text`, `page_seg_mode`
- `PascalCase` pour types et enums : `OcrEngine`, `PageSegMode`
- `SCREAMING_SNAKE_CASE` pour constantes : `DEFAULT_LANGUAGE`

### Documentation
- Tout élément **public** doit avoir un commentaire `///`
- Modules documentés avec `//!` en début de fichier
- Inclure des exemples d'utilisation quand pertinent

### Tests
- Chaque fonction publique devrait avoir au moins un test
- Nommer les tests de manière descriptive : `test_grayscale_conversion`
- Utiliser des assertions claires avec messages explicites

---

## 🐛 Gestion des Erreurs

- Utiliser `Result<T, E>` pour toutes les opérations faillibles
- Privilégier `anyhow::Result` pour les erreurs applicatives simples
- Créer des erreurs personnalisées avec `thiserror` si nécessaire
- Ne **jamais** utiliser `.unwrap()` dans le code de production
- `.unwrap()` acceptable uniquement dans :
  - Tests unitaires
  - Exemples de documentation
  - Situations où le panic est intentionnel et documenté

---

## 📚 Documentation

### Module (`//!`)
```rust
//! Module de configuration pour Tesseract OCR.
//!
//! Ce module fournit les structures et méthodes pour configurer
//! le moteur OCR avec différents modes de segmentation et paramètres.
```

### Fonction publique (`///`)
```rust
/// Extrait le texte d'une image.
///
/// # Arguments
///
/// * `path` - Chemin vers l'image à analyser
///
/// # Exemple
///
/// ```
/// let mut engine = OcrEngine::new(OcrConfig::default())?;
/// let text = engine.extract_text_from_file("image.png")?;
/// ```
///
/// # Erreurs
///
/// Retourne une erreur si :
/// - Le fichier n'existe pas
/// - L'image est corrompue
/// - Tesseract échoue
pub fn extract_text_from_file(&mut self, path: &Path) -> Result<String>
```

---

## ✅ Checklist de Qualité

Avant de marquer une tâche comme terminée :

- [ ] Le code compile sans warnings
- [ ] `make fmt` n'a rien modifié
- [ ] `make clippy` ne retourne aucun warning
- [ ] Les tests passent (`make test`)
- [ ] Le code public est documenté avec `///`
- [ ] La tâche fait exactement ce qui est demandé, ni plus ni moins
- [ ] Commit créé avec message descriptif au format Conventional Commits
- [ ] Tâche marquée comme complétée dans `TODO.md`

---

## 🔄 Cycle de Travail Idéal

```
┌─────────────────────────────────────────┐
│ Démarrage de session                    │
└──────────────┬──────────────────────────┘
               │
               ▼
┌─────────────────────────────────────────┐
│ Lire TODO.md                            │
└──────────────┬──────────────────────────┘
               │
               ▼
┌─────────────────────────────────────────┐
│ Identifier prochaine tâche non complétée│
└──────────────┬──────────────────────────┘
               │
               ▼
┌─────────────────────────────────────────┐
│ Annoncer la tâche à l'utilisateur       │
└──────────────┬──────────────────────────┘
               │
               ▼
┌─────────────────────────────────────────┐
│ Implémenter la tâche                    │
└──────────────┬──────────────────────────┘
               │
               ▼
┌─────────────────────────────────────────┐
│ Validation :                            │
│ - make fmt                              │
│ - make clippy                           │
│ - make build                            │
│ - make test                             │
│ (OU : make validate)                    │
└──────────────┬──────────────────────────┘
               │
               ▼
         ┌─────┴─────┐
         │ Tout passe?│
         └─────┬─────┘
               │
        ┌──────┴──────┐
        │             │
       Oui           Non
        │             │
        │             ▼
        │    ┌────────────────┐
        │    │ Corriger erreurs│
        │    └────────┬────────┘
        │             │
        │             └──────────┐
        │                        │
        ▼                        ▼
┌─────────────────────────────────────────┐
│ Créer commit avec message approprié     │
└──────────────┬──────────────────────────┘
               │
               ▼
┌─────────────────────────────────────────┐
│ Marquer tâche complétée dans TODO.md    │
└──────────────┬──────────────────────────┘
               │
               ▼
┌─────────────────────────────────────────┐
│ Fin de session - Attendre instructions  │
└─────────────────────────────────────────┘
```

---

## 💡 Philosophie

> **Qualité > Quantité**
> 
> Il vaut mieux une tâche bien faite, testée et validée, qu'un enchaînement rapide de plusieurs tâches avec des bugs ou warnings.

> **Respect strict des contraintes**
>
> Les contraintes (une tâche, validation systématique) sont là pour garantir la qualité et réduire la consommation de tokens. Les respecter est essentiel.

---

## 📞 En Cas de Problème

Si une tâche bloque ou nécessite une clarification :

1. **Ne pas** improviser ou deviner
2. **Documenter** clairement le problème rencontré
3. **Demander** des clarifications à l'utilisateur
4. **Proposer** des alternatives si pertinent
5. **Attendre** validation avant de procéder

### Exemples de situations nécessitant clarification :
- Dépendance manquante ou incompatible
- Ambiguïté dans les spécifications de la tâche
- Choix technique entre plusieurs approches valides
- Erreur Tesseract système (pas de la crate)

---

## 🔍 Points d'Attention Spécifiques au Projet

### Tesseract
- Toujours tester qu'une image de test existe avant de l'utiliser
- Gérer les erreurs d'initialisation Tesseract proprement
- Documenter les limitations connues de Tesseract

### Images
- Ne pas créer d'images synthétiques (consommation tokens)
- L'utilisateur fournira ses propres images de test
- Supporter les formats courants : PNG, JPG, TIFF

### Configuration
- Permettre flexibilité dans les configurations
- Documenter l'effet de chaque paramètre
- Fournir des présets sensés pour cas d'usage courants

---

## 📈 Suivi de Progression

Le fichier `TODO.md` contient :
- Liste complète des tâches par phase
- Statut de chaque tâche (`[ ]` ou `[x]`)
- Statistiques de progression

**Important** : Mettre à jour les statistiques à chaque tâche complétée.

---

## 🎓 Apprentissage

Ce projet est **éducatif**. Le code doit donc :
- Être clair et lisible
- Être bien documenté
- Démontrer les bonnes pratiques Rust
- Permettre de comprendre le fonctionnement de Tesseract

Ne pas optimiser prématurément. La clarté prime sur la performance.

---

**Version** : 1.1  
**Dernière mise à jour** : 2026-02-16  
**Projet** : Text Recognition - OCR Tesseract Learning
