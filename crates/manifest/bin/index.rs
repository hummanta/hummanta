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
