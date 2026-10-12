//! ol (OverlayFS & Container Mount Manager) CLI
use anyhow::Result;

fn main() -> Result<()> {
    println!("ol v0.1.0 - Unprivileged container mount manager, OverlayFS layers, and sandbox isolation boundaries.");
    ol::init()?;
    println!("Engine initialized successfully.");
    Ok(())
}
