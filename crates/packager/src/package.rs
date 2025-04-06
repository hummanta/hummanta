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

use hummanta_utils::{archive::archive_file, checksum};

use crate::utils::is_executable;

/// Package all executables in the output directory
pub async fn package(
    input_path: &Path,
    output_path: &Path,
    target: &str,
    version: &str,
) -> Result<()> {
    for entry in WalkDir::new(input_path).max_depth(1).into_iter().filter_map(Result::ok) {
        let path = entry.into_path();
        if path.is_file() && is_executable(&path) {
            process(path, output_path, target, version).await?;
        }
    }

    Ok(())
}

/// Process a single executable by creating a tar.gz archive and checksum
async fn process(path: PathBuf, output_path: &Path, target: &str, version: &str) -> Result<()> {
    let bin_name = path.file_stem().unwrap().to_string_lossy().to_string();
    let archive_name = format!("{}-{}-{}.tar.gz", bin_name, version, target);
    let archive_path = output_path.join(&archive_name);
    let checksum_path = output_path.join(format!("{}.sha256", archive_name));

    println!("{}: \n  {}\n  {}\n", bin_name, archive_path.display(), checksum_path.display());

    // Create a tar.gz archive for the executable
    archive_file(&path, &archive_path)
        .await
        .context(format!("Failed to create archive for {:?}", path))?;

    // Generate checksum for the archive
    checksum::generate(&archive_path, &checksum_path)
        .await
        .context(format!("Failed to generate checksum for {:?}", archive_path))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use std::fs;
    #[cfg(unix)]
    use std::os::unix::fs::PermissionsExt;
    use tempfile::tempdir;

    use super::*;

    #[tokio::test]
    async fn test_package_with_executable() {
        let temp_dir = tempdir().unwrap();
        let input_path = temp_dir.path();
        let output_path = temp_dir.path();

        // Dynamically set the file name and target platform based on the operating system
        let (executable_name, target) = if cfg!(windows) {
            ("mock-executable.exe", "x86_64-pc-windows-msvc") // For Windows
        } else {
            ("mock-executable", "x86_64-unknown-linux-gnu") // For other platforms
        };

        // Create an empty mock executable file
        let executable_path = input_path.join(executable_name);
        fs::File::create(&executable_path).unwrap(); // Simply create the file

        #[cfg(unix)]
        {
            // Set executable permissions for Unix platforms
            fs::set_permissions(&executable_path, fs::Permissions::from_mode(0o755)).unwrap();
        }

        let version = "v1.0.0";

        // Call the package function to process the file
        let result = package(input_path, output_path, target, version).await;
        assert!(result.is_ok());

        // Construct the archive and checksum file names
        let archive_name = format!("mock-executable-{}-{}.tar.gz", version, target);
        let checksum_name = format!("{}.sha256", archive_name);

        // Ensure the archive and checksum files are created
        assert!(output_path.join(&archive_name).exists());
        assert!(output_path.join(&checksum_name).exists());
    }

    #[tokio::test]
    async fn test_package_with_non_executable() {
        let temp_dir = tempdir().unwrap();
        let input_path = temp_dir.path();
        let output_path = temp_dir.path();

        // Dynamically set the target platform based on the operating system
        let target = if cfg!(windows) {
            "x86_64-pc-windows-msvc" // For Windows
        } else {
            "x86_64-unknown-linux-gnu" // For other platforms
        };

        // Create a non-executable file
        let non_executable_path = input_path.join("non-executable");
        fs::File::create(&non_executable_path).unwrap();

        let version = "v1.0.0";

        // Call the package function to process the file
        let result = package(input_path, output_path, target, version).await;
        assert!(result.is_ok());

        // Construct the archive and checksum file names
        let archive_name = format!("non-executable-{}-{}.tar.gz", version, target);
        let checksum_name = format!("{}.sha256", archive_name);

        // Ensure that the archive and checksum files do not exist since the file is not executable
        assert!(!output_path.join(&archive_name).exists());
        assert!(!output_path.join(&checksum_name).exists());
    }
}
