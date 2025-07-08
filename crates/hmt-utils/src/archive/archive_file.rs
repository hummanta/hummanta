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

use std::{fs, path::Path};

use anyhow::{Context, Result};
use flate2::{write::GzEncoder, Compression};
use tar::Builder;

/// Archive a single file into tar.gz
pub async fn archive_file(src: &Path, dest: &Path) -> Result<()> {
    if !src.exists() {
        anyhow::bail!("Source file does not exist: {:?}", src);
    }
    if !src.is_file() {
        anyhow::bail!("Source path is not a file: {:?}", src);
    }

    if let Some(parent) = dest.parent() {
        fs::create_dir_all(parent)
            .context("Failed to create parent directories for destination")?;
    }

    let file = fs::File::create(dest).context(format!("Failed to create archive: {dest:?}"))?;
    let encoder = GzEncoder::new(file, Compression::default());
    let mut tar = Builder::new(encoder);

    let file_name = src
        .file_name()
        .and_then(|n| n.to_str())
        .ok_or_else(|| anyhow::anyhow!("Invalid UTF-8 in source file name"))?;

    tar.append_path_with_name(src, file_name).context("Failed to add file to tar")?;
    tar.finish().context("Failed to finish tar creation")?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::{fs, io::Write};

    use flate2::read::GzDecoder;
    use tar::Archive;
    use tempfile::tempdir;

    use super::*;

    #[tokio::test]
    async fn test_archive_success() {
        let temp_dir = tempdir().unwrap();
        let src_file_path = temp_dir.path().join("test_file.txt");
        let dest_file_path = temp_dir.path().join("archive.tar.gz");

        // Create a test file
        let mut file = fs::File::create(&src_file_path).unwrap();
        writeln!(file, "This is a test file").unwrap();

        // Call the archive function
        let result = archive_file(&src_file_path, &dest_file_path).await;

        // Assert success
        assert!(result.is_ok());
        assert!(dest_file_path.exists());
    }

    #[tokio::test]
    async fn test_archive_missing_source() {
        let temp_dir = tempdir().unwrap();
        let src_file_path = temp_dir.path().join("non_existent_file.txt");
        let dest_file_path = temp_dir.path().join("archive.tar.gz");

        // Call the archive function with a non-existent source file
        let result = archive_file(&src_file_path, &dest_file_path).await;

        // Assert failure
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_unarchive_success() {
        let temp_dir = tempdir().unwrap();
        let src_file_path = temp_dir.path().join("test_file.txt");
        let archive_file_path = temp_dir.path().join("archive.tar.gz");
        let extract_dir = temp_dir.path().join("extracted");

        // Create a test file
        let mut file = fs::File::create(&src_file_path).unwrap();
        writeln!(file, "This is a test file").unwrap();

        // Create an archive
        archive_file(&src_file_path, &archive_file_path).await.unwrap();

        // Extract the archive
        fs::create_dir(&extract_dir).unwrap();
        let archive_file = fs::File::open(&archive_file_path).unwrap();
        let decoder = GzDecoder::new(archive_file);
        let mut archive = Archive::new(decoder);
        archive.unpack(&extract_dir).unwrap();

        // Verify the extracted file
        let extracted_file_path = extract_dir.join("test_file.txt");
        assert!(extracted_file_path.exists());
        let content = fs::read_to_string(extracted_file_path).unwrap();
        assert_eq!(content, "This is a test file\n");
    }
}
