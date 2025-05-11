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
    ffi::{OsStr, OsString},
    path::{Path, PathBuf},
    process::Output,
};

use anyhow::{anyhow, Context as _};
use tokio::process::Command;

use hmt_manifest::CategoryMap;
use tracing::debug;

use crate::errors::Result;

pub fn confirm(prompt: &str) -> Result<bool> {
    println!("{prompt}");

    let mut input = String::new();
    std::io::stdin().read_line(&mut input)?;

    Ok(input.trim().eq_ignore_ascii_case("y"))
}

pub fn print_domain_packages(domain: &str, categories: &CategoryMap) {
    println!("{domain}");
    for packages in categories.values() {
        for (name, entry) in packages {
            println!("  {name} {}", entry.version);
            if let Some(desc) = &entry.description {
                println!("  {desc}");
            }
        }
    }
}

/// Executes a system command asynchronously and returns its complete output
pub async fn command<S, I, T>(program: S, args: I) -> Result<Output>
where
    S: AsRef<OsStr>,
    I: IntoIterator<Item = T>,
    T: AsRef<OsStr>,
{
    // Convert arguments to OsString for display purposes
    let args_vec: Vec<OsString> = args.into_iter().map(|a| a.as_ref().to_os_string()).collect();
    let prog = program.as_ref().to_string_lossy();
    let args_str = args_vec.iter().map(|a| a.to_string_lossy()).collect::<Vec<_>>().join(" ");
    debug!("Executing {prog} {args_str}");

    Command::new(program.as_ref()).args(&args_vec).output().await.context("Command execute failed!")
}

/// Searches for `filename` in current directory
/// and parent directories until found or root is reached.
pub fn find<P: AsRef<Path>>(filename: P) -> Result<PathBuf> {
    let mut current_dir = std::env::current_dir()?;

    loop {
        let candidate = current_dir.join(&filename);

        // Check if the candidate path exists and is a file.
        // Path::is_file() handles existence checks and file type internally.
        if candidate.is_file() {
            return Ok(candidate);
        }

        // Try to move to the parent directory.
        // PathBuf::pop() returns false if current_dir is already a root,
        // or an empty path, or has no parent component.
        if !current_dir.pop() {
            // No more parent directories to check.
            break;
        }
    }

    // If the loop finishes, the file was not found in the hierarchy.
    Err(anyhow!("Not found {}", filename.as_ref().display()))
}
