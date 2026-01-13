# @affectively/wasm-cache-ops

High-performance WebAssembly caching operations written in Rust.

[![npm version](https://img.shields.io/npm/v/@affectively/wasm-cache-ops.svg)](https://www.npmjs.com/package/@affectively/wasm-cache-ops)
[![crates.io](https://img.shields.io/crates/v/affectively-cache-ops.svg)](https://crates.io/crates/affectively-cache-ops)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

## Features

- **Batch Operations** - Efficient bulk cache operations
- **Compression** - LZ4-style compression for cached data
- **Eviction Policies** - LRU, LFU, and TTL-based eviction
- **Fast Hashing** - High-performance hash functions

## Installation

```bash
npm install @affectively/wasm-cache-ops
```

## Quick Start

```typescript
import init, { batch_get, batch_set, compress, decompress } from '@affectively/wasm-cache-ops';

await init();

// Batch operations
const results = batch_get(keys);
batch_set(entries);

// Compression
const compressed = compress(data);
const original = decompress(compressed);
```

## License

MIT License - see [LICENSE](./LICENSE) for details.

---

Made with ️ by [AFFECTIVELY](https://affectively.ai)
