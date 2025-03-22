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

pub fn is_executable(path: &Path) -> bool {
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;

        path.metadata().map(|m| m.permissions().mode() & 0o111 != 0).unwrap_or(false)
    }
    #[cfg(windows)]
    {
        path.extension().map(|ext| ext == "exe").unwrap_or(false)
    }
}

#[cfg(test)]
mod tests {
    #[cfg(unix)]
    use std::{fs, os::unix::fs::PermissionsExt};
    use std::{fs::File, path::PathBuf};

    use super::*;

    #[test]
    fn test_is_executable_unix_executable_file() {
        #[cfg(unix)]
        {
            let temp_dir = tempfile::tempdir().unwrap();
            let file_path = temp_dir.path().join("test_executable");
            File::create(&file_path).unwrap();
            let mut perms = fs::metadata(&file_path).unwrap().permissions();
            perms.set_mode(0o755); // Set executable permissions
            fs::set_permissions(&file_path, perms).unwrap();

            assert!(is_executable(&file_path));
        }
    }

    #[test]
    fn test_is_executable_unix_non_executable_file() {
        #[cfg(unix)]
        {
            let temp_dir = tempfile::tempdir().unwrap();
            let file_path = temp_dir.path().join("test_non_executable");
            File::create(&file_path).unwrap();
            let mut perms = fs::metadata(&file_path).unwrap().permissions();
            perms.set_mode(0o644); // No executable permissions
            fs::set_permissions(&file_path, perms).unwrap();

            assert!(!is_executable(&file_path));
        }
    }

    #[test]
    fn test_is_executable_windows_exe_file() {
        #[cfg(windows)]
        {
            let temp_dir = tempfile::tempdir().unwrap();
            let file_path = temp_dir.path().join("test_executable.exe");
            File::create(&file_path).unwrap();

            assert!(is_executable(&file_path));
        }
    }

    #[test]
    fn test_is_executable_windows_non_exe_file() {
        #[cfg(windows)]
        {
            let temp_dir = tempfile::tempdir().unwrap();
            let file_path = temp_dir.path().join("test_non_executable.txt");
            File::create(&file_path).unwrap();

            assert!(!is_executable(&file_path));
        }
    }

    #[test]
    fn test_is_executable_nonexistent_file() {
        let nonexistent_path = PathBuf::from("nonexistent_file");
        assert!(!is_executable(&nonexistent_path));
    }
}
