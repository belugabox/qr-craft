# 🎨 QR Craft

[![Docker](https://img.shields.io/badge/docker-%230db7ed.svg?style=for-the-badge&logo=docker&logoColor=white)](https://hub.docker.com)
[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)](https://www.rust-lang.org/)
[![Dioxus](https://img.shields.io/badge/Dioxus-354458?style=for-the-badge&logo=rust&logoColor=white)](https://dioxuslabs.com/)
[![Tailwind CSS](https://img.shields.io/badge/tailwindcss-%2338B2AC.svg?style=for-the-badge&logo=tailwind-css&logoColor=white)](https://tailwindcss.com/)

Une application web moderne de génération de codes QR construite avec **Rust** et **Dioxus**. Créez, personnalisez et sauvegardez vos codes QR en toute simplicité.

![QR Craft Preview](https://via.placeholder.com/800x400/4F46E5/FFFFFF?text=QR+Craft+Preview)

## ✨ Fonctionnalités

- 🚀 **Génération instantanée** de codes QR
- 🎨 **Personnalisation avancée** : taille, transparence
- 💾 **Sauvegarde persistante** des codes générés
- 📱 **Interface responsive** et moderne
- 🔄 **Chargement automatique** des QR sauvegardés
- 🗑️ **Gestion complète** : charger, supprimer
- 🌐 **Déploiement multi-plateforme** (Web, Desktop)

## 🛠️ Technologies Utilisées

- **Frontend**: [Dioxus](https://dioxuslabs.com/) - Framework Rust pour le web
- **Styling**: [Tailwind CSS](https://tailwindcss.com/) - Framework CSS utilitaire
- **Backend**: [Axum](https://docs.rs/axum/latest/axum/) - Serveur web asynchrone
- **QR Generation**: [qrcode](https://docs.rs/qrcode/latest/qrcode/) - Bibliothèque de génération QR
- **Build**: [Cargo](https://doc.rust-lang.org/cargo/) - Gestionnaire de paquets Rust
- **Container**: [Docker](https://www.docker.com/) - Conteneurisation

## 🚀 Démarrage Rapide

### Avec Docker (Recommandé)

```bash
# Récupérer et lancer la dernière image
docker pull ghcr.io/belugabox/qr-craft:main
docker run -p 8080:8080 ghcr.io/belugabox/qr-craft:main
```

Ou utiliser Docker Compose :

```bash
docker-compose up
```

L'application sera accessible sur [http://localhost:8080](http://localhost:8080)

### Développement Local

#### Prérequis

- [Rust](https://rustup.rs/) (1.70+)
- [Node.js](https://nodejs.org/) et npm
- [Dioxus CLI](https://dioxuslabs.com/learn/0.6/getting_started#the-dx-tool)

#### Installation

1. **Cloner le dépôt**

   ```bash
   git clone https://github.com/belugabox/qr-craft.git
   cd qr-craft
   ```

2. **Installer les dépendances**

   ```bash
   cargo build
   npm install
   ```

3. **Démarrer Tailwind CSS**

   ```bash
   npx tailwindcss -i ./input.css -o ./assets/tailwind.css --watch
   ```

4. **Lancer l'application**
   ```bash
   dx serve --platform web
   ```

## 📖 Utilisation

### Génération de QR Codes

1. **Saisir le contenu** : URL, texte, ou données à encoder
2. **Personnaliser** :
   - **Taille** : 128x128, 256x256, ou 512x512 pixels
   - **Transparence** : Activer/désactiver le fond transparent
3. **Générer** : Cliquer sur "Generate"
4. **Sauvegarder** : Utiliser le bouton "Save" pour conserver le QR

### Gestion des QR Sauvegardés

- **Chargement automatique** au démarrage
- **Charger** un QR sauvegardé pour modification
- **Supprimer** les QR non désirés

## 🏗️ Architecture

```
qr-craft/
├── src/
│   ├── main.rs              # Point d'entrée et composants principaux
│   ├── qr_generator.rs      # Composant de génération QR
│   ├── saved_qr_list.rs     # Composant de gestion des QR sauvegardés
│   └── qrcode.rs            # Fonctions serveur pour les opérations QR
├── assets/                  # Ressources statiques
├── data/                    # Stockage persistant des QR
├── Dockerfile               # Configuration Docker
├── docker-compose.yml       # Configuration Docker Compose
└── Dioxus.toml             # Configuration Dioxus
```

## 🐳 Déploiement

### Images Docker

Les images Docker sont automatiquement publiées sur [GitHub Container Registry](https://ghcr.io/belugabox/qr-craft).

#### Tags disponibles :

- `main` : Dernière version de la branche principale
- `v1.x.x` : Versions taggées
- `sha-<commit>` : Versions spécifiques par commit

#### Utilisation avancée :

```bash
# Version spécifique
docker run -p 8080:8080 ghcr.io/belugabox/qr-craft:v1.0.0

# Avec variables d'environnement
docker run -e PORT=3000 -p 3000:3000 ghcr.io/belugabox/qr-craft:main
```

### Build Local

```bash
# Générer le bundle web
dx bundle --platform web --release --out-dir web

# Construire l'image Docker
docker build -t qr-craft:local .

# Lancer le conteneur
docker run -p 8080:8080 qr-craft:local
```

## 🔧 Développement

### Structure du Projet

```
src/
├── main.rs              # Application principale et layout
├── qr_generator.rs      # Logique de génération QR
├── saved_qr_list.rs     # Gestion de la liste des QR sauvegardés
└── qrcode.rs            # API serveur (génération, sauvegarde, chargement)
```

### Commandes de Développement

```bash
# Compilation
cargo build

# Tests
cargo test

# Linting
cargo clippy

# Formatage
cargo fmt

# Serveur de développement
dx serve --platform web

# Build de production
dx bundle --platform web --release
```

### Ajouter de Nouvelles Fonctionnalités

1. **Nouveau composant** : Créer un fichier dans `src/`
2. **API serveur** : Ajouter des fonctions dans `qrcode.rs`
3. **Styling** : Modifier les classes Tailwind dans les composants
4. **Tests** : Ajouter des tests unitaires et d'intégration

## 🤝 Contribution

Les contributions sont les bienvenues ! Voici comment contribuer :

1. **Fork** le projet
2. **Créer** une branche pour votre fonctionnalité (`git checkout -b feature/AmazingFeature`)
3. **Commit** vos changements (`git commit -m 'Add some AmazingFeature'`)
4. **Push** vers la branche (`git push origin feature/AmazingFeature`)
5. **Ouvrir** une Pull Request

### Guidelines

- Suivre le style de code Rust standard
- Ajouter des tests pour les nouvelles fonctionnalités
- Mettre à jour la documentation si nécessaire
- Respecter les conventions de commit

## 📄 Licence

Ce projet est sous licence MIT - voir le fichier [LICENSE](LICENSE) pour plus de détails.

## 🙏 Remerciements

- [Dioxus](https://dioxuslabs.com/) pour le framework web Rust
- [Tailwind CSS](https://tailwindcss.com/) pour le styling
- [qrcode-rs](https://github.com/kennytm/qrcode-rust) pour la génération QR
- La communauté Rust pour l'écosystème incroyable

## 📞 Support

- 🐛 **Issues** : [GitHub Issues](https://github.com/belugabox/qr-craft/issues)
- 💬 **Discussions** : [GitHub Discussions](https://github.com/belugabox/qr-craft/discussions)
- 📧 **Email** : contact@belugabox.dev

---

<div align="center">
  <p>Fait avec ❤️ et Rust</p>
  <p>
    <a href="https://github.com/belugabox/qr-craft">GitHub</a> •
    <a href="https://hub.docker.com/r/belugabox/qr-craft">Docker Hub</a> •
    <a href="https://dioxuslabs.com/">Dioxus</a>
  </p>
</div>

## Docker

Docker images are automatically published to GitHub Container Registry.

```bash
# Pull and run the latest image
docker pull ghcr.io/belugabox/qr-craft:main
docker run -p 8080:8080 ghcr.io/belugabox/qr-craft:main
```

Or use Docker Compose:

```bash
docker-compose up
```

See [Docker Release Documentation](.github/DOCKER_RELEASE.md) for more details.

# Development

## Build & Déploiement (français)

Si vous voulez construire l'image Docker locale qui sert l'application web bundlée :

1. Générer le bundle web avec dx (pré-requis : avoir `dx` installé — par exemple via `cargo binstall dioxus-cli`):

```bash
# depuis la racine du dépôt
dx bundle --platform web --release --out-dir web
```

Après cette commande vous devez avoir un dossier `web/` contenant au minimum `public/` (assets, index.html) et un binaire `server` Linux.

2. Construire l'image Docker runtime (le `Dockerfile` à la racine attend un dossier `web/` dans le contexte de build) :

```bash
# depuis la racine du dépôt
docker build -t qr-craft:local .
```

3. Lancer le conteneur (exemple en mappant le port 8080) :

```bash
docker run --rm -e PORT=8080 -p 8080:8080 qr-craft:local
```

Notes :

- Si vous voulez construire et publier depuis CI, voyez le workflow GitHub Actions dans `.github/workflows/ci.yml`.
- Si `dx` génère seulement `server.exe` (cas Windows), l'image Linux ne pourra pas exécuter ce binaire : exécutez `dx` sur un runner Linux (ou dans WSL) pour produire un binaire Linux natif.

Your new bare-bones project includes minimal organization with a single `main.rs` file and a few assets.

```
project/
├─ assets/ # Any assets that are used by the app should be placed here
├─ src/
│  ├─ main.rs # main.rs is the entry point to your application and currently contains all components for the app
├─ Cargo.toml # The Cargo.toml file defines the dependencies and feature flags for your project
```

### Tailwind

1. Install npm: https://docs.npmjs.com/downloading-and-installing-node-js-and-npm
2. Install the Tailwind CSS CLI: https://tailwindcss.com/docs/installation
3. Run the following command in the root of the project to start the Tailwind CSS compiler:

```bash
npx tailwindcss -i ./tailwind.css -o ./assets/tailwind.css --watch
```

### Serving Your App

Run the following command in the root of your project to start developing with the default platform:

```bash
dx serve --platform web
```

To run for a different platform, use the `--platform platform` flag. E.g.

```bash
dx serve --platform desktop
```
