# ol

> ol (OverlayFS & Container Mount Manager)  
> Unprivileged container mount manager, OverlayFS layers, and sandbox isolation boundaries.  
> Dual-licensed under `Apache-2.0 OR MIT`.

## Philosophical Foundations
- **Champion Inspiration:** Bryan Cantrill
- **Zero Days Open Source Initiative (ZOSI):** Machine sympathy, low-level efficiency, and verified contracts.
- **Architecture:** Zero-I/O domain model in `ol-domain` separated from systems runtime in `ol`.

## Crates
- `crates/ol`: Systems implementation and runtime interfaces.
- `crates/ol-domain`: Pure zero-I/O domain models, validation, and algebraic state machines.

## Building
All builds must specify `-j 2` to prevent memory exhaustion on shared multi-agent hosts:
```bash
cargo build -j 2
cargo test -j 2
```
