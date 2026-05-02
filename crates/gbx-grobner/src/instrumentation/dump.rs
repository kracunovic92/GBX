//! File dumping helpers for optional F4 debug output.
//!
//! Dumps are intended for offline inspection after a run. They should be used
//! carefully because dumping full F4 structures can be very expensive.

use std::io;
use std::path::Path;

#[cfg(feature = "profile-dump")]
use std::{fs, path::PathBuf};

/// Dump a serializable value as pretty JSON.
///
/// Enabled only with the `profile-dump` feature. Without that feature this is
/// a no-op.
#[cfg(feature = "profile-dump")]
pub fn dump_json<T>(dir: impl AsRef<Path>, name: impl AsRef<str>, value: &T) -> io::Result<()>
where
    T: serde::Serialize,
{
    fs::create_dir_all(dir.as_ref())?;

    let mut path = PathBuf::from(dir.as_ref());
    path.push(format!("{}.json", sanitize_file_name(name.as_ref())));

    let file = fs::File::create(path)?;
    serde_json::to_writer_pretty(file, value)?;

    Ok(())
}

/// No-op JSON dump when `profile-dump` is disabled.
#[cfg(not(feature = "profile-dump"))]
#[inline]
pub fn dump_json<T>(_dir: impl AsRef<Path>, _name: impl AsRef<str>, _value: &T) -> io::Result<()> {
    Ok(())
}

/// Dump plain text.
///
/// Useful for readable polynomial/basis/matrix dumps.
#[cfg(feature = "profile-dump")]
pub fn dump_text(dir: impl AsRef<Path>, name: impl AsRef<str>, content: &str) -> io::Result<()> {
    fs::create_dir_all(dir.as_ref())?;

    let mut path = PathBuf::from(dir.as_ref());
    path.push(format!("{}.txt", sanitize_file_name(name.as_ref())));

    fs::write(path, content)
}

/// No-op text dump when `profile-dump` is disabled.
#[cfg(not(feature = "profile-dump"))]
#[inline]
pub fn dump_text(_dir: impl AsRef<Path>, _name: impl AsRef<str>, _content: &str) -> io::Result<()> {
    Ok(())
}

#[cfg(feature = "profile-dump")]
fn sanitize_file_name(name: &str) -> String {
    name.chars()
        .map(|ch| match ch {
            'a'..='z' | 'A'..='Z' | '0'..='9' | '_' | '-' | '.' => ch,
            _ => '_',
        })
        .collect()
}
