# @affectively/wasm-cache-ops

`@affectively/wasm-cache-ops` is a Rust/WebAssembly module for cache-oriented batch operations and related helpers.

The fair brag is that it keeps a cache-heavy workflow small and fast: batch reads and writes, compression, and helper operations all sit behind one WASM boundary.

## What It Helps You Do

- perform batch cache gets and sets
- compress and decompress cached payloads
- build cache-oriented logic around a smaller utility module

## Installation

```bash
npm install @affectively/wasm-cache-ops
```

## Quick Start

```ts
import init, {
  batch_get,
  batch_set,
  compress,
  decompress,
} from '@affectively/wasm-cache-ops';

await init();

const results = batch_get(keys);
batch_set(entries);

const compressed = compress(data);
const original = decompress(compressed);
```

## Why This README Is Grounded

Cache Ops does not need a bigger pitch than that. The strongest fair brag is that it already gives callers a focused WASM helper for a few high-value cache operations.
