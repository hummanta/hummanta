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
use hummanta_manifest::*;

const HUMMANTA_GITHUB_REPO: &str = "github.com/hummanta/hummanta";

#[derive(Debug, Parser)]
struct Arguments {
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

#[tokio::main]
async fn main() {
    let args = Arguments::parse();

    // Prepare the input directory
    if !args.path.exists() {
        eprintln!("Error: input directory {:?} does not exist.", args.path);
        std::process::exit(1);
    }

    // Prepare the output directory
    let output_dir = args.output_dir();
    if !output_dir.exists() {
        eprintln!("Error: output directory {:?} does not exist.", output_dir);
        std::process::exit(1);
    }

    // Prepare the manifest output directory
    let manifests_output_dir = output_dir.join("manifests");
    if !manifests_output_dir.exists() {
        fs::create_dir_all(&output_dir).expect("Failed to create output directory for manifests");
    }

    // Process the toolchain manifests.
    let toolchain_input_dir = args.path.join("toolchains");
    if !toolchain_input_dir.exists() {
        eprintln!("Error: toolchains directory {:?} does not exist.", toolchain_input_dir);
        std::process::exit(1);
    }

    let toolchain_output_dir = manifests_output_dir.join("toolchains");
    if !toolchain_output_dir.exists() {
        fs::create_dir_all(&toolchain_output_dir)
            .expect("Failed to create toolchain output directory");
    }

    println!("Generating manifests of toolchains");

    // Call the toolchain generate function to handle processing.
    process_toolchain_manifests(&toolchain_input_dir, &toolchain_output_dir, &args).await;

    println!("Done!");
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
async fn process_toolchain_manifests(input_path: &Path, output_path: &Path, args: &Arguments) {
    // Read the index.toml file and convert it into an IndexManifest struct.
    let index_input_path = input_path.join("index.toml");
    let manifest = IndexManifest::read(&index_input_path)
        .unwrap_or_else(|_| panic!("Failed to parse TOML at {}", index_input_path.display()));

    // For each toolchain entry in the IndexManifest, read the corresponding
    // toolchain file, parse it into a ToolchainManifest struct, and write
    // the serialized struct to the output path.
    for (_, path) in manifest.iter() {
        process_toolchain_manifest(&input_path.join(path), &output_path.join(path), args).await;
    }

    // Copy the index.toml file to the output directory.
    let index_output_path = output_path.join("index.toml");
    process_index_manifest(&index_input_path, &index_output_path);
}

/// Process the toolchain manifest
async fn process_toolchain_manifest(input_path: &Path, output_path: &Path, args: &Arguments) {
    let manifest = ToolchainManifest::read(input_path)
        .unwrap_or_else(|_| panic!("Failed to parse TOML at {}", input_path.display()));

    // For each toolchain entry in the ToolchainManifest, convert it into
    // ReleaseToolchain struct if it is a PackageToolchain.
    let mut result = ToolchainManifest::new();

    for (category, tools) in manifest.iter() {
        for (name, toolchain) in tools {
            if let Toolchain::Package(package) = toolchain {
                let release = build_release_toolchain(package, args).await;

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
async fn build_release_toolchain(pkg: &PackageToolchain, args: &Arguments) -> ReleaseToolchain {
    let mut targets = HashMap::new();
    let output_dir = args.output_dir();
    let version = args.version();
    let bin_name = pkg.name();

    for target in &pkg.targets {
        // Skip the target if it is not the same as the current target.
        if target.ne(target_triple::TARGET) {
            eprintln!("Skipping target: {} of package: {}", target, bin_name);
            continue;
        }

        let archive_name = format!("{}-{}-{}.tar.gz", bin_name, version, target);

        let archive_path = output_dir.join(&archive_name);
        if !archive_path.exists() {
            panic!("Archive not found: {}", archive_path.display());
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

        let checksum_path = output_dir.join(format!("{}.sha256", archive_name));
        let hash = fs::read_to_string(&checksum_path).unwrap_or_else(|_| {
            panic!("Failed to read SHA256 from file: {}", checksum_path.display())
        });

        targets.insert(target.to_string(), TargetInfo::new(url, hash));
    }

    ReleaseToolchain::new(version, targets)
}
