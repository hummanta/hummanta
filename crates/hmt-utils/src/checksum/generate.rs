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

use std::path::Path;

use anyhow::{Context, Result};
use sha2::{Digest, Sha256};
use tokio::{
    fs,
    io::{AsyncReadExt, AsyncWriteExt, BufReader},
};

/// Generate SHA256 checksum of a file and write it to an output file
pub async fn generate(file: &Path, output_path: &Path) -> Result<()> {
    // Open the file for reading
    let mut hasher = Sha256::new();
    let file = fs::File::open(file)
        .await
        .context(format!("Failed to open file for checksum: {file:?}"))?;
    let mut reader = BufReader::new(file);
    let mut buffer = [0; 4096];

    // Read the file in chunks and update the hash
    while let Ok(bytes_read) = reader.read(&mut buffer).await {
        if bytes_read == 0 {
            break;
        }
        hasher.update(&buffer[..bytes_read]);
    }

    // Finalize the hash
    let checksum = format!("{:x}", hasher.finalize());

    // Create the checksum file
    let mut checksum_file = fs::File::create(output_path)
        .await
        .context(format!("Failed to create checksum file: {output_path:?}"))?;

    // Write the checksum to the file
    checksum_file.write_all(checksum.as_bytes()).await.context("Failed to write checksum")?;
    checksum_file.flush().await.context("Failed to flush checksum")?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::{fs, io::Write};
    use tempfile::tempdir;

    use super::*;

    #[tokio::test]
    async fn test_checksum_file() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test_file.txt");
        let output_path = dir.path().join("checksum.txt");

        // Create a test file
        let mut file = fs::File::create(&file_path).unwrap();
        file.write_all(b"Hello, world!").unwrap();

        // Generate checksum
        generate(&file_path, &output_path).await.unwrap();

        // Verify checksum file exists
        assert!(output_path.exists());

        // Read checksum from file
        let checksum_content = fs::read_to_string(output_path).unwrap();

        // Calculate the SHA256 checksum of the file
        let mut hasher = Sha256::new();
        let file_content = fs::read(&file_path).unwrap(); // Read the file content
        hasher.update(&file_content);
        let expected_checksum = format!("{:x}", hasher.finalize());

        // Verify checksum matches the calculated checksum
        assert_eq!(checksum_content, expected_checksum);
    }

    #[tokio::test]
    async fn test_checksum_nonexistent_file() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("nonexistent_file.txt");
        let output_path = dir.path().join("checksum.txt");

        // Attempt to generate checksum for a nonexistent file
        let result = generate(&file_path, &output_path).await;

        // Verify error is returned
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_checksum_output_file_unwritable() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test_file.txt");
        let output_path = dir.path().join("checksum.txt");

        // Create a test file
        let mut file = fs::File::create(&file_path).unwrap();
        writeln!(file, "Hello, world!").unwrap();

        // Create an unwritable output file
        let output_file = fs::File::create(&output_path).unwrap();
        let metadata = output_file.metadata().unwrap();
        let mut permissions = metadata.permissions();
        permissions.set_readonly(true);
        fs::set_permissions(&output_path, permissions).unwrap();

        // Attempt to generate checksum
        let result = generate(&file_path, &output_path).await;

        // Verify error is returned
        assert!(result.is_err());
    }
}
