use std::env;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

/// Resolve and create (if needed) an examples output directory under target/.
///
/// - Base directory: uses `CARGO_TARGET_DIR` if set, otherwise `./target`.
/// - Final path: `<base>/examples/<example_name>`.
pub fn example_output_dir(example_name: &str) -> io::Result<PathBuf> {
    let base = env::var("CARGO_TARGET_DIR").unwrap_or_else(|_| "target".to_string());
    let dir = Path::new(&base).join("examples").join(example_name);
    fs::create_dir_all(&dir)?;
    Ok(dir)
}

/// Convenience: join a file name to the example output dir.
pub fn example_output_path(example_name: &str, file_name: &str) -> io::Result<PathBuf> {
    Ok(example_output_dir(example_name)?.join(file_name))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn builds_examples_dir_path() {
        let dir = example_output_dir("unit_test_example").unwrap();
        // Only check suffix so this test is robust to CARGO_TARGET_DIR
        assert!(dir.ends_with("examples/unit_test_example"));
    }
}
