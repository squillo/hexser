#!/bin/bash
# Regenerate architecture diagram for RealWorld API
#
# This script provides a quick pipeline for continuously building the
# visual representation of the architecture as alterations happen.
#
# Usage: ./regenerate_diagram.sh
#
# Revision History
# - 2025-10-10T11:12:00Z @AI: Initial diagram regeneration pipeline script.

set -e

echo "🔧 RealWorld API - Architecture Diagram Pipeline"
echo "================================================"
echo ""

echo "📦 Building the diagram generator..."
cargo build --bin generate_architecture_diagram --quiet

echo "✨ Generating architecture diagram..."
cargo run --bin generate_architecture_diagram --quiet

echo ""
echo "✅ Architecture diagram updated in: architecture_diagram.mmd"
echo ""
echo "💡 To view the diagram:"
echo "   - Open README.md in GitHub (renders Mermaid automatically)"
echo "   - Use a Mermaid preview tool (VS Code extension, mermaid.live, etc.)"
echo ""
echo "🎉 Diagram pipeline complete!"
