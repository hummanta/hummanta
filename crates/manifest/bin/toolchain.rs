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

use std::{collections::HashMap, fs, path::Path};

use hummanta_manifest::*;

use crate::{args::Arguments, index, HUMMANTA_GITHUB_REPO};

const INDEX_MANIFEST_NAME: &str = "index.toml";
const TOOLCHAINS_DIR_NAME: &str = "toolchains";
const CHECKSUM_FILE_SUFFIX: &str = ".sha256";

/// process the toolchain manifests
pub async fn generate(
    input_path: &Path,
    artifact_path: &Path,
    output_path: &Path,
    args: &Arguments,
) {
    // Prepare the input, output and index paths.
    let input_path = input_path.join(TOOLCHAINS_DIR_NAME);
    if !input_path.exists() {
        eprintln!("Error: toolchains directory {:?} does not exist.", input_path);
        std::process::exit(1);
    }

    let output_path = output_path.join(TOOLCHAINS_DIR_NAME);
    if !output_path.exists() {
        fs::create_dir_all(&output_path).expect("Failed to create toolchain output directory");
    }

    let index_input_path = input_path.join(INDEX_MANIFEST_NAME);
    let index_output_path = output_path.join(INDEX_MANIFEST_NAME);

    // Read the index.toml file and convert it into an IndexManifest struct.
    let manifest = IndexManifest::read(&index_input_path)
        .unwrap_or_else(|_| panic!("Failed to parse TOML at {}", index_input_path.display()));

    // For each toolchain entry in the IndexManifest, read the corresponding
    // toolchain file, parse it into a ToolchainManifest struct, and write
    // the serialized struct to the output path.
    for (_, path) in manifest.iter() {
        process(&input_path.join(path), artifact_path, &output_path.join(path), args).await;
    }

    // Copy the index.toml file to the output directory.
    index::generate(&index_input_path, &index_output_path);
}

/// Process the toolchain manifest
async fn process(input_path: &Path, artifact_path: &Path, output_path: &Path, args: &Arguments) {
    let manifest = ToolchainManifest::read(input_path)
        .unwrap_or_else(|_| panic!("Failed to parse TOML at {}", input_path.display()));

    // For each toolchain entry in the ToolchainManifest, convert it into
    // ReleaseToolchain struct if it is a PackageToolchain.
    let mut result = ToolchainManifest::new();

    for (category, tools) in manifest.iter() {
        for (name, toolchain) in tools {
            if let Toolchain::Package(package) = toolchain {
                let release = build(package, artifact_path, args).await;
                println!(
                    "Generated manifests for package: {name} with targets: {:?}",
                    &release.targets.keys()
                );
                result.insert(category.clone(), name.clone(), release.into());
            }
        }
    }

    // write the result to the output path.
    result
        .write(output_path)
        .unwrap_or_else(|_| panic!("Failed to write to output path: {}", output_path.display()));
}

/// Build the release toolchain
async fn build(pkg: &PackageToolchain, artifact_path: &Path, args: &Arguments) -> ReleaseToolchain {
    let version = args.version();
    let bin_name = pkg.name();

    let mut targets = HashMap::new();

    for target in &pkg.targets {
        let archive_name = format!("{}-{}-{}.tar.gz", bin_name, version, target);

        let archive_path = artifact_path.join(&archive_name);
        if !archive_path.exists() {
            eprintln!("Archive not found: {}, skipped", archive_path.display());
            continue;
        }

        let url = if args.local {
            let archive_path = archive_path
                .canonicalize()
                .unwrap_or_else(|_| panic!("Failed to canonicalize: {}", archive_path.display()));
            format!("file://{}", archive_path.display())
        } else {
            format!(
                "https://{}/releases/download/{}/{}",
                HUMMANTA_GITHUB_REPO, version, archive_name
            )
        };

        let checksum_file = format!("{}{}", archive_name, CHECKSUM_FILE_SUFFIX);
        let checksum_path = artifact_path.join(checksum_file);
        let hash = fs::read_to_string(&checksum_path).unwrap_or_else(|_| {
            panic!("Failed to read SHA256 from file: {}", checksum_path.display())
        });

        targets.insert(target.to_string(), TargetInfo::new(url, hash));
    }

    ReleaseToolchain::new(version, targets)
}
