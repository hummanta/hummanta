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

    // Get the target directory based on the target and profile
    pub fn target_dir(&self) -> PathBuf {
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

    pub fn output_dir(&self) -> PathBuf {
        let target_dir = env::var("CARGO_TARGET_DIR").unwrap_or_else(|_| "target".to_string());
        Path::new(&target_dir).join("artifacts")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_target_with_value() {
        let args = Arguments {
            target: "x86_64-unknown-linux-gnu".to_string(),
            version: "".to_string(),
            profile: "".to_string(),
        };
        assert_eq!(args.target(), "x86_64-unknown-linux-gnu");
    }

    #[test]
    fn test_target_without_value() {
        let args =
            Arguments { target: "".to_string(), version: "".to_string(), profile: "".to_string() };
        assert_eq!(args.target(), target_triple::TARGET.to_string());
    }

    #[test]
    fn test_version_with_value() {
        let args = Arguments {
            target: "".to_string(),
            version: "v1.0.0".to_string(),
            profile: "".to_string(),
        };
        assert_eq!(args.version(), "v1.0.0");
    }

    #[test]
    fn test_version_without_value() {
        let args =
            Arguments { target: "".to_string(), version: "".to_string(), profile: "".to_string() };
        assert_eq!(args.version(), format!("v{}", env!("CARGO_PKG_VERSION")));
    }

    #[test]
    fn test_profile_with_value() {
        let args = Arguments {
            target: "".to_string(),
            version: "".to_string(),
            profile: "release".to_string(),
        };
        assert_eq!(args.profile(), "release");
    }

    #[test]
    fn test_profile_without_value() {
        let args =
            Arguments { target: "".to_string(), version: "".to_string(), profile: "".to_string() };
        assert_eq!(args.profile(), "debug");
    }

    #[test]
    fn test_output_dir_with_target_and_profile() {
        let args = Arguments {
            target: "x86_64-unknown-linux-gnu".to_string(),
            version: "".to_string(),
            profile: "release".to_string(),
        };
        assert_eq!(
            args.target_dir(),
            Path::new("target").join("x86_64-unknown-linux-gnu").join("release")
        );
    }

    #[test]
    fn test_output_dir_without_target() {
        let args = Arguments {
            target: "".to_string(),
            version: "".to_string(),
            profile: "debug".to_string(),
        };
        assert_eq!(args.target_dir(), Path::new("target").join("debug"));
    }
}
