#!/bin/sh
set -e

IP=${IP:-0.0.0.0}
PORT=${PORT:-8080}

cd /app

echo "Starting qr-craft runtime: IP=$IP PORT=$PORT"

if [ -x ./server ]; then
  echo "Found native server binary ./server — starting it"
  exec ./server
fi

if [ -x ./server.exe ]; then
  echo "Found Windows server binary ./server.exe — this image cannot run .exe natively"
  echo "Either rebuild the bundle targeting Linux or run a Windows container. Starting fallback static server instead."
fi

if [ -d ./public ]; then
  echo "Serving ./public via python http.server on ${PORT}"
  # Use python if available (debian slim includes it in many cases). If not present,
  # print instructions and exit.
  if command -v python3 >/dev/null 2>&1; then
    exec python3 -m http.server "${PORT}" --bind "${IP}" --directory ./public
  elif command -v python >/dev/null 2>&1; then
    exec python -m http.server "${PORT}" --bind "${IP}" --directory ./public
  else
    echo "python not found in image — cannot serve static assets. Exiting."
    exit 2
  fi
fi

echo "No server binary or public folder found in /app — nothing to run."
exit 1
