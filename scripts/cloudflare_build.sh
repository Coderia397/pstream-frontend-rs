#!/usr/bin/env bash
# Build the site on a Linux builder (Cloudflare Pages). Installs a minimal Rust toolchain with the wasm target, Trunk and the
# Tailwind standalone binary, then runs a release build into dist/. Versions match the ones used on the dev machine.
#   Pages settings: build command `bash scripts/cloudflare_build.sh`, output directory `dist`.
#   Optional environment variable: PSTREAM_ENGINE_URL (base URL of the recommendation engine, no trailing slash).
set -euo pipefail

TRUNK_VERSION="${TRUNK_VERSION:-0.21.14}"
TAILWIND_VERSION="${TAILWIND_VERSION:-v4.0.9}"

if ! command -v cargo >/dev/null 2>&1; then
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --profile minimal --default-toolchain stable -t wasm32-unknown-unknown
fi
# shellcheck disable=SC1091
. "$HOME/.cargo/env"
rustup target add wasm32-unknown-unknown

mkdir -p .tools
if [ ! -x .tools/trunk ]; then
  curl -sSfL "https://github.com/trunk-rs/trunk/releases/download/v${TRUNK_VERSION}/trunk-x86_64-unknown-linux-gnu.tar.gz" | tar -xz -C .tools
fi
curl -sSfL -o tailwindcss "https://github.com/tailwindlabs/tailwindcss/releases/download/${TAILWIND_VERSION}/tailwindcss-linux-x64"
chmod +x tailwindcss

rustc --version
./.tools/trunk --version
./.tools/trunk build --release
ls -la dist | head -20
