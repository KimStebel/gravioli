#!/bin/bash
set -euo pipefail

# Build the web version
./build-web.sh

echo "Deploying to gh-pages..."

# Create a temporary directory for the gh-pages content
TMPDIR=$(mktemp -d)
trap 'rm -rf "$TMPDIR"' EXIT

cp -r build/web/* "$TMPDIR/"

# Set up the gh-pages branch in the temp dir
cd "$TMPDIR"
git init
git checkout -b gh-pages
git add .
git commit -m "Deploy web build"

# Push to the remote gh-pages branch
REMOTE=$(git -C /home/k/garlic remote get-url origin)
git push --force "$REMOTE" gh-pages

echo "Done! Site will be available at GitHub Pages once enabled in repo settings."
