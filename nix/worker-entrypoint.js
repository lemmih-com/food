// Cloudflare Worker entrypoint that lazy-loads the wasm-bindgen output.
// Import the compiled wasm module directly so we can initialize without
// relying on URL resolution within the Workers runtime.

import init, { fetch as wasmFetch } from './index.js';
import wasmModule from './index_bg.wasm';

let initPromise;

async function ensureInitialized() {
  if (!initPromise) {
    initPromise = init(wasmModule);
  }
  return initPromise;
}

export default {
  async fetch(request, env, ctx) {
    await ensureInitialized();
    return wasmFetch(request, env, ctx);
  },
};

