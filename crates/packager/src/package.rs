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

use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use walkdir::WalkDir;

use crate::{archive::archive, checksum::checksum, utils::is_executable};

/// Package all executables in the output directory
pub async fn package(output_dir: &Path, version: &str, target: &str) -> Result<()> {
    for entry in WalkDir::new(output_dir).max_depth(1).into_iter().filter_map(Result::ok) {
        let path = entry.into_path();
        if path.is_file() && is_executable(&path) {
            process(path, output_dir, version, target).await?;
        }
    }

    Ok(())
}

/// Process a single executable by creating a tar.gz archive and checksum
async fn process(path: PathBuf, output_dir: &Path, version: &str, target: &str) -> Result<()> {
    let bin_name = path.file_name().unwrap().to_string_lossy().to_string();
    let archive_name = format!("{}-{}-{}.tar.gz", bin_name, version, target);
    let archive_path = output_dir.join(&archive_name);
    let checksum_path = output_dir.join(format!("{}.sha256", archive_name));

    println!("{}: \n  {}\n  {}\n", bin_name, archive_path.display(), checksum_path.display());

    // Create a tar.gz archive for the executable
    archive(&path, &archive_path)
        .await
        .context(format!("Failed to create archive for {:?}", path))?;

    // Generate checksum for the archive
    checksum(&archive_path, &checksum_path)
        .await
        .context(format!("Failed to generate checksum for {:?}", archive_path))?;

    Ok(())
}
#[cfg(test)]
mod tests {
    use std::{
        fs::{self, File},
        io::Write,
        os::unix::fs::PermissionsExt,
    };
    use tempfile::tempdir;

    use super::*;

    #[tokio::test]
    async fn test_package_with_executable() {
        let temp_dir = tempdir().unwrap();
        let output_dir = temp_dir.path();

        // Create a mock executable file
        let executable_path = output_dir.join("mock_executable");
        let mut file = File::create(&executable_path).unwrap();
        file.write_all(b"#!/bin/bash\necho Hello").unwrap();
        #[cfg(unix)]
        {
            fs::set_permissions(&executable_path, fs::Permissions::from_mode(0o755)).unwrap();
        }

        let version = "v1.0.0";
        let target = "x86_64-unknown-linux-gnu";

        let result = package(output_dir, version, target).await;
        assert!(result.is_ok());

        let archive_name = format!("mock_executable-{}-{}.tar.gz", version, target);
        let checksum_name = format!("{}.sha256", archive_name);

        assert!(output_dir.join(&archive_name).exists());
        assert!(output_dir.join(&checksum_name).exists());
    }

    #[tokio::test]
    async fn test_package_with_non_executable() {
        let temp_dir = tempdir().unwrap();
        let output_dir = temp_dir.path();

        // Create a non-executable file
        let non_executable_path = output_dir.join("non_executable");
        let mut file = File::create(&non_executable_path).unwrap();
        file.write_all(b"Hello, world!").unwrap();

        let version = "v1.0.0";
        let target = "x86_64-unknown-linux-gnu";

        let result = package(output_dir, version, target).await;
        assert!(result.is_ok());

        let archive_name = format!("non_executable-{}-{}.tar.gz", version, target);
        let checksum_name = format!("{}.sha256", archive_name);

        assert!(!output_dir.join(&archive_name).exists());
        assert!(!output_dir.join(&checksum_name).exists());
    }

    #[tokio::test]
    async fn test_process_creates_archive_and_checksum() {
        let temp_dir = tempdir().unwrap();
        let output_dir = temp_dir.path();

        // Create a mock executable file
        let executable_path = output_dir.join("mock_executable");
        let mut file = File::create(&executable_path).unwrap();
        file.write_all(b"#!/bin/bash\necho Hello").unwrap();
        #[cfg(unix)]
        {
            fs::set_permissions(&executable_path, fs::Permissions::from_mode(0o755)).unwrap();
        }

        let version = "v1.0.0";
        let target = "x86_64-unknown-linux-gnu";

        let result = process(executable_path.clone(), output_dir, version, target).await;
        assert!(result.is_ok());

        let archive_name = format!("mock_executable-{}-{}.tar.gz", version, target);
        let checksum_name = format!("{}.sha256", archive_name);

        assert!(output_dir.join(&archive_name).exists());
        assert!(output_dir.join(&checksum_name).exists());
    }
}
