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

# Copy the bundled web assets and server binary. The build will succeed even if one
# of these files is missing; the start script will fail with a helpful message.
COPY web/public /app/public
COPY web/server /app/server
COPY web/server.exe /app/server.exe

RUN chmod +x /app/server /app/server.exe || true

# Expose the configured port (default 8080). Users can override PORT at runtime.
EXPOSE ${PORT}

# Copy helper start script and make executable
COPY start.sh /app/start.sh
RUN chmod +x /app/start.sh

ENTRYPOINT ["/app/start.sh"]
