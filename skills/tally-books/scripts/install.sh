#!/usr/bin/env bash
# Install tally-books skill for Claude Code / Claude Desktop (filesystem skills)
# and build the CLI against the local tally-sdk-rs checkout.
set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
SKILL_DIR="$(cd "${SCRIPT_DIR}/.." && pwd)"
SDK_ROOT="$(cd "${SKILL_DIR}/../.." && pwd)"
SKILL_NAME="tally-books"
CLAUDE_SKILLS_DIR="${CLAUDE_SKILLS_DIR:-${HOME}/.claude/skills}"
TARGET_LINK="${CLAUDE_SKILLS_DIR}/${SKILL_NAME}"

if [[ ! -f "${SDK_ROOT}/Cargo.toml" ]]; then
  echo "error: could not find tally-sdk-rs Cargo.toml at ${SDK_ROOT}" >&2
  exit 1
fi

if ! grep -q '^name = "tally-sdk-rust"' "${SDK_ROOT}/Cargo.toml"; then
  echo "error: ${SDK_ROOT}/Cargo.toml is not tally-sdk-rust" >&2
  exit 1
fi

if ! command -v cargo >/dev/null 2>&1; then
  echo "error: cargo is required. Install Rust from https://rustup.rs" >&2
  exit 1
fi

# Persist SDK root for agents / later shells.
echo "${SDK_ROOT}" > "${SKILL_DIR}/.sdk-root"
export TALLY_SDK_ROOT="${SDK_ROOT}"

echo "Building skill CLI (release) against local SDK at ${SDK_ROOT}..."
(
  cd "${SCRIPT_DIR}"
  cargo build --release
)

mkdir -p "${CLAUDE_SKILLS_DIR}"
if [[ -e "${TARGET_LINK}" || -L "${TARGET_LINK}" ]]; then
  if [[ -L "${TARGET_LINK}" ]]; then
    rm "${TARGET_LINK}"
  else
    echo "error: ${TARGET_LINK} exists and is not a symlink. Move it aside and re-run." >&2
    exit 1
  fi
fi
ln -s "${SKILL_DIR}" "${TARGET_LINK}"

BIN_DIR="${SCRIPT_DIR}/target/release"
cat <<EOF

Installed skill: ${TARGET_LINK}
  -> ${SKILL_DIR}

SDK root: ${SDK_ROOT}
Binaries:
  ${BIN_DIR}/tally-ping
  ${BIN_DIR}/tally-ledgers
  ${BIN_DIR}/tally-create-voucher

Quick check (Tally must be running on :9000):
  ${BIN_DIR}/tally-ping

Claude Desktop (ZIP upload alternative):
  cd "${SKILL_DIR}/.." && zip -r tally-books.zip tally-books \\
    -x 'tally-books/scripts/target/*' -x 'tally-books/.sdk-root'
  Then add the zip under Customize → Skills.
  Prefer the symlink install above when the app reads ~/.claude/skills.

EOF
