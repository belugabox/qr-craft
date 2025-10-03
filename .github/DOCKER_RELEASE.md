# Docker Release Instructions

## Automatic Docker Release

The CI automatically builds and publishes Docker images to GitHub Container Registry (ghcr.io).

## Available Images

Images are published at: `ghcr.io/belugabox/qr-craft`

### Tagging Strategy

- **Main branch**: `ghcr.io/belugabox/qr-craft:main`
- **Commit SHA**: `ghcr.io/belugabox/qr-craft:sha-<commit-hash>`
- **Version tags** (e.g., v1.0.0):
  - `ghcr.io/belugabox/qr-craft:1.0.0`
  - `ghcr.io/belugabox/qr-craft:1.0`
  - `ghcr.io/belugabox/qr-craft:1`

## Creating a New Release

To create a new Docker release:

1. Update the version in `Cargo.toml` if needed
2. Commit your changes
3. Create and push a tag:
   ```bash
   git tag v1.0.0
   git push origin v1.0.0
   ```
4. The CI will automatically build and push the Docker image with appropriate tags

## Using the Docker Image

### Pull from GitHub Container Registry

```bash
# Pull latest from main
docker pull ghcr.io/belugabox/qr-craft:main

# Pull specific version
docker pull ghcr.io/belugabox/qr-craft:1.0.0
```

### Run the Container

```bash
docker run -p 8080:8080 ghcr.io/belugabox/qr-craft:main
```

### Using Docker Compose

Update `docker-compose.yml`:

```yaml
version: '3.8'
services:
  qr-craft:
    image: ghcr.io/belugabox/qr-craft:main
    ports:
      - "8080:8080"
    restart: unless-stopped
```

## Multi-Platform Support

Images are built for:
- linux/amd64
- linux/arm64

Docker will automatically pull the correct image for your platform.
