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

use std::{collections::HashMap, path::Path};

use anyhow::Result;

use hmt_manifest::{Artifact, Package, Release, ReleaseManifest};
use hmt_utils::checksum::{self, CHECKSUM_FILE_SUFFIX};
use tracing::warn;

/// Generate a release manifest based on package configuration and artifacts
///
/// # Arguments
/// * `config` - Package configuration containing target information
/// * `artifacts_dir` - Directory containing the release artifacts
/// * `version` - Version string for the release
///
/// # Returns
/// A Result containing the generated ReleaseManifest
pub fn generate(package: &Package, artifacts_dir: &Path, version: &str) -> Result<ReleaseManifest> {
    let release = Release::new(version.to_string());
    let mut manifest = ReleaseManifest::new(release, HashMap::new());

    for target in &package.targets {
        let artifact_name = format!("{}-{}-{}.tar.gz", package.name, version, target);

        let checksum_file = format!("{artifact_name}.{CHECKSUM_FILE_SUFFIX}");
        let checksum_path = artifacts_dir.join(checksum_file);

        // In local development mode, we can only generate artifacts for the current platform
        // and cannot cross-compile for other platforms, so we skip them.
        if !checksum_path.exists() {
            warn!("Artifact not found: {}, skipped", artifact_name);
            continue;
        }

        let hash = checksum::read(&checksum_path)?;
        let url = format!("{}/releases/download/{}/{}", package.repository, version, artifact_name);

        manifest.add_artifact(target.clone(), Artifact { url, hash });
    }

    Ok(manifest)
}
