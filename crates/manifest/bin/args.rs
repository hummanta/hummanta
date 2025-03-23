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

    /// The profile to build with (e.g., release)
    #[arg(long = "profile")]
    profile: String,

    /// The target triple (e.g., x86_64-unknown-linux-gnu)
    #[arg(long = "target")]
    target: String,

    /// The version of the package (e.g., v0.1.1)
    #[arg(long = "version")]
    version: String,
}

impl Arguments {
    // Determine the profile, defaulting to "debug" if not set
    pub fn profile(&self) -> String {
        if self.profile.is_empty() {
            env::var("CARGO_CFG_PROFILE").unwrap_or_else(|_| "debug".to_string())
        } else {
            self.profile
                .eq("dev")
                .then(|| "debug".to_string())
                .unwrap_or_else(|| self.profile.clone())
        }
    }

    // Determine the target triple, defaulting to the system's target if not set
    pub fn target(&self) -> String {
        if self.target.is_empty() {
            target_triple::TARGET.to_string()
        } else {
            self.target.clone()
        }
    }

    // Determine the version, defaulting to CARGO_PKG_VERSION with 'v' prefix if not set
    pub fn version(&self) -> String {
        if self.version.is_empty() {
            format!("v{}", env!("CARGO_PKG_VERSION"))
        } else {
            self.version.clone()
        }
    }

    // Get the output directory based on the target and profile
    pub fn output_dir(&self) -> PathBuf {
        let target = self.target();
        let profile = self.profile();

        let target_dir = env::var("CARGO_TARGET_DIR").unwrap_or_else(|_| "target".to_string());
        let output_dir = if self.target.is_empty() {
            Path::new(&target_dir).join(profile)
        } else {
            Path::new(&target_dir).join(target).join(profile)
        };

        output_dir
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_profile() {
        let args = Arguments {
            path: PathBuf::from("."),
            local: false,
            profile: String::new(),
            target: String::new(),
            version: String::new(),
        };
        env::set_var("CARGO_CFG_PROFILE", "release");
        assert_eq!(args.profile(), "release");
    }

    #[test]
    fn test_custom_profile() {
        let args = Arguments {
            path: PathBuf::from("."),
            local: false,
            profile: "dev".to_string(),
            target: String::new(),
            version: String::new(),
        };
        assert_eq!(args.profile(), "debug");
    }

    #[test]
    fn test_default_target() {
        let args = Arguments {
            path: PathBuf::from("."),
            local: false,
            profile: String::new(),
            target: String::new(),
            version: String::new(),
        };
        assert_eq!(args.target(), target_triple::TARGET.to_string());
    }

    #[test]
    fn test_custom_target() {
        let args = Arguments {
            path: PathBuf::from("."),
            local: false,
            profile: String::new(),
            target: "x86_64-unknown-linux-gnu".to_string(),
            version: String::new(),
        };
        assert_eq!(args.target(), "x86_64-unknown-linux-gnu");
    }

    #[test]
    fn test_default_version() {
        let args = Arguments {
            path: PathBuf::from("."),
            local: false,
            profile: String::new(),
            target: String::new(),
            version: String::new(),
        };
        assert_eq!(args.version(), format!("v{}", env!("CARGO_PKG_VERSION")));
    }

    #[test]
    fn test_custom_version() {
        let args = Arguments {
            path: PathBuf::from("."),
            local: false,
            profile: String::new(),
            target: String::new(),
            version: "v1.2.3".to_string(),
        };
        assert_eq!(args.version(), "v1.2.3");
    }

    #[test]
    fn test_output_dir_default_target() {
        let args = Arguments {
            path: PathBuf::from("."),
            local: false,
            profile: "release".to_string(),
            target: String::new(),
            version: String::new(),
        };
        env::set_var("CARGO_TARGET_DIR", "custom_target_dir");
        assert_eq!(args.output_dir(), Path::new("custom_target_dir").join("release"));
    }

    #[test]
    fn test_output_dir_custom_target() {
        let args = Arguments {
            path: PathBuf::from("."),
            local: false,
            profile: "release".to_string(),
            target: "x86_64-unknown-linux-gnu".to_string(),
            version: String::new(),
        };
        env::set_var("CARGO_TARGET_DIR", "custom_target_dir");
        assert_eq!(
            args.output_dir(),
            Path::new("custom_target_dir").join("x86_64-unknown-linux-gnu").join("release")
        );
    }
}
