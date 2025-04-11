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

use std::{
    env,
    path::{Path, PathBuf},
};

use clap::Parser;

#[derive(Debug, Parser)]
pub struct Arguments {
    /// Specify the path of the manifest directory
    #[arg(long = "path")]
    pub path: PathBuf,

    /// Generate local manifests with file paths
    #[arg(long = "local")]
    pub local: bool,

    /// The version of the package (e.g., v0.1.1)
    #[arg(long = "version")]
    pub version: String,
}

impl Arguments {
    // Determine the version, defaulting to CARGO_PKG_VERSION with 'v' prefix if not set
    pub fn version(&self) -> String {
        if self.version.is_empty() {
            format!("v{}", env!("CARGO_PKG_VERSION"))
        } else {
            self.version.clone()
        }
    }

    pub fn artifact_dir(&self) -> PathBuf {
        let target_dir = env::var("CARGO_TARGET_DIR").unwrap_or_else(|_| "target".to_string());
        Path::new(&target_dir).join("artifacts")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_version() {
        let args = Arguments { path: PathBuf::from("."), local: false, version: String::new() };
        assert_eq!(args.version(), format!("v{}", env!("CARGO_PKG_VERSION")));
    }

    #[test]
    fn test_custom_version() {
        let args =
            Arguments { path: PathBuf::from("."), local: false, version: "v1.2.3".to_string() };
        assert_eq!(args.version(), "v1.2.3");
    }
}
