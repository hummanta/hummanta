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

use std::{io::Cursor, sync::Arc};

use anyhow::Context as _;
use clap::Args;
use flate2::read::GzDecoder;
use tar::Archive;
use tokio::fs;

use hummanta_fetcher::{FetchContext, DEFAULT_FETCHER};

use crate::{context::Context, errors::Result};

const HUMMANTA_GITHUB_REPO: &str = "github.com/hummanta/hummanta";
const MANIFEST_ARCHIVE_NAME: &str = "manifests";

/// Add a specific Hummanta version
#[derive(Args, Debug)]
pub struct Command {
    /// The version to add
    version: String,
}

impl Command {
    pub async fn exec(&self, ctx: Arc<Context>) -> Result<()> {
        let version = &self.version;

        // Create target directory
        let manifests_dir = ctx.manifests_dir().join(version);
        fs::create_dir_all(&manifests_dir).await.context("Failed to create manifest directory")?;

        let archive_url = format!(
            "https://{}/releases/download/{}/{}-{}.tar.gz",
            HUMMANTA_GITHUB_REPO, version, MANIFEST_ARCHIVE_NAME, version
        );

        // @TODO: Currently, the manifests does not support checksum,
        // so we will skip this for now.
        let context = FetchContext::new(&archive_url);

        // Fetch and verify the checksum
        let data = DEFAULT_FETCHER.fetch(&context).await?;

        // Unpack the file and extract its contents to the target directory
        let buffer = Cursor::new(data);
        let decoder = GzDecoder::new(buffer);
        let mut archive = Archive::new(decoder);
        archive.unpack(manifests_dir).context("Failed to unpack tar.gz file")?;

        println!("Successfully added and verified version {}", version);
        Ok(())
    }
}
