FROM debian:bookworm-slim

# Small runtime image that expects a bundled `web` folder in the docker build context.
# Use `dx bundle --platform web` to produce a `web` folder (containing `public/` and
# the server binary) and then build this image from the repo root:
#
#   docker build -f Dockerfile.runtime -t qr-craft:latest .
#
# The build context should include a `web` directory. Example:
#   target/dx/qr-craft/release/web -> ./web
#
ENV IP=0.0.0.0
ENV PORT=8080

WORKDIR /app

# Copy the bundled web assets and any server binaries that exist. On Linux CI the
# bundle will typically contain `web/server` (no .exe). Trying to `COPY` a missing
# file causes buildx to fail during cache-key computation, so we copy the whole
# `web` dir and then move/adjust files conditionally.
COPY web /app/web

# If present, move server binaries into /app and make them executable. The
# conditional commands are safe if a file is missing.
RUN set -eu; \
    if [ -f /app/web/server ]; then mv /app/web/server /app/server; fi; \
    if [ -f /app/server ]; then chmod +x /app/server || true; fi; \
    if [ -d /app/web/public ]; then rm -rf /app/public && mv /app/web/public /app/public; fi

# Expose the configured port (default 8080). Users can override PORT at runtime.
EXPOSE ${PORT}

# Copy helper start script and make executable
COPY start.sh /app/start.sh
RUN chmod +x /app/start.sh

ENTRYPOINT ["/app/start.sh"]
