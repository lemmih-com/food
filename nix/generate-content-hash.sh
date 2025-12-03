#!/usr/bin/env bash
set -euo pipefail

# Generate a hash of client, server (worker), and assets to detect changes
# This hash will be used to determine if a preview deployment is needed

OUTPUT_FILE="${1:-content-hash.txt}"

# Find all relevant source files and generate a combined hash
{
  # Hash Rust source files (client, worker, app)
  find crates/app/src crates/client/src crates/worker/src -type f -name "*.rs" 2>/dev/null | sort | xargs -r cat

  # Hash Cargo.toml files
  find crates -type f -name "Cargo.toml" 2>/dev/null | sort | xargs -r cat
  cat Cargo.toml Cargo.lock

  # Hash CSS configuration
  cat <<'EOF'
@tailwind base;
@tailwind components;
@tailwind utilities;
EOF

  # Hash worker entrypoint
  cat nix/worker-entrypoint.js

} | sha256sum | awk '{print $1}' > "$OUTPUT_FILE"

echo "Content hash generated: $(cat "$OUTPUT_FILE")"
