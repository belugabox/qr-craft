# QR Craft

A modern QR code generator web application built with Rust and Dioxus.

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
