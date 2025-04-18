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

use std::path::PathBuf;

use clap::Parser;

/// Generate Hummanta-compatible package and release manifests
#[derive(Debug, Parser)]
pub struct Args {
    /// Path to the hmt-package.toml file
    #[arg(long)]
    pub package: PathBuf,

    /// Directory containing built artifact tarballs and their .sha256 checksums
    #[arg(long)]
    pub artifacts_dir: PathBuf,

    /// Output directory for manifest files (index.toml and release-<version>.toml)
    #[arg(long)]
    pub output_dir: PathBuf,

    /// Version to publish
    #[arg(long)]
    pub version: String,
}
