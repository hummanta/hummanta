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

use std::{fs::File, path::Path};

use anyhow::{Context, Result};
use flate2::{write::GzEncoder, Compression};
use tar::Builder;

/// Archive the given input directory, and save it to output path
pub async fn archive(input_path: &Path, output_path: &Path) -> Result<()> {
    let file = File::create(output_path)
        .context(format!("Failed to create archive: {:?}", output_path))?;
    let encoder = GzEncoder::new(file, Compression::default());

    let mut tar = Builder::new(encoder);
    tar.append_dir_all("", input_path).context("Failed to add directory to archive")?;
    tar.finish().context("Failed to finish tar creation")?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::{
        fs::{self, File},
        io::{Read, Write},
    };
    use tempfile::tempdir;

    use super::*;

    #[tokio::test]
    async fn test_archive_success() {
        let temp_dir = tempdir().unwrap();
        let input_dir = temp_dir.path().join("input");
        let output_file = temp_dir.path().join("archive.tar.gz");

        // Create input directory and files
        fs::create_dir(&input_dir).unwrap();
        let file_path = input_dir.join("test_file.txt");
        let mut file = File::create(&file_path).unwrap();
        writeln!(file, "Hello, world!").unwrap();

        // Call the archive function
        let result = archive(&input_dir, &output_file).await;

        // Assert success
        assert!(result.is_ok());
        assert!(output_file.exists());
    }

    #[tokio::test]
    async fn test_archive_empty_directory() {
        let temp_dir = tempdir().unwrap();
        let input_dir = temp_dir.path().join("empty_input");
        let output_file = temp_dir.path().join("empty_archive.tar.gz");

        // Create an empty input directory
        fs::create_dir(&input_dir).unwrap();

        // Call the archive function
        let result = archive(&input_dir, &output_file).await;

        // Assert success
        assert!(result.is_ok());
        assert!(output_file.exists());
    }

    #[tokio::test]
    async fn test_archive_nonexistent_input() {
        let temp_dir = tempdir().unwrap();
        let input_dir = temp_dir.path().join("nonexistent_input");
        let output_file = temp_dir.path().join("nonexistent_archive.tar.gz");

        // Call the archive function with a nonexistent input directory
        let result = archive(&input_dir, &output_file).await;

        // Assert failure
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_archive_output_path_unwritable() {
        let temp_dir = tempdir().unwrap();
        let input_dir = temp_dir.path().join("input");
        let output_file = temp_dir.path().join("unwritable_dir").join("archive.tar.gz");

        // Create input directory and files
        fs::create_dir(&input_dir).unwrap();
        let file_path = input_dir.join("test_file.txt");
        let mut file = File::create(&file_path).unwrap();
        writeln!(file, "Hello, world!").unwrap();

        // Call the archive function with an unwritable output path
        let result = archive(&input_dir, &output_file).await;

        // Assert failure
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_archive_validate_content() {
        let temp_dir = tempdir().unwrap();
        let input_dir = temp_dir.path().join("input");
        let output_file = temp_dir.path().join("archive.tar.gz");

        // Create input directory and files
        fs::create_dir(&input_dir).unwrap();
        let file_path = input_dir.join("test_file.txt");
        let mut file = File::create(&file_path).unwrap();
        writeln!(file, "Hello, world!").unwrap();

        // Call the archive function
        let result = archive(&input_dir, &output_file).await;

        // Assert success
        assert!(result.is_ok());
        assert!(output_file.exists());

        // Validate the content of the archive
        let file = File::open(&output_file).unwrap();
        let decoder = flate2::read::GzDecoder::new(file);
        let mut archive = tar::Archive::new(decoder);

        let entries = archive.entries().unwrap();
        let mut found_file = false;

        for entry in entries {
            let mut entry = entry.unwrap();
            if entry.path().unwrap() == Path::new("test_file.txt") {
                found_file = true;

                // Validate file content
                let mut content = String::new();
                entry.read_to_string(&mut content).unwrap();
                assert_eq!(content, "Hello, world!\n");
            }
        }

        assert!(found_file, "Expected file 'test_file.txt' not found in archive");
    }
}
