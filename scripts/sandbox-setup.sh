#!/usr/bin/env bash
# Mycelix Space - Holochain Sandbox Setup Script
# This script sets up a local Holochain sandbox for development

set -e

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_ROOT="$(dirname "$SCRIPT_DIR")"
SANDBOX_DIR="$PROJECT_ROOT/.sandbox"
HAPP_PATH="$PROJECT_ROOT/workdir/mycelix_space.happ"

echo "=== Mycelix Space Sandbox Setup ==="
echo ""

# Check for hc command
if ! command -v hc &> /dev/null; then
    echo "ERROR: 'hc' command not found."
    echo ""
    echo "Please enter the Nix development shell first:"
    echo "  cd $PROJECT_ROOT"
    echo "  nix develop"
    echo ""
    exit 1
fi

# Check for hApp file
if [ ! -f "$HAPP_PATH" ]; then
    echo "hApp not found at: $HAPP_PATH"
    echo "Building hApp first..."
    echo ""
    "$SCRIPT_DIR/build-happ.sh"
fi

# Create sandbox directory
mkdir -p "$SANDBOX_DIR"

echo "Creating Holochain sandbox..."
echo ""

# Generate sandbox (creates conductor config and data directory)
cd "$PROJECT_ROOT"
hc sandbox generate \
    --root "$SANDBOX_DIR" \
    --app-port 8888 \
    "$HAPP_PATH" \
    --run

echo ""
echo "=== Sandbox Setup Complete ==="
echo ""
echo "The conductor is now running on port 8888."
echo ""
echo "To run the UI:"
echo "  cd ui"
echo "  npm install"
echo "  npm run dev"
echo ""
echo "Then open http://localhost:5173 in your browser."
echo ""
echo "Press Ctrl+C to stop the conductor."
