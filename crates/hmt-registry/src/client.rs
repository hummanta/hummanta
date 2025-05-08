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

use hmt_fetcher::{FetchContext, Fetcher};
use hmt_manifest::IndexManifest;
use hmt_utils::bytes::FromSlice;

use crate::error::{RegistryError, Result};

/// A client for interacting with Hummanta Registry.
pub struct RegistryClient {
    fetcher: Fetcher,
    base_url: String,
}

impl RegistryClient {
    /// Creates a new instance.
    pub fn new(url: &str) -> Self {
        Self { fetcher: Fetcher::default(), base_url: url.trim_end_matches('/').to_string() }
    }

    #[inline]
    /// Fetches data from the registry using a rewritten fetch context.
    pub async fn fetch(&self, context: &FetchContext) -> Result<Vec<u8>> {
        self.fetcher.fetch(&self.rewrite_context(context)).await.map_err(RegistryError::from)
    }

    /// Fetches and parses the index manifest from the registry.
    pub async fn index(&self) -> Result<IndexManifest> {
        let context = FetchContext::new("index.toml");
        let bytes = self.fetch(&context).await?;
        let manifest = IndexManifest::from_slice(&bytes)?;

        Ok(manifest)
    }

    /// Resolves the full URL by combining the base URL with the relative path
    /// from the context. If the context URL is already absolute, it is used directly.
    fn rewrite_context(&self, context: &FetchContext) -> FetchContext {
        let absolute_url = if context.url.contains("://") {
            context.url.clone()
        } else {
            format!("{}/{}", self.base_url, context.url)
        };

        FetchContext {
            url: absolute_url,
            checksum: context.checksum.clone(),
            checksum_url: context.checksum_url.clone(),
        }
    }
}
