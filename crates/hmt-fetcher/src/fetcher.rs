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

use std::{collections::HashMap, sync::Arc};

use crate::{
    context::FetchContext,
    errors::{FetchError, FetchResult},
    local::LocalFetcher,
    remote::RemoteFetcher,
    traits,
};

/// Manages multiple fetchers and routes requests based on URL scheme
pub struct Fetcher {
    fetchers: HashMap<String, Arc<dyn traits::Fetcher + Send + Sync>>,
}

impl Fetcher {
    /// Creates a new instance with default fetchers registered
    pub fn new() -> Self {
        Self { fetchers: HashMap::new() }
    }

    /// Registers a new fetcher implementation
    pub fn register(&mut self, fetcher: Arc<dyn traits::Fetcher + Send + Sync>) {
        for scheme in fetcher.supported_schemes() {
            self.fetchers.insert(scheme.to_string(), fetcher.clone());
        }
    }

    /// Fetches content from any supported source
    pub async fn fetch(&self, context: &FetchContext) -> FetchResult<Vec<u8>> {
        let scheme = self.scheme(&context.url)?;

        let fetcher =
            self.fetchers.get(&scheme).ok_or_else(|| FetchError::UnsupportedScheme(scheme))?;

        fetcher.fetch(context).await
    }

    /// Parse url and return scheme
    fn scheme(&self, url: &str) -> FetchResult<String> {
        url.split("://")
            .next()
            .map(|s| s.to_string())
            .ok_or_else(|| FetchError::InvalidUrl(url.to_string()))
    }
}

impl Default for Fetcher {
    /// Holds the default fetcher instance.
    fn default() -> Self {
        let mut fetcher = Self::new();

        // Register default fetchers
        fetcher.register(Arc::new(RemoteFetcher::new()));
        fetcher.register(Arc::new(LocalFetcher));

        fetcher
    }
}

mod tests {
    use async_trait::async_trait;

    use super::*;

    #[allow(dead_code)]
    struct MockFetcher {
        schemes: Vec<&'static str>,
    }

    #[async_trait]
    impl traits::Fetcher for MockFetcher {
        fn supported_schemes(&self) -> Vec<&'static str> {
            self.schemes.to_vec()
        }

        async fn fetch(&self, _: &FetchContext) -> FetchResult<Vec<u8>> {
            Ok(vec![1, 2, 3, 4]) // Mocked data
        }
    }

    #[tokio::test]
    async fn test_fetcher_register_and_fetch() {
        let mut fetcher = Fetcher::new();
        fetcher.register(Arc::new(MockFetcher { schemes: vec!["http"] }));

        let context = FetchContext::new("http://example.com").checksum("dummy_hash");
        let result = fetcher.fetch(&context).await;

        assert!(result.is_ok());
        assert_eq!(result.unwrap(), vec![1, 2, 3, 4]);
    }

    #[tokio::test]
    async fn test_fetcher_invalid_url() {
        let fetcher = Fetcher::new();

        let context = FetchContext::new("invalid_url").checksum("dummy_hash");
        let result = fetcher.fetch(&context).await;

        assert!(result.is_err());
        if let Err(FetchError::InvalidUrl(url)) = result {
            assert_eq!(url, "invalid_url");
        }
    }

    #[tokio::test]
    async fn test_fetcher_unsupported_scheme() {
        let fetcher = Fetcher::new();

        let context = FetchContext::new("ftp://example.com").checksum("dummy_hash");
        let result = fetcher.fetch(&context).await;

        assert!(result.is_err());
        if let Err(FetchError::UnsupportedScheme(scheme)) = result {
            assert_eq!(scheme, "ftp");
        }
    }
}
