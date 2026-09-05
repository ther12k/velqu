#!/usr/bin/env bash
# Environment manifest probe for the fresh external beta container
# (BETA-016-A). Prints one fact per line; exits non-zero if any
# prerequisite documented in docs/beta/QUICKSTART.md is missing.
set -euo pipefail

fail() { echo "MANIFEST-FAIL: $*" >&2; exit 1; }

[ "$(uname -m)" = "x86_64" ] || fail "unsupported architecture $(uname -m)"
. /etc/os-release
[ "$(id -un)" = "beta" ] || fail "expected unprivileged user 'beta', got '$(id -un)'"

echo "os=${PRETTY_NAME}"
echo "arch=$(uname -m)"
echo "kernel=$(uname -r)"
echo "user=$(id -un)"
command -v bun >/dev/null || fail "bun missing"
echo "bun=$(bun --version)"
command -v cargo >/dev/null || fail "cargo missing"
command -v rustc >/dev/null || fail "rustc missing"
echo "cargo=$(cargo --version)"
echo "rustc=$(rustc --version)"
echo "gcc=$(gcc --version | head -1)"
echo "git=$(git --version)"
echo "bun_install_dir=${BUN_INSTALL:-unset}"
echo "rustup_toolchain=$(rustup show active-toolchain 2>/dev/null || echo unset)"

# Freshness: no Velqu material may exist in the fresh environment.
if [ -d /velqu ] || find /home/beta -maxdepth 2 -iname '*velqu*' | grep -q .; then
  fail "fresh environment contains Velqu material"
fi
echo "fresh=no-velqu-material"
echo "MANIFEST-OK"
