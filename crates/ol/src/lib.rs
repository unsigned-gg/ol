//! ol (OverlayFS & Container Mount Manager)
//!
//! Unprivileged container mount manager, OverlayFS layers, and sandbox isolation boundaries.
//!
//! # Architecture
//! Anchored in the systems principles of **Bryan Cantrill**.


pub use ol_domain::*;

/// Initializes the `ol` systems engine.
pub fn init() -> anyhow::Result<()> {
    Ok(())
}
