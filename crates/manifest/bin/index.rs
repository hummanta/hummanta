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

/// Generate the index manifest
///
/// Copy the file from the input path to the output path
pub fn generate(input_path: &Path, output_path: &Path) {
    fs::copy(input_path, output_path).unwrap_or_else(|_| {
        panic!("Failed to copy {} to {}", input_path.display(), output_path.display())
    });
}

#[cfg(test)]
mod tests {
    use std::{
        fs::{self, File},
        io::Write,
    };
    use tempfile::tempdir;

    use super::*;

    #[test]
    fn test_generate_success() {
        let temp_dir = tempdir().unwrap();
        let input_file_path = temp_dir.path().join("input.txt");
        let output_file_path = temp_dir.path().join("output.txt");

        // Create a sample input file
        let mut input_file = File::create(&input_file_path).unwrap();
        writeln!(input_file, "Hello, world!").unwrap();

        // Call the generate function
        generate(&input_file_path, &output_file_path);

        // Verify the output file exists and has the same content
        let output_content = fs::read_to_string(output_file_path).unwrap();
        assert_eq!(output_content, "Hello, world!\n");
    }

    #[test]
    #[should_panic(expected = "Failed to copy")]
    fn test_generate_input_file_missing() {
        let temp_dir = tempdir().unwrap();
        let input_file_path = temp_dir.path().join("nonexistent.txt");
        let output_file_path = temp_dir.path().join("output.txt");

        // Call the generate function with a missing input file
        generate(&input_file_path, &output_file_path);
    }

    #[test]
    #[should_panic(expected = "Failed to copy")]
    fn test_generate_output_path_invalid() {
        let temp_dir = tempdir().unwrap();
        let input_file_path = temp_dir.path().join("input.txt");

        // Create a sample input file
        let mut input_file = File::create(&input_file_path).unwrap();
        writeln!(input_file, "Hello, world!").unwrap();

        // Call the generate function with an invalid output path
        let invalid_output_path = temp_dir.path().join("nonexistent_dir/output.txt");
        generate(&input_file_path, &invalid_output_path);
    }
}
