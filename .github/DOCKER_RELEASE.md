# 🐳 Documentation des Releases Docker

## Vue d'ensemble

Ce document décrit le processus de publication automatique des images Docker pour **QR Craft** via GitHub Actions et GitHub Container Registry (GHCR).

## 🚀 Publication Automatique

### Workflow CI/CD

Le projet utilise GitHub Actions pour automatiser la construction et publication des images Docker :

- **Déclencheur** : Push sur `main`, pull requests, et tags de version
- **Plateformes** : `linux/amd64` et `linux/arm64`
- **Registry** : `ghcr.io/belugabox/qr-craft`

### Fichiers de Configuration

- **`.github/workflows/ci.yml`** : Workflow principal CI/CD
- **`Dockerfile`** : Définition de l'image de production
- **`docker-compose.yml`** : Configuration pour le développement

## 🏷️ Stratégie de Taggage

### Branches et Commits

| Événement         | Tag                           | Description             |
| ----------------- | ----------------------------- | ----------------------- |
| Push sur `main`   | `main`                        | Dernière version stable |
| Commit spécifique | `sha-<short-sha>`             | Version par commit      |
| Tag de version    | `v1.2.3`, `1.2.3`, `1.2`, `1` | Versions sémantiques    |

### Exemples de Tags

```bash
# Branche principale
ghcr.io/belugabox/qr-craft:main

# Version complète
ghcr.io/belugabox/qr-craft:v1.2.3

# Version mineure
ghcr.io/belugabox/qr-craft:1.2

# Version majeure
ghcr.io/belugabox/qr-craft:1

# Commit spécifique
ghcr.io/belugabox/qr-craft:sha-a1b2c3d
```

## 📦 Utilisation des Images

### Pull depuis GHCR

```bash
# Dernière version de main
docker pull ghcr.io/belugabox/qr-craft:main

# Version spécifique
docker pull ghcr.io/belugabox/qr-craft:v1.2.3

# Version mineure
docker pull ghcr.io/belugabox/qr-craft:1.2
```

### Exécution du Conteneur

```bash
# Port par défaut (8080)
docker run -p 8080:8080 ghcr.io/belugabox/qr-craft:main

# Port personnalisé
docker run -e PORT=3000 -p 3000:3000 ghcr.io/belugabox/qr-craft:main

# Mode détaché
docker run -d -p 8080:8080 --name qr-craft ghcr.io/belugabox/qr-craft:main
```

### Variables d'Environnement

| Variable   | Défaut | Description              |
| ---------- | ------ | ------------------------ |
| `PORT`     | `8080` | Port d'écoute du serveur |
| `RUST_LOG` | `info` | Niveau de logging        |

## 🛠️ Développement Local

### Build de l'Image

```bash
# 1. Générer le bundle web
dx bundle --platform web --release --out-dir web

# 2. Vérifier le contenu généré
ls -la web/
# Doit contenir : public/ server (binaire Linux)

# 3. Construire l'image Docker
docker build -t qr-craft:local .

# 4. Tester localement
docker run -p 8080:8080 qr-craft:local
```

### Docker Compose

```yaml
version: "3.8"
services:
  qr-craft:
    image: ghcr.io/belugabox/qr-craft:main
    ports:
      - "8080:8080"
    environment:
      - PORT=8080
      - RUST_LOG=info
    restart: unless-stopped
    healthcheck:
      test: ["CMD", "curl", "-f", "http://localhost:8080"]
      interval: 30s
      timeout: 10s
      retries: 3
```

## 🔄 Processus de Release

### Création d'une Release

1. **Préparation**

   ```bash
   # Mettre à jour la version dans Cargo.toml si nécessaire
   # Commiter les changements
   git add .
   git commit -m "Release v1.2.3"
   ```

2. **Créer et pousser le tag**

   ```bash
   # Créer le tag annoté
   git tag -a v1.2.3 -m "Release version 1.2.3"

   # Pousser le tag
   git push origin v1.2.3
   ```

