use anyhow::{Context, Result};
use std::path::Path;

pub fn write_text(path: impl AsRef<Path>, text: impl AsRef<str>) -> Result<()> {
    let path = path.as_ref();
    std::fs::write(path, text.as_ref()).with_context(|| format!("writing {}", path.display()))?;
    Ok(())
}
