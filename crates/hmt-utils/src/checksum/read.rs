// Copyright (c) The Hummanta Authors. All rights reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use anyhow::{Context, Result};
use std::path::Path;

use super::CHECKSUM_FILE_SUFFIX;

/// Reads checksum from a .sha256 file
///
/// # Arguments
///
/// * `path` - Path to the .sha256 file
///
/// # Returns
///
/// Returns the checksum string with leading and trailing whitespace removed
///
/// # Errors
///
/// Returns an error if:
/// - The file extension is not .sha256
/// - The file does not exist
/// - The file content is empty
pub fn read(path: &Path) -> Result<String> {
    // Check file extension
    if path.extension().and_then(|ext| ext.to_str()) != Some(CHECKSUM_FILE_SUFFIX) {
        return Err(anyhow::anyhow!(
            "Invalid file extension: expected .sha256, got {}",
            path.display()
        ));
    }

    // Read file content
    let content = std::fs::read_to_string(path)
        .context(format!("Failed to read SHA256 from file: {}", path.display()))?;

    // Check if file content is empty
    let content = content.trim();
    if content.is_empty() {
        return Err(anyhow::anyhow!("SHA256 file is empty: {}", path.display()));
    }

    Ok(content.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use std::io::Write;
    use tempfile::tempdir;

    #[test]
    fn test_read_valid_checksum() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.sha256");
        let mut file = File::create(&file_path).unwrap();
        writeln!(file, "abc123").unwrap();

        let result = read(&file_path).unwrap();
        assert_eq!(result, "abc123");
    }

    #[test]
    fn test_read_checksum_with_whitespace() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.sha256");
        let mut file = File::create(&file_path).unwrap();
        writeln!(file, "  abc123  ").unwrap();

        let result = read(&file_path).unwrap();
        assert_eq!(result, "abc123");
    }

    #[test]
    fn test_read_invalid_extension() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.txt");
        let mut file = File::create(&file_path).unwrap();
        writeln!(file, "abc123").unwrap();

        let result = read(&file_path);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Invalid file extension"));
    }

    #[test]
    fn test_read_empty_file() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.sha256");
        File::create(&file_path).unwrap();

        let result = read(&file_path);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("SHA256 file is empty"));
    }

    #[test]
    fn test_read_nonexistent_file() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("nonexistent.sha256");

        let result = read(&file_path);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Failed to read SHA256 from file"));
    }
}
