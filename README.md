# food.lemmih.com

Minimal Leptos hello-world rendered on Cloudflare Workers with SSR enabled.

## Development

```bash
cd food-lemmih-com
wrangler dev
```

The `wrangler dev` command uses Nix to build the worker via `nix build .#website`.

## End-to-end test

With `wrangler dev` running you can either:

- Run via Nix (from the repository root, note the quotes to satisfy zsh):

  ```bash
  nix run '.#e2e-tests' -- --help   # passes through extra args to cargo run
  ```

- Or run via Cargo directly:

```bash
cd food-lemmih-com/e2e-tests
cargo run
```

Set `FOOD_APP_BASE_URL` if the worker is hosted on a different origin.

