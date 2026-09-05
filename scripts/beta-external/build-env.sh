#!/usr/bin/env bash
# Build the fresh external beta environment image (BETA-016-A) and probe
# its manifest. Fails closed: any probe failure fails the build script.
#
# Usage: scripts/beta-external/build-env.sh [transcript-file]
# The transcript (default: stdout) records every command and output so
# the environment is externally reproducible.
set -euo pipefail

cd "$(dirname "$0")"
IMAGE="velqu-beta-external:0.1.0-beta.1"
TRANSCRIPT="${1:-/dev/stdout}"

{
  echo "== BETA-016-A external environment build =="
  echo "== host: $(uname -srm), docker $(docker --version | cut -d' ' -f3 | tr -d ,)"
  echo "== \$ git rev-parse HEAD"
  git rev-parse HEAD
  echo "== \$ docker build -t ${IMAGE} ."
  docker build -t "${IMAGE}" .
  echo "== image id/digest =="
  docker images --digests --format '{{.Repository}}:{{.Tag}} {{.Digest}} ({{.ID}})' "${IMAGE%%:*}"
  echo "== \$ docker run --rm ${IMAGE} (manifest probe) =="
  docker run --rm "${IMAGE}"
  echo "ENV-BUILD-OK"
} 2>&1 | tee "${TRANSCRIPT}"

# tee swallows the pipeline status; re-check the probe explicitly.
docker run --rm "${IMAGE}" >/dev/null
echo "VERIFY-PROBE-OK"
