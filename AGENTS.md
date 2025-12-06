# Agent Guidelines

See: https://raw.githubusercontent.com/lemmih/best-practice/refs/heads/main/AGENTS.md

## Before Pushing Changes

Before pushing changes to this repository, you must run the following commands:

1. Run the flake checks:
   ```bash
   nix flake check
   ```

2. Run the end-to-end tests:
   ```bash
   nix run .#e2e-tests
   ```

These steps ensure code quality and functionality before changes are committed.
