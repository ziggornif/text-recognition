# Ressources de Test

Organisation des images et fichiers de test pour le projet Text Recognition.

## Structure

```
resources/
├── simple/          # Images simples (texte clair, fond uni, haute qualité)
├── medium/          # Images de difficulté moyenne (qualité variable, fond simple)
├── complex/         # Images complexes (fond texturé, bruit, déformations)
├── expected/        # Fichiers .txt contenant le texte attendu pour chaque image
└── img-*.png        # Images de test existantes (à organiser dans les sous-dossiers)
```

## Convention de Nommage

Pour chaque image de test, créer un fichier texte correspondant dans `expected/` :

- Image : `simple/document_01.png`
- Texte attendu : `expected/document_01.txt`

## Catégories

### Simple
Images avec :
- Texte clair et net
- Fond uni (blanc ou très clair)
- Haute résolution
- Pas de bruit ou déformations
- Police standard

**Exemples** : Documents scannés de qualité, texte imprimé propre

### Medium
Images avec :
- Qualité variable
- Fond simple mais pas forcément uniforme
- Résolution moyenne
- Légères variations de contraste
- Polices variées

**Exemples** : Photos de documents, captures d'écran

### Complex
Images avec :
- Fond texturé ou complexe
- Bruit important
- Basse résolution
- Déformations (perspective, rotation)
- Polices manuscrites ou stylisées
- Faible contraste

**Exemples** : Photos de panneaux, texte sur photos, documents dégradés

## Utilisation

Les images de ce dossier sont utilisées pour :
- Tests manuels avec le CLI
- Tests d'intégration automatisés
- Validation du fonctionnement de base de l'OCR
- Tests de différents modes PSM
- Évaluation de l'impact du prétraitement
- Calcul des métriques de qualité (CER, WER)

## Notes

- Formats supportés : PNG, JPG, JPEG, TIFF
- Taille recommandée : entre 500x500 et 2000x2000 pixels
- Les images existantes (img-*.png) peuvent être déplacées dans les sous-dossiers appropriés selon leur difficulté
