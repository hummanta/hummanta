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

use std::{io::Cursor, path::Path};

use anyhow::{Context, Result};
use flate2::read::GzDecoder;
use tar::Archive;

/// Unpack a `.tar.gz` archive from memory buffer into the target directory
pub fn unpack(data: &[u8], target_dir: &Path) -> Result<()> {
    let buffer = Cursor::new(data);
    let decoder = GzDecoder::new(buffer);
    let mut archive = Archive::new(decoder);

    archive.unpack(target_dir).context("Failed to unpack archive")?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::archive::archive_file;

    use super::*;
    use std::{fs, io::Write};
    use tempfile::tempdir;

    #[tokio::test]
    async fn test_unpack_archive() -> Result<()> {
        // Create a single temporary directory for all files
        let temp_dir = tempdir()?;

        // Create a temporary source file
        let file_path = temp_dir.path().join("hello.txt");
        let mut file = fs::File::create(&file_path)?;
        writeln!(file, "Hello, world!")?;

        // Define the destination path for the archive
        let archive_path = temp_dir.path().join("hello.tar.gz");

        // Archive the file using `archive_file`
        archive_file(&file_path, &archive_path).await?;

        // Unpack the tar.gz file to the same temp directory
        let unpacked_dir = tempdir()?;
        unpack(&fs::read(archive_path)?, unpacked_dir.path())?;

        // Check if the file was unpacked correctly
        let unpacked_file = unpacked_dir.path().join("hello.txt");
        assert!(unpacked_file.exists());

        let content = fs::read_to_string(unpacked_file)?;
        assert_eq!(content.trim(), "Hello, world!");

        Ok(())
    }
}
