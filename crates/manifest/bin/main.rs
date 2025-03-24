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

mod args;
mod index;
mod toolchain;

use clap::Parser;
use std::fs;

const HUMMANTA_GITHUB_REPO: &str = "github.com/hummanta/hummanta";

#[tokio::main]
async fn main() {
    let args = args::Arguments::parse();

    // Prepare the input directory
    let input_path = args.path.to_path_buf();
    if !input_path.exists() {
        eprintln!("Error: input directory {:?} does not exist.", input_path);
        std::process::exit(1);
    }

    // Prepare the artifacts directory
    let artifact_path = args.artifact_dir();
    if !artifact_path.exists() {
        eprintln!("Error: artifacts directory {:?} does not exist.", artifact_path);
        std::process::exit(1);
    }

    // Prepare the manifest output directory
    let output_path = artifact_path.join("manifests");
    if !output_path.exists() {
        fs::create_dir_all(&output_path).expect("Failed to create output directory for manifests");
    }

    // Call the toolchain generate function to handle processing.
    println!("Generating manifests of toolchains");
    toolchain::generate(&input_path, &artifact_path, &output_path, &args).await;

    // Archive all the manifests

    println!("Done!");
}
