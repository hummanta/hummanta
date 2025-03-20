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
    collections::HashMap,
    env, fs,
    path::{Path, PathBuf},
};

use clap::Parser;
use hummanta_manifest::{
    IndexManifest, PackageToolchain, ReleaseToolchain, TargetInfo, Toolchain, ToolchainManifest,
};
use sha2::{Digest, Sha256};
use tokio::{fs::File, io::AsyncReadExt};

pub const HUMMANTA_GITHUB_REPO: &str = "hummanta/hummanta";

#[derive(Debug, Parser)]
pub struct Arguments {
    /// Specify the path of the manifest directory
    #[arg(short = 'p', long = "path")]
    pub path: PathBuf,

    /// Generate local manifests with file paths
    #[arg(short = 'l', long = "local")]
    pub local: bool,
}

struct ReleaseOption {
    local: bool,
    repo: String,
    version: String,
}

#[tokio::main]
async fn main() {
    let args = Arguments::parse();

    // Prepare the output directory
    let output_dir = output_dir().join("manifests");
    if !output_dir.exists() {
        fs::create_dir_all(&output_dir).expect("Failed to create output directory");
    }

    // Process the toolchain manifests.
    let toolchain_input_dir = args.path.join("toolchains");
    let toolchain_output_dir = output_dir.join("toolchains");

    if !toolchain_output_dir.exists() {
        fs::create_dir_all(&toolchain_output_dir)
            .expect("Failed to create toolchain output directory");
    }

    let opt = ReleaseOption {
        local: args.local,
        repo: HUMMANTA_GITHUB_REPO.to_owned(),
        version: env!("CARGO_PKG_VERSION").to_owned(),
    };

    process_toolchain_manifests(&toolchain_input_dir, &toolchain_output_dir, &opt).await;
}

/// Process the index manifest
///
/// Copy the file from the input path to the output path
fn process_index_manifest(input_path: &Path, output_path: &Path) {
    fs::copy(input_path, output_path).unwrap_or_else(|_| {
        panic!("Failed to copy {} to {}", input_path.display(), output_path.display())
    });
}

// process the toolchain manifests
async fn process_toolchain_manifests(input_path: &Path, output_path: &Path, opt: &ReleaseOption) {
    // Read the index.toml file and convert it into an IndexManifest struct.
    let index_input_path = input_path.join("index.toml");
    let manifest = IndexManifest::from_file(&index_input_path)
        .unwrap_or_else(|_| panic!("Failed to parse TOML at {}", index_input_path.display()));

    // For each toolchain entry in the IndexManifest, read the corresponding
    // toolchain file, parse it into a ToolchainManifest struct, and write
    // the serialized struct to the output path.
    for (_, path) in manifest.iter() {
        process_toolchain_manifest(&input_path.join(path), &output_path.join(path), opt).await;
    }

    // Copy the index.toml file to the output directory.
    let index_output_path = output_path.join("index.toml");
    process_index_manifest(&index_input_path, &index_output_path);
}

/// Process the toolchain manifest
async fn process_toolchain_manifest(input_path: &Path, output_path: &Path, opt: &ReleaseOption) {
    let manifest = ToolchainManifest::from_file(input_path)
        .unwrap_or_else(|_| panic!("Failed to parse TOML at {}", input_path.display()));

    // For each toolchain entry in the ToolchainManifest, convert it into
    // ReleaseToolchain struct if it is a PackageToolchain.
    let mut result = ToolchainManifest::new();

    for (category, tools) in manifest.iter() {
        for (name, toolchain) in tools {
            if let Toolchain::Package(package) = toolchain {
                let release = build_release_toolchain(package, opt).await.into();
                result.insert(category.clone(), name.clone(), release);
            }
        }
    }

    // write the result to the output path.
    let toml_string = toml::to_string(&result)
        .unwrap_or_else(|_| panic!("Failed to serialize to TOML: {}", input_path.display()));
    fs::write(output_path, toml_string)
        .unwrap_or_else(|_| panic!("Failed to write to output path: {}", output_path.display()));
}

/// Build the release toolchain
async fn build_release_toolchain(pkg: &PackageToolchain, opt: &ReleaseOption) -> ReleaseToolchain {
    let version = opt.version.clone();
    let target = match opt.local {
        true => build_local_target(pkg).await,
        false => build_github_target(pkg, &opt.repo, &opt.version),
    };
    ReleaseToolchain::new(version, target)
}

/// Build the local target, the url is a file path, and just one target is supported.
async fn build_local_target(pkg: &PackageToolchain) -> HashMap<String, TargetInfo> {
    let mut targets = HashMap::new();

    let target = target_triple::TARGET;
    let path = output_dir().join(pkg.name()).canonicalize().expect("Failed to canonicalize");
    let url = format!("file://{}", path.display());
    let hash = calculate_sha256(&path).await.expect("Failed to calculate SHA256");
    targets.insert(target.to_string(), TargetInfo::new(url, hash));

    targets
}

/// Build the github target, the url is a github release url.
fn build_github_target(
    pkg: &PackageToolchain,
    repo: &str,
    version: &str,
) -> HashMap<String, TargetInfo> {
    let mut targets = HashMap::new();

    for target in &pkg.targets {
        let file = format!("{}-{}-{}.tar.gz", pkg.name(), version, target);
        let url = format!("https://github.com/{}/releases/download/{}/{}", repo, version, file);
        let hash = "#SHA256 HASH PLACEHOLDER#".to_string();
        targets.insert(target.to_string(), TargetInfo::new(url, hash));
    }

    targets
}

/// Get the output directory
fn output_dir() -> PathBuf {
    let profile = env::var("CARGO_CFG_PROFILE").unwrap_or_else(|_| "debug".to_string());
    let target_dir = env::var("CARGO_TARGET_DIR").unwrap_or_else(|_| "target".to_string());

    Path::new(&target_dir).join(profile)
}

/// Calculate the SHA256 hash of a file
async fn calculate_sha256(path: impl AsRef<Path>) -> Result<String, Box<dyn std::error::Error>> {
    let mut file = File::open(path).await?;
    let mut hasher = Sha256::new();
    let mut buffer = [0; 4096];

    while let Ok(bytes_read) = file.read(&mut buffer).await {
        if bytes_read == 0 {
            break;
        }
        hasher.update(&buffer[..bytes_read]);
    }

    let result = hasher.finalize();
    Ok(format!("{:x}", result))
}
