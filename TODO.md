# TODO - Text Recognition

Liste des tâches à réaliser pour le projet Text Recognition.

**Instructions** : Ne traiter qu'**UNE SEULE** tâche à la fois. Marquer la tâche comme complétée en changeant `[ ]` en `[x]` une fois terminée et validée.

---

## Phase 1 : Fondations

- [x] **1.1** - Initialiser projet Cargo (`cargo init --lib`)
- [x] **1.2** - Configurer `Cargo.toml` avec dépendances de base
- [x] **1.3** - Créer structure de répertoires (`src/`, `tests/`, `test_images/`, `docs/`)
- [x] **1.4** - Créer `src/config.rs` avec structure `OcrConfig` minimale
- [x] **1.5** - Créer enum `PageSegMode` avec les 14 variantes
- [x] **1.6** - Implémenter `OcrConfig::default()`
- [x] **1.7** - Créer `src/ocr.rs` avec structure `OcrEngine`
- [x] **1.8** - Implémenter `OcrEngine::new()`
- [x] **1.9** - Implémenter `OcrEngine::extract_text_from_file()`
- [x] **1.10** - Créer `src/lib.rs` avec exports publics
- [x] **1.11** - Créer `src/main.rs` avec CLI minimale (clap)
- [x] **1.12** - Test manuel avec une image simple

---

## Phase 2 : Configuration Complète

- [x] **2.1** - Ajouter conversion `PageSegMode` → Tesseract PSM
- [x] **2.2** - Ajouter champ `tesseract_variables: HashMap<String, String>` dans `OcrConfig`
- [x] **2.3** - Implémenter application des variables dans `OcrEngine`
- [x] **2.4** - Créer `OcrConfig::document_preset()`
- [x] **2.5** - Créer `OcrConfig::screenshot_preset()`
- [x] **2.6** - Créer `OcrConfig::single_line_preset()`
- [x] **2.7** - Créer `OcrConfig::photo_preset()`
- [x] **2.8** - Ajouter option CLI `--psm`
- [x] **2.9** - Ajouter option CLI `--lang`
- [x] **2.10** - Créer tests unitaires pour les présets

---

## Phase 3 : Prétraitement

- [x] **3.1** - Créer `src/preprocessing.rs`
- [x] **3.2** - Créer structure `PreprocessingConfig`
- [x] **3.3** - Créer enum `BinarizationMethod`
- [x] **3.4** - Implémenter `to_grayscale()`
- [x] **3.5** - Implémenter `binarize()` avec méthode Otsu
- [x] **3.6** - Implémenter `binarize()` avec seuil fixe
- [x] **3.7** - Implémenter `binarize()` avec méthode adaptative
- [x] **3.8** - Implémenter `adjust_contrast()`
- [x] **3.9** - Implémenter `denoise()`
- [x] **3.10** - Implémenter `deskew()`
- [x] **3.11** - Implémenter `preprocess_image()` (pipeline complet)
- [x] **3.12** - Intégrer prétraitement dans `OcrEngine`
- [x] **3.13** - Ajouter option CLI `--preprocess`
- [x] **3.14** - Créer tests unitaires du prétraitement

---

## Phase 4 : Métriques

- [x] **4.1** - Créer `src/metrics.rs`
- [x] **4.2** - Créer structure `OcrMetrics`
- [x] **4.3** - Créer enum `TextError`
- [x] **4.4** - Implémenter distance de Levenshtein
- [x] **4.5** - Implémenter `calculate_cer()`
- [x] **4.6** - Implémenter `calculate_wer()`
- [x] **4.7** - Implémenter `compare_ocr_result()`
- [x] **4.8** - Implémenter `generate_diff_report()`
- [x] **4.9** - Ajouter option CLI `--expected`
- [x] **4.10** - Ajouter option CLI `--metrics`
- [x] **4.11** - Créer tests unitaires des métriques

---

## Phase 5 : Tests

- [x] **5.1** - Créer structure `resources/` avec sous-dossiers (simple/medium/complex/expected)
- [ ] **5.2** - Organiser les images existantes dans `resources/simple/` (3 images)
- [ ] **5.3** - Organiser les images existantes dans `resources/medium/` (3 images)
- [ ] **5.4** - Organiser les images existantes dans `resources/complex/` (2 images)
- [ ] **5.5** - Créer fichiers `.txt` correspondants dans `resources/expected/`
- [ ] **5.6** - Créer `tests/integration_tests.rs` avec tests basiques
- [ ] **5.7** - Créer `tests/psm_tests.rs` avec tests de tous les modes PSM
- [ ] **5.8** - Créer `tests/preprocessing_tests.rs` avec tests de prétraitement
- [ ] **5.9** - Créer `tests/metrics_tests.rs` avec tests de métriques
- [ ] **5.10** - Ajouter option CLI `--test-all-psm`
- [ ] **5.11** - Corriger les bugs identifiés par les tests

---

## Phase 6 : Documentation

- [ ] **6.1** - Créer `README.md` avec description du projet
- [ ] **6.2** - Ajouter section Installation dans README
- [ ] **6.3** - Ajouter exemples d'utilisation CLI dans README
- [ ] **6.4** - Ajouter exemples d'utilisation lib dans README
- [ ] **6.5** - Créer `docs/parametrage-tesseract.md` avec guide des PSM
- [ ] **6.6** - Ajouter section variables Tesseract dans la doc
- [ ] **6.7** - Ajouter section prétraitement dans la doc
- [ ] **6.8** - Ajouter tableaux de résultats dans la doc
- [ ] **6.9** - Documenter le code avec rustdoc (commentaires ///)
- [ ] **6.10** - Générer et vérifier la documentation (`cargo doc --open`)

---

## Phase 7 : Extensions (Optionnel)

- [ ] **7.1** - Implémenter vraie fonction `deskew()` avec détection d'angle
- [ ] **7.2** - Support de fichiers de configuration JSON/TOML
- [ ] **7.3** - Mode batch (traiter plusieurs images)
- [ ] **7.4** - Export de métriques en CSV
- [ ] **7.5** - Visualisation des bounding boxes (format HOCR)
- [ ] **7.6** - Comparaison de performances entre prétraitements
- [ ] **7.7** - Support d'autres langues (téléchargement auto de modèles)
- [ ] **7.8** - Interface web simple

---

## Statistiques

- **Total tâches Phase 1-6** : 67 tâches
- **Tâches complétées** : 48
- **Tâches restantes** : 19
- **Progression** : 71.6%

---

**Dernière mise à jour** : 2026-02-13