3. **Vérification**
   - Le workflow GitHub Actions se déclenche automatiquement
   - Attendre la completion du build (~10-15 minutes)
   - Vérifier que les images sont publiées sur GHCR

### Rollback

En cas de problème avec une release :

```bash
# Supprimer le tag local et distant
git tag -d v1.2.3
git push origin :refs/tags/v1.2.3

# Les images Docker restent sur GHCR mais ne sont plus référencées
```

## 🏗️ Architecture Docker

### Dockerfile Multi-Stage

```dockerfile
# Stage 1: Build de l'application Rust
FROM rust:1.70-slim as builder
WORKDIR /app
COPY . .
RUN dx bundle --platform web --release --out-dir web

# Stage 2: Image de production
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*
COPY --from=builder /app/web/ /app/
EXPOSE 8080
CMD ["./server"]
```

### Optimisations

- **Multi-platform** : Support amd64 et arm64
- **Minimal image** : Utilisation de Debian slim
- **Security** : Utilisateur non-root, certificats à jour
- **Performance** : Binaire compilé en release

## 🔍 Dépannage

### Problèmes Courants

#### Build qui échoue sur Windows

```bash
# Le binaire généré est un .exe Windows
# Solution : Utiliser WSL ou un runner Linux
dx bundle --platform web --release --out-dir web
```

#### Image trop volumineuse

```bash
# Vérifier la taille des layers
docker history ghcr.io/belugabox/qr-craft:main

# Optimiser le Dockerfile
# - Utiliser des images de base plus petites
# - Nettoyer le cache apt
# - Supprimer les fichiers temporaires
```

#### Problème de permissions

```bash
# Vérifier les permissions du binaire
docker run --rm ghcr.io/belugabox/qr-craft:main ls -la ./server

# Le binaire doit avoir les droits d'exécution
RUN chmod +x ./server
```

### Logs et Debugging

```bash
# Logs du conteneur
docker logs <container-id>

# Accès au shell du conteneur
docker run -it --entrypoint /bin/bash ghcr.io/belugabox/qr-craft:main

# Variables d'environnement détaillées
docker run -e RUST_LOG=debug -p 8080:8080 ghcr.io/belugabox/qr-craft:main
```

## 📊 Métriques et Monitoring

### Health Check

L'image inclut un health check basique :

```dockerfile
HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
  CMD curl -f http://localhost:${PORT:-8080}/ || exit 1
```

### Métriques Disponibles

- **Taille de l'image** : ~50MB (slim Debian + binaire Rust)
- **Temps de build** : ~10-15 minutes sur GitHub Actions
- **Temps de démarrage** : < 2 secondes
- **Utilisation mémoire** : ~20MB au repos

## 🔒 Sécurité

### Bonnes Pratiques

- **Images signées** : Utilisation de GHCR avec authentification
- **Scans de sécurité** : Intégration avec GitHub Security
- **Mises à jour** : Images de base régulièrement mises à jour
- **Permissions minimales** : Conteneur s'exécute avec utilisateur restreint

### Mises à Jour de Sécurité

```bash
# Reconstruire régulièrement
# Le workflow se déclenche sur schedule hebdomadaire
# Mise à jour automatique des dépendances Rust
```

## 📚 Ressources Supplémentaires

- [Documentation Dioxus](https://dioxuslabs.com/)
- [Guide Docker](https://docs.docker.com/)
- [GitHub Container Registry](https://docs.github.com/en/packages/working-with-a-github-packages-registry/working-with-the-container-registry)
- [GitHub Actions](https://docs.github.com/en/actions)

## 🤝 Contribution

Pour contribuer aux améliorations Docker :

1. Tester les changements localement
2. Mettre à jour cette documentation
3. Ouvrir une PR avec description détaillée
4. Attendre l'approbation et le merge

---

_Dernière mise à jour : Octobre 2025_
