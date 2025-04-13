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

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// `PackageIndex` keeps track of all versions of a component package.
///
/// This structure represents an index of all versions for a given package,
/// including metadata (e.g., name, language, kind) and version information.
///
/// Example:
/// ```toml
/// [meta]
/// name = "solidity-detector-foundry"
/// language = "solidity"
/// kind = "detector"
/// description = "Solidity detector for Foundry projects"
///
/// latest = "1.2.0"
///
/// [versions."1.2.0"]
/// manifest = "https://github.com/hummanta/solidity-detector-foundry/releases/download/v1.2.0/manifest.toml"
///
/// [versions."1.1.0"]
/// manifest = "https://github.com/hummanta/solidity-detector-foundry/releases/download/v1.1.0/manifest.toml"
/// ```
#[derive(Debug, Serialize, Deserialize)]
pub struct PackageIndex {
    /// Metadata for the package, such as name, language, and kind.
    pub meta: PackageMeta,

    /// The latest version of the package.
    pub latest: String,

    /// A mapping of all versions to their manifest URLs.
    pub versions: HashMap<String, VersionInfo>,
}

/// `PackageMeta` contains general metadata for a package.
#[derive(Debug, Serialize, Deserialize)]
pub struct PackageMeta {
    /// The name of the package.
    pub name: String,

    /// The programming language used for the package.
    pub language: String,

    /// The kind of the package (e.g., "detector", "compiler").
    pub kind: String,

    /// A description of the package (optional).
    pub description: Option<String>,
}

/// `VersionInfo` contains information about a specific version of the package.
#[derive(Debug, Serialize, Deserialize)]
pub struct VersionInfo {
    /// The URL to the manifest file for the specific version.
    pub manifest: String,
}

/// `PackageManifest` describes a specific released version of a package.
///
/// This structure holds detailed information about a released version of the package,
/// including version-specific metadata, target platforms, and artifact details.
///
/// Example:
/// ```toml
/// [package]
/// name = "solidity-detector-foundry"
/// version = "1.2.0"
/// description = "Solidity detector for Foundry projects"
/// language = "solidity"
/// kind = "detector"
///
/// targets = [
///   "x86_64-apple-darwin",
///   "aarch64-apple-darwin",
///   "x86_64-unknown-linux-gnu"
/// ]
///
/// [artifacts.x86_64-apple-darwin]
/// url = "https://github.com/hummanta/solidity-detector-foundry/releases/download/v1.2.0/solidity-detector-foundry-x86_64-apple-darwin.tar.gz"
/// hash = "a80a0dd7425173064ce6d1a4ba04b18a967484d6f0d19080170843229065c006"
///
/// [artifacts.aarch64-apple-darwin]
/// url = "..."
/// hash = "..."
///
/// [artifacts.x86_64-unknown-linux-gnu]
/// url = "..."
/// hash = "..."
/// ```
#[derive(Debug, Serialize, Deserialize)]
pub struct PackageManifest {
    /// General information about the package.
    pub package: PackageInfo,

    /// The list of target platforms for which the package is built.
    pub targets: Vec<String>,

    /// A mapping of target platforms to their corresponding artifacts.
    pub artifacts: HashMap<String, Artifact>,
}

/// `PackageInfo` contains detailed information about the package itself.
#[derive(Debug, Serialize, Deserialize)]
pub struct PackageInfo {
    /// The name of the package.
    pub name: String,

    /// The version of the package.
    pub version: String,

    /// An optional description of the package.
    pub description: Option<String>,

    /// The programming language used for the package.
    pub language: String,

    /// The kind of the package (e.g., "detector", "compiler").
    pub kind: String,
}

/// `Artifact` contains the URL and hash for a specific artifact of a target platform.
#[derive(Debug, Serialize, Deserialize)]
pub struct Artifact {
    /// The URL to download the artifact from.
    pub url: String,

    /// The hash of the artifact file, used for integrity checking.
    pub hash: String,
}
