# Guide de Paramétrage de Tesseract OCR

Ce document fournit un guide complet sur les différents paramètres de Tesseract OCR, en particulier les **modes de segmentation de page (PSM)**, qui sont essentiels pour obtenir de bons résultats.

## Table des matières

- [Introduction](#introduction)
- [Modes de Segmentation de Page (PSM)](#modes-de-segmentation-de-page-psm)
- [Guide de sélection du PSM](#guide-de-sélection-du-psm)
- [Langues](#langues)
- [Variables Tesseract](#variables-tesseract)
- [Résolution (DPI)](#résolution-dpi)
- [Bonnes pratiques](#bonnes-pratiques)

---

## Introduction

Tesseract OCR est un moteur de reconnaissance de texte puissant, mais ses résultats dépendent fortement de la **configuration** utilisée. Le paramètre le plus important est le **mode de segmentation de page (PSM)**, qui indique à Tesseract comment interpréter la structure de l'image.

**Principe** : Avant de reconnaître le texte, Tesseract doit d'abord **segmenter** l'image pour identifier les zones de texte, les lignes, les mots et les caractères. Le mode PSM guide cette segmentation.

---

## Modes de Segmentation de Page (PSM)

Tesseract propose **14 modes PSM** (numérotés de 0 à 13). Chaque mode est optimisé pour un type de mise en page spécifique.

### PSM 0 - OSD Only (Orientation and Script Detection)

**Description** : Détecte uniquement l'orientation et le script de l'image, sans extraire de texte.

**Usage** :
- Déterminer l'orientation d'un document scanné (0°, 90°, 180°, 270°)
- Identifier le script (latin, arabe, chinois, etc.)

**Quand l'utiliser** :
- Pré-traitement pour corriger l'orientation avant l'OCR
- Analyse de documents dont l'orientation est inconnue

**Exemple CLI** :
```bash
cargo run -- image.png --psm 0
```

**Note** : N'extrait pas de texte, retourne uniquement les métadonnées d'orientation.

---

### PSM 1 - Automatic Page Segmentation with OSD

**Description** : Segmentation automatique avec détection d'orientation et de script.

**Usage** :
- Documents complets dont l'orientation peut varier
- Première tentative sur un document inconnu

**Quand l'utiliser** :
- Documents scannés qui peuvent être mal orientés
- Besoin de robustesse face à différentes orientations

**Avantages** :
- Correction automatique de l'orientation
- Adaptabilité maximale

**Inconvénients** :
- Plus lent (détection OSD supplémentaire)
- Peut échouer sur des images de faible qualité

**Exemple CLI** :
```bash
cargo run -- document.png --psm 1
```

---

### PSM 2 - Automatic Page Segmentation (no OSD)

**Description** : Segmentation automatique sans détection d'orientation.

**Usage** :
- Documents correctement orientés
- Alternative plus rapide au PSM 1

**Quand l'utiliser** :
- Images déjà correctement orientées
- Besoin de vitesse

**Note** : Similaire au PSM 3 mais avec des heuristiques légèrement différentes.

**Exemple CLI** :
```bash
cargo run -- document.png --psm 2
```

---

### PSM 3 - Fully Automatic Page Segmentation (DEFAULT)

**Description** : Segmentation entièrement automatique sans détection d'orientation ni de script. **C'est le mode par défaut.**

**Usage** :
- Documents texte classiques
- Mise en page standard (paragraphes, colonnes)
- **Premier choix pour la plupart des documents**

**Quand l'utiliser** :
- Documents bien formatés et orientés
- Texte organisé en paragraphes ou colonnes
- Pages de livres, articles, rapports

**Avantages** :
- Bon équilibre entre qualité et vitesse
- Fonctionne bien sur la plupart des documents

**Inconvénients** :
- Peut sur-segmenter du texte simple
- Moins efficace sur des mises en page non conventionnelles

**Exemple CLI** :
```bash
cargo run -- document.png --psm 3
# Ou simplement (mode par défaut) :
cargo run -- document.png
```

---

### PSM 4 - Single Column of Text

**Description** : Assume une seule colonne de texte de taille variable.

**Usage** :
- Documents à une seule colonne
- Articles, lettres, documents simples

**Quand l'utiliser** :
- Texte organisé en une seule colonne verticale
- Pas de mise en page multi-colonnes
- Documents linéaires (du haut vers le bas)

**Avantages** :
- Meilleure précision sur les documents à colonne unique
- Évite les erreurs de segmentation multi-colonnes

**Exemple CLI** :
```bash
cargo run -- lettre.png --psm 4
```

**Cas d'usage typiques** :
- Lettres
- E-mails imprimés
- Documents officiels simples

---

### PSM 5 - Single Vertical Block of Text

**Description** : Assume un seul bloc uniforme de texte aligné verticalement.

**Usage** :
- Blocs de texte denses et uniformes
- Texte justifié ou aligné

**Quand l'utiliser** :
- Paragraphes denses sans espacement important
- Texte continu sans variations de mise en page

**Différence avec PSM 4** :
- PSM 4 : Colonne de taille variable (plusieurs blocs possibles)
- PSM 5 : Bloc unique et uniforme

**Exemple CLI** :
```bash
cargo run -- bloc_texte.png --psm 5
```

---

### PSM 6 - Single Uniform Block of Text

**Description** : Assume un seul bloc de texte uniforme (sans contrainte de verticalité).

**Usage** :
- Blocs de texte simples
- Paragraphes isolés
- Zones de texte extraites

**Quand l'utiliser** :
- Une seule zone de texte dans l'image
- Texte homogène sans variations importantes
- Post-its, étiquettes, panneaux

**Avantages** :
- Simple et efficace pour les blocs isolés
- Bon pour les extraits de texte

**Exemple CLI** :
```bash
cargo run -- paragraphe.png --psm 6
```

**Cas d'usage typiques** :
- Paragraphe unique
- Zone de texte extraite d'un document plus grand
- Étiquette de produit

---

### PSM 7 - Single Line of Text

**Description** : Traite l'image comme une seule ligne de texte.

**Usage** :
- Lignes de texte isolées
- Titres, en-têtes
- Champs de formulaires

**Quand l'utiliser** :
- Image contenant une seule ligne horizontale
- Extraction de titres
- Lecture de champs de formulaires

**Avantages** :
- Très rapide
- Précis pour les lignes uniques
- Pas de sur-segmentation

**Inconvénients** :
- Échoue si l'image contient plusieurs lignes

**Exemple CLI** :
```bash
cargo run -- titre.png --psm 7
```

**Cas d'usage typiques** :
- Titres de documents
- En-têtes de sections
- Lignes de formulaires
- Sous-titres

---

### PSM 8 - Single Word

**Description** : Traite l'image comme un seul mot.

**Usage** :
- Mots isolés
- Extraction de mots spécifiques
- CAPTCHA simples

**Quand l'utiliser** :
- Image contenant un seul mot
- Reconnaissance de mots isolés (étiquettes, tags)

**Avantages** :
- Optimisé pour les mots uniques
- Rapide

**Inconvénients** :
- Échoue si plusieurs mots sont présents

**Exemple CLI** :
```bash
cargo run -- mot.png --psm 8
```

**Cas d'usage typiques** :
- Tags, étiquettes
- Noms isolés
- CAPTCHA textuels simples

---

### PSM 9 - Single Word in a Circle

**Description** : Traite l'image comme un seul mot dans un cercle.

**Usage** :
- Texte circulaire
- Logos avec texte en arc
- Sceaux, tampons

**Quand l'utiliser** :
- Texte courbé ou circulaire
- Logos
- Sceaux officiels

**Note** : Mode très spécialisé, rarement utilisé.

**Exemple CLI** :
```bash
cargo run -- logo_circulaire.png --psm 9
```

---

### PSM 10 - Single Character

**Description** : Traite l'image comme un seul caractère.

**Usage** :
- Reconnaissance de caractères isolés
- CAPTCHA caractère par caractère
- Validation de saisie

**Quand l'utiliser** :
- Image contenant un seul caractère
- Reconnaissance caractère par caractère

**Avantages** :
- Très précis pour les caractères uniques
- Utile pour la reconnaissance séquentielle

**Exemple CLI** :
```bash
cargo run -- caractere.png --psm 10
```

**Cas d'usage typiques** :
- CAPTCHA
- Reconnaissance de plaques d'immatriculation (caractère par caractère)
- Validation de formulaires

---

### PSM 11 - Sparse Text

**Description** : Trouve autant de texte que possible sans ordre particulier.

**Usage** :
- Texte épars, dispersé
- Captures d'écran
- Images avec texte dans différentes zones

**Quand l'utiliser** :
- Texte non structuré
- Captures d'écran d'interfaces
- Images avec du texte dispersé (affiches, publicités)

**Avantages** :
- Trouve du texte partout dans l'image
- Très tolérant à la mise en page

**Inconvénients** :
- Ordre du texte peut être incorrect
- Peut capturer du bruit

**Exemple CLI** :
```bash
cargo run -- screenshot.png --psm 11
```

**Cas d'usage typiques** :
- Captures d'écran d'applications
- Affiches publicitaires
- Memes, images avec texte dispersé
- Panneaux d'affichage

---

### PSM 12 - Sparse Text with OSD

**Description** : Texte épars avec détection d'orientation et de script.

**Usage** :
- Même que PSM 11, mais avec correction d'orientation

**Quand l'utiliser** :
- Texte épars dont l'orientation est incertaine
- Captures d'écran mal orientées

**Exemple CLI** :
```bash
cargo run -- screenshot_rotated.png --psm 12
```

---

### PSM 13 - Raw Line

**Description** : Traite l'image comme une seule ligne de texte, sans utiliser de modèle de langue Tesseract spécifique.

**Usage** :
- Ligne de texte brute
- Bypass des modèles de langue

**Quand l'utiliser** :
- Texte technique (codes, formules)
- Besoin de reconnaissance brute sans correction linguistique

**Différence avec PSM 7** :
- PSM 7 : Utilise les modèles de langue pour améliorer la reconnaissance
- PSM 13 : Reconnaissance brute sans post-traitement linguistique

**Exemple CLI** :
```bash
cargo run -- code_ligne.png --psm 13
```

**Cas d'usage typiques** :
- Codes techniques (hexadécimal, base64)
- Formules mathématiques
- Lignes de commande

---

## Guide de Sélection du PSM

Utilisez ce tableau pour choisir le bon mode PSM selon votre type de document :

| Type de document | PSM recommandé | PSM alternatif |
|------------------|----------------|----------------|
| Document texte complet (livre, article) | **3** (Auto) | 1 (Auto with OSD) |
| Document à une colonne (lettre) | **4** (Single Column) | 3 (Auto) |
| Paragraphe unique | **6** (Single Block) | 5 (Vertical Block) |
| Titre, en-tête | **7** (Single Line) | 6 (Single Block) |
| Champ de formulaire (une ligne) | **7** (Single Line) | 13 (Raw Line) |
| Mot unique (tag, étiquette) | **8** (Single Word) | 7 (Single Line) |
| Caractère unique | **10** (Single Char) | 8 (Single Word) |
| Capture d'écran (UI, interface) | **11** (Sparse Text) | 12 (Sparse + OSD) |
| Affiche, publicité | **11** (Sparse Text) | 3 (Auto) |
| Document mal orienté | **1** (Auto with OSD) | 12 (Sparse + OSD) |
| Code technique (ligne) | **13** (Raw Line) | 7 (Single Line) |
| Logo circulaire | **9** (Word in Circle) | - |

---

## Arbre de Décision

```
Votre image contient-elle...

└─ Un seul caractère ?
   └─ OUI → PSM 10 (Single Character)
   └─ NON ↓

└─ Un seul mot ?
   └─ OUI → PSM 8 (Single Word)
   └─ NON ↓

└─ Une seule ligne de texte ?
   └─ OUI → PSM 7 (Single Line) ou PSM 13 (Raw Line pour code)
   └─ NON ↓

└─ Un paragraphe unique ou bloc de texte ?
   └─ OUI → PSM 6 (Single Block)
   └─ NON ↓

└─ Du texte épars/dispersé (screenshot, affiche) ?
   └─ OUI → PSM 11 (Sparse Text)
   └─ NON ↓

└─ Un document structuré classique ?
   └─ Une colonne → PSM 4 (Single Column)
   └─ Plusieurs colonnes ou complexe → PSM 3 (Auto)
   └─ Orientation incertaine → PSM 1 (Auto with OSD)
```

---

## Langues

Tesseract supporte plus de 100 langues. Les plus courantes :

| Code | Langue |
|------|--------|
| `fra` | Français |
| `eng` | Anglais |
| `deu` | Allemand |
| `spa` | Espagnol |
| `ita` | Italien |
| `por` | Portugais |
| `nld` | Néerlandais |
| `rus` | Russe |
| `jpn` | Japonais |
| `chi_sim` | Chinois simplifié |
| `chi_tra` | Chinois traditionnel |
| `kor` | Coréen |
| `ara` | Arabe |

### Combiner plusieurs langues

```bash
# Français + Anglais
cargo run -- image.png --language fra+eng

# Allemand + Anglais
cargo run -- image.png --language deu+eng
```

### Installation de langues supplémentaires

**Debian/Ubuntu** :
```bash
sudo apt-get install tesseract-ocr-fra tesseract-ocr-eng tesseract-ocr-deu
```

**macOS** :
```bash
brew install tesseract-lang
```

**Vérifier les langues installées** :
```bash
tesseract --list-langs
```

---

## Variables Tesseract

Tesseract permet de configurer son comportement via des **variables internes**. Ces variables offrent un contrôle fin sur le processus de reconnaissance et peuvent améliorer significativement les résultats dans certains cas.

### Qu'est-ce qu'une variable Tesseract ?

Les variables Tesseract (aussi appelées **config vars** ou **parameters**) sont des paramètres internes du moteur OCR qui contrôlent :
- Le traitement des images
- Les seuils de détection
- Les règles linguistiques
- Le formatage de la sortie
- Les optimisations de performance

### Comment utiliser les variables ?

Dans ce projet, les variables peuvent être configurées via la structure `OcrConfig` :

```rust
use std::collections::HashMap;
use text_recognition::{OcrConfig, OcrEngine, PageSegMode};

let mut config = OcrConfig::default();

// Créer un HashMap de variables Tesseract
let mut variables = HashMap::new();
variables.insert("tessedit_char_whitelist".to_string(), "0123456789".to_string());

config.tesseract_variables = variables;

let mut engine = OcrEngine::new(config)?;
let text = engine.extract_text_from_file("image.png")?;
```

### Variables Essentielles

Voici les variables les plus utiles, organisées par catégorie.

---

#### 1. Filtrage de Caractères

##### `tessedit_char_whitelist`

**Description** : Liste blanche de caractères autorisés. Seuls ces caractères seront reconnus.

**Usage** : Restreindre la reconnaissance à un ensemble spécifique de caractères.

**Exemples** :

```rust
// Chiffres uniquement
variables.insert("tessedit_char_whitelist".to_string(), "0123456789".to_string());

// Lettres majuscules uniquement
variables.insert("tessedit_char_whitelist".to_string(), "ABCDEFGHIJKLMNOPQRSTUVWXYZ".to_string());

// Hexadécimal
variables.insert("tessedit_char_whitelist".to_string(), "0123456789ABCDEFabcdef".to_string());

// Code-barres numérique
variables.insert("tessedit_char_whitelist".to_string(), "0123456789-".to_string());
```

**Cas d'usage** :
- Codes postaux (chiffres uniquement)
- Plaques d'immatriculation (lettres + chiffres)
- Codes hexadécimaux
- Formulaires numériques

---

##### `tessedit_char_blacklist`

**Description** : Liste noire de caractères interdits. Ces caractères ne seront jamais reconnus.

**Usage** : Exclure des caractères connus pour être absents du texte.

**Exemples** :

```rust
// Exclure ponctuation spécifique
variables.insert("tessedit_char_blacklist".to_string(), "!@#$%^&*()".to_string());

// Exclure chiffres
variables.insert("tessedit_char_blacklist".to_string(), "0123456789".to_string());

// Exclure caractères ambigus (1/l, 0/O)
variables.insert("tessedit_char_blacklist".to_string(), "1l0O".to_string());
```

**Cas d'usage** :
- Texte littéraire (pas de chiffres)
- Éviter confusion entre caractères similaires
- Documents avec contraintes linguistiques

---

#### 2. Post-traitement Linguistique

##### `tessedit_pageseg_mode`

**Description** : Définit le mode de segmentation de page (équivalent au PSM).

**Usage** : Alternative à la configuration via l'enum `PageSegMode`.

**Exemples** :

```rust
// PSM 7 (Single Line)
variables.insert("tessedit_pageseg_mode".to_string(), "7".to_string());

// PSM 11 (Sparse Text)
variables.insert("tessedit_pageseg_mode".to_string(), "11".to_string());
```

**Note** : Il est préférable d'utiliser le champ `page_seg_mode` de `OcrConfig` plutôt que cette variable.

---

##### `load_system_dawg`
##### `load_freq_dawg`
##### `load_number_dawg`
##### `load_punc_dawg`
##### `load_bigram_dawg`

**Description** : Active/désactive le chargement de dictionnaires linguistiques (DAWG = Directed Acyclic Word Graph).

- `load_system_dawg` : Dictionnaire système (mots courants)
- `load_freq_dawg` : Mots fréquents
- `load_number_dawg` : Nombres
- `load_punc_dawg` : Ponctuation
- `load_bigram_dawg` : Bigrammes (paires de mots)

**Valeurs** : `"T"` (activé) ou `"F"` (désactivé)

**Usage** : Désactiver les dictionnaires pour du texte technique, codes, ou formules.

**Exemples** :

```rust
// Désactiver tous les dictionnaires pour du code source
variables.insert("load_system_dawg".to_string(), "F".to_string());
variables.insert("load_freq_dawg".to_string(), "F".to_string());
variables.insert("load_number_dawg".to_string(), "F".to_string());
variables.insert("load_punc_dawg".to_string(), "F".to_string());
variables.insert("load_bigram_dawg".to_string(), "F".to_string());

// Activer uniquement nombres et ponctuation pour des données structurées
variables.insert("load_system_dawg".to_string(), "F".to_string());
variables.insert("load_freq_dawg".to_string(), "F".to_string());
variables.insert("load_number_dawg".to_string(), "T".to_string());
variables.insert("load_punc_dawg".to_string(), "T".to_string());
variables.insert("load_bigram_dawg".to_string(), "F".to_string());
```

**Cas d'usage** :
- Code source (désactiver tous les dictionnaires)
- Codes techniques (hexadécimal, base64)
- Formules mathématiques
- Données structurées (CSV, JSON)

---

#### 3. Détection et Traitement

##### `textord_min_linesize`

**Description** : Taille minimale de ligne détectée (en pixels).

**Valeur par défaut** : `0.5`

**Usage** : Ajuster la détection de lignes pour du texte très petit ou très grand.

**Exemples** :

```rust
// Texte très petit
variables.insert("textord_min_linesize".to_string(), "0.3".to_string());

// Texte très grand
variables.insert("textord_min_linesize".to_string(), "1.5".to_string());
```

---

##### `preserve_interword_spaces`

**Description** : Préserve les espaces multiples entre les mots.

**Valeurs** : `"0"` (non) ou `"1"` (oui)

**Valeur par défaut** : `"0"`

**Usage** : Utile pour les documents avec espacement significatif (tableaux, code).

**Exemples** :

```rust
// Préserver espaces multiples
variables.insert("preserve_interword_spaces".to_string(), "1".to_string());
```

**Cas d'usage** :
- Code source indenté
- Tableaux alignés avec espaces
- Documents avec mise en page spécifique

---

##### `textord_noise_sizelimit`

**Description** : Taille maximale (en pixels) pour considérer un élément comme du bruit.

**Valeur par défaut** : `7.0`

**Usage** : Ajuster le filtrage du bruit.

**Exemples** :

```rust
// Filtrer davantage de bruit (images de mauvaise qualité)
variables.insert("textord_noise_sizelimit".to_string(), "10.0".to_string());

// Conserver les petits éléments (petits caractères)
variables.insert("textord_noise_sizelimit".to_string(), "3.0".to_string());
```

---

#### 4. Seuils de Confiance

##### `tessedit_char_blacklist`

**Description** : (déjà décrite ci-dessus)

---

##### `classify_bln_numeric_mode`

**Description** : Active le mode numérique pour améliorer la reconnaissance des chiffres.

**Valeurs** : `"0"` (non) ou `"1"` (oui)

**Valeur par défaut** : `"0"`

**Usage** : Améliorer la reconnaissance des chiffres (codes postaux, numéros).

**Exemples** :

```rust
// Activer mode numérique
variables.insert("classify_bln_numeric_mode".to_string(), "1".to_string());
```

---

#### 5. Sortie et Formatage

##### `hocr_font_info`

**Description** : Inclure les informations de police dans la sortie HOCR.

**Valeurs** : `"0"` (non) ou `"1"` (oui)

**Usage** : Extraction de métadonnées de police.

**Exemples** :

```rust
variables.insert("hocr_font_info".to_string(), "1".to_string());
```

---

### Cas d'Usage Pratiques

#### Exemple 1 : Reconnaissance de Code Postal

```rust
use std::collections::HashMap;
use text_recognition::{OcrConfig, OcrEngine, PageSegMode};

let mut config = OcrConfig {
    page_seg_mode: PageSegMode::SingleLine,
    language: "eng".to_string(),
    ..Default::default()
};

let mut variables = HashMap::new();
variables.insert("tessedit_char_whitelist".to_string(), "0123456789".to_string());
variables.insert("classify_bln_numeric_mode".to_string(), "1".to_string());

config.tesseract_variables = variables;

let mut engine = OcrEngine::new(config)?;
let code_postal = engine.extract_text_from_file("code_postal.png")?;
```

---

#### Exemple 2 : Reconnaissance de Code Source

```rust
use std::collections::HashMap;
use text_recognition::{OcrConfig, OcrEngine, PageSegMode};

let mut config = OcrConfig {
    page_seg_mode: PageSegMode::Auto,
    language: "eng".to_string(),
    ..Default::default()
};

let mut variables = HashMap::new();

// Désactiver tous les dictionnaires
variables.insert("load_system_dawg".to_string(), "F".to_string());
variables.insert("load_freq_dawg".to_string(), "F".to_string());
variables.insert("load_number_dawg".to_string(), "F".to_string());
variables.insert("load_punc_dawg".to_string(), "F".to_string());
variables.insert("load_bigram_dawg".to_string(), "F".to_string());

// Préserver espaces multiples
variables.insert("preserve_interword_spaces".to_string(), "1".to_string());

config.tesseract_variables = variables;

let mut engine = OcrEngine::new(config)?;
let code = engine.extract_text_from_file("code_source.png")?;
```

---

#### Exemple 3 : Plaque d'Immatriculation

```rust
use std::collections::HashMap;
use text_recognition::{OcrConfig, OcrEngine, PageSegMode};

let mut config = OcrConfig {
    page_seg_mode: PageSegMode::SingleLine,
    language: "eng".to_string(),
    ..Default::default()
};

let mut variables = HashMap::new();
variables.insert("tessedit_char_whitelist".to_string(), "ABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789-".to_string());

config.tesseract_variables = variables;

let mut engine = OcrEngine::new(config)?;
let plaque = engine.extract_text_from_file("plaque.png")?;
```

---

### Liste des Variables Principales

Voici un tableau récapitulatif des variables les plus utiles :

| Variable | Type | Description | Cas d'usage |
|----------|------|-------------|-------------|
| `tessedit_char_whitelist` | String | Caractères autorisés | Codes, formulaires numériques |
| `tessedit_char_blacklist` | String | Caractères interdits | Éviter confusion, contraintes |
| `load_system_dawg` | T/F | Dictionnaire système | Désactiver pour code technique |
| `load_freq_dawg` | T/F | Mots fréquents | Désactiver pour code technique |
| `load_number_dawg` | T/F | Nombres | Désactiver pour texte pur |
| `load_punc_dawg` | T/F | Ponctuation | Désactiver pour codes |
| `load_bigram_dawg` | T/F | Bigrammes | Désactiver pour code technique |
| `preserve_interword_spaces` | 0/1 | Préserver espaces multiples | Code source, tableaux |
| `textord_min_linesize` | Float | Taille min de ligne (pixels) | Texte très petit/grand |
| `textord_noise_sizelimit` | Float | Seuil de filtrage du bruit | Images de mauvaise qualité |
| `classify_bln_numeric_mode` | 0/1 | Mode numérique | Chiffres, codes postaux |
| `hocr_font_info` | 0/1 | Infos police en HOCR | Extraction métadonnées |

---

### Lister Toutes les Variables Disponibles

Pour voir toutes les variables disponibles dans votre installation de Tesseract :

```bash
tesseract --print-parameters
```

Cela affichera une liste complète avec les valeurs par défaut.

---

### Bonnes Pratiques pour les Variables

1. **Commencer simple** : N'ajoutez des variables que si le comportement par défaut ne convient pas.

2. **Tester l'impact** : Chaque variable peut affecter les résultats. Testez avec et sans pour mesurer l'impact.

3. **Documenter vos configurations** : Gardez une trace des variables utilisées et de leurs effets.

4. **Combiner avec PSM** : Les variables fonctionnent mieux quand le bon PSM est déjà sélectionné.

5. **Whitelist vs Blacklist** : Ne jamais utiliser les deux en même temps (résultats imprévisibles).

6. **Performance** : Désactiver les dictionnaires (`load_*_dawg`) peut améliorer la vitesse pour du texte technique.

---

### Ressources sur les Variables

- [Liste complète des variables Tesseract](https://tesseract-ocr.github.io/tessdoc/tess3/ControlParams.html)
- [Configuration avancée](https://tesseract-ocr.github.io/tessdoc/ImproveQuality.html)
- [Commande `--print-parameters`](https://tesseract-ocr.github.io/tessdoc/)

---

## Résolution (DPI)

La résolution de l'image (DPI - Dots Per Inch) affecte la qualité de l'OCR.

### Recommandations

| Résolution | Qualité | Usage |
|------------|---------|-------|
| < 200 DPI | Faible | Éviter si possible |
| **300 DPI** | **Optimale** | **Recommandé** |
| 400-600 DPI | Très bonne | Documents de haute qualité |
| > 600 DPI | Excellente | Documents anciens, détériorés |

### Spécifier le DPI

```bash
cargo run -- image.png --dpi 300
```

**Note** : Si l'image ne contient pas de métadonnées DPI, Tesseract utilisera la valeur fournie.

---

## Bonnes Pratiques

### 1. Toujours tester plusieurs PSM

Le meilleur PSM dépend de votre image spécifique. Utilisez `--test-all-psm` :

```bash
cargo run -- image.png --test-all-psm --expected expected.txt
```

Cela vous montrera quel PSM donne les meilleurs résultats (CER, WER).

### 2. Prétraiter les images de faible qualité

Si l'image est bruitée, floue ou de faible contraste :

```bash
cargo run -- image.png --preprocess --psm 3
```

Ou plus finement :

```bash
cargo run -- image.png --grayscale --binarize --denoise --contrast 1.5 --psm 3
```

### 3. Utiliser la bonne langue

Spécifiez toujours la langue correcte :

```bash
cargo run -- image.png --language fra --psm 3
```

### 4. Ajuster le DPI si nécessaire

Pour les images de faible résolution :

```bash
cargo run -- low_res.png --dpi 150 --psm 3
```

### 5. Mesurer la qualité

Comparez toujours avec un texte de référence :

```bash
cargo run -- image.png --expected expected.txt --metrics --psm 3
```

### 6. Documenter vos résultats

Gardez une trace des configurations qui fonctionnent bien pour vos types d'images :

- Document type A : PSM 3, prétraitement Otsu
- Screenshot : PSM 11, sans prétraitement
- Formulaires : PSM 7, binarisation adaptative

---

## Exemples Pratiques

### Document texte classique

```bash
cargo run -- document.pdf.png \
  --language fra \
  --psm 3 \
  --dpi 300
```

### Capture d'écran d'application

```bash
cargo run -- screenshot.png \
  --language eng \
  --psm 11 \
  --preprocess
```

### Photo de document (qualité moyenne)

```bash
cargo run -- photo_doc.jpg \
  --language fra \
  --psm 3 \
  --grayscale \
  --binarize \
  --binarize-method adaptive \
  --denoise \
  --contrast 1.5 \
  --dpi 300
```

### Extraction de titre

```bash
cargo run -- titre.png \
  --language fra \
  --psm 7 \
  --dpi 300
```

### Code hexadécimal (ligne)

```bash
cargo run -- hex_code.png \
  --language eng \
  --psm 13
```

---

## Ressources

- [Documentation officielle Tesseract](https://tesseract-ocr.github.io/)
- [Tesseract GitHub](https://github.com/tesseract-ocr/tesseract)
- [Guide d'amélioration de la qualité](https://tesseract-ocr.github.io/tessdoc/ImproveQuality.html)
- [Liste des langues supportées](https://tesseract-ocr.github.io/tessdoc/Data-Files-in-different-versions.html)

---

**Auteur** : Projet Text Recognition  
**Date** : 2026-02-14  
**Version** : 1.0
