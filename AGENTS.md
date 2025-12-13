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

## Tech Stack

**Languages:** Rust 1.91.0 (Edition 2021), WebAssembly, JavaScript, SQL, Nix

**Frameworks:** Leptos 0.8.x (full-stack SSR+CSR), Axum 0.8.7 (HTTP), Tower, Tailwind CSS 4

**Infrastructure:** Cloudflare Workers (compute), D1 (SQLite DB), KV (auth), R2 (Nix cache)

**Build:** Nix Flakes (nixpkgs-25.05), Crane, Wrangler, wasm-bindgen 0.2.106, wasm-opt, esbuild

**CI/CD:** GitHub Actions—`nix flake check`, e2e tests, Workers deploy, PR previews with cloned D1

**Testing:** E2E via thirtyfour (WebDriver) + GeckoDriver + local Wrangler

**Linting:** rustfmt, clippy, leptosfmt, alejandra

**Workspace:**
- `crates/app` — Shared Leptos app (SSR+CSR)
- `crates/client` — Client hydration entry
- `crates/worker` — Cloudflare Worker (Axum+Leptos SSR)
- `crates/screenshot` — Screenshot utility
- `e2e-tests` — End-to-end test suite
