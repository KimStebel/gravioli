#!/bin/bash
set -e

IMAGE_NAME="garlic-claude"
CONTAINER_NAME="garlic-claude"
PROJECT_DIR="$(cd "$(dirname "$0")" && pwd)"

docker build -t "$IMAGE_NAME" "$PROJECT_DIR"

exec docker run --rm -it \
  --name "$CONTAINER_NAME" \
  --env-file "$PROJECT_DIR/.env" \
  -e ANTHROPIC_API_KEY \
  -v "$PROJECT_DIR":/home/node/project \
  -v "$HOME/.claude":/home/node/.claude \
  -v "$HOME/.claude.json":/home/node/.claude.json \
  -w /home/node/project \
  "$IMAGE_NAME" \
  claude --dangerously-skip-permissions
