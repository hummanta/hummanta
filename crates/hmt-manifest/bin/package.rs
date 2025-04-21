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

use anyhow::Result;
use semver::Version;

use hmt_manifest::{ManifestFile, Package, PackageManifest};

/// Creates a new package manifest file with the given configuration
///
/// # Arguments
/// * `config` - Package configuration containing metadata and targets
/// * `path` - Path where the manifest file should be created
/// * `version` - Initial version of the package
pub fn create(package: &Package, path: &Path, version: &str) -> Result<()> {
    let mut manifest = PackageManifest::new(package.clone(), version.to_string());
    manifest.add_release(version.to_string(), format!("release-{}.toml", version));

    manifest.save(path)?;
    Ok(())
}

/// Updates an existing package manifest with new configuration and version
///
/// # Arguments
/// * `config` - Updated package configuration
/// * `path` - Path to the existing manifest file
/// * `version` - New version to be added
pub fn update(package: &Package, path: &Path, version: &str) -> Result<()> {
    // Read the existing manifest
    let mut manifest = PackageManifest::load(path)?;

    // Update package metadata and targets
    manifest.package = package.clone();

    // Update the latest version if the new version is higher
    if Version::parse(version)? > Version::parse(manifest.latest.as_str())? {
        manifest.latest = version.to_string();
    }

    // Add new release file if it doesn't exist
    let release = format!("release-{}.toml", version);
    if !manifest.releases.contains_key(version) {
        manifest.add_release(version.to_string(), release);
    }

    manifest.save(path)?;
    Ok(())
}
