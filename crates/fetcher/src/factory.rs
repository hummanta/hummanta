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
    errors::{FetchError, FetchResult},
    Fetcher,
};

/// Manages multiple fetchers and routes requests based on URL scheme
pub struct FetcherFactory {
    fetchers: HashMap<String, Arc<dyn Fetcher + Send + Sync>>,
}

impl FetcherFactory {
    /// Creates a new factory with default fetchers registered
    pub fn new() -> Self {
        Self { fetchers: HashMap::new() }
    }

    /// Registers a new fetcher implementation
    pub fn register(&mut self, fetcher: Arc<dyn Fetcher + Send + Sync>) {
        for scheme in fetcher.supported_schemes() {
            self.fetchers.insert(scheme.to_string(), fetcher.clone());
        }
    }

    /// Fetches content from any supported source
    pub async fn fetch(&self, url: &str, hash: &str) -> FetchResult<Vec<u8>> {
        let scheme =
            url.split("://").next().ok_or_else(|| FetchError::InvalidUrl(url.to_string()))?;

        let fetcher = self
            .fetchers
            .get(scheme)
            .ok_or_else(|| FetchError::UnsupportedScheme(scheme.to_string()))?;

        fetcher.fetch(url, hash).await
    }
}

impl Default for FetcherFactory {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use async_trait::async_trait;

    use super::*;
    use std::sync::Arc;

    struct MockFetcher {
        schemes: Vec<&'static str>,
    }

    #[async_trait]
    impl Fetcher for MockFetcher {
        fn supported_schemes(&self) -> Vec<&'static str> {
            self.schemes.to_vec()
        }

        async fn fetch(&self, _url: &str, _hash: &str) -> FetchResult<Vec<u8>> {
            Ok(vec![1, 2, 3, 4]) // Mocked data
        }
    }

    #[tokio::test]
    async fn test_fetcher_factory_register_and_fetch() {
        let mut factory = FetcherFactory::new();
        let fetcher = Arc::new(MockFetcher { schemes: vec!["http"] });
        factory.register(fetcher.clone());

        let result = factory.fetch("http://example.com", "dummy_hash").await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), vec![1, 2, 3, 4]);
    }

    #[tokio::test]
    async fn test_fetcher_factory_invalid_url() {
        let factory = FetcherFactory::new();
        let result = factory.fetch("invalid_url", "dummy_hash").await;
        assert!(result.is_err());
        if let Err(FetchError::InvalidUrl(url)) = result {
            assert_eq!(url, "invalid_url");
        }
    }

    #[tokio::test]
    async fn test_fetcher_factory_unsupported_scheme() {
        let factory = FetcherFactory::new();
        let result = factory.fetch("ftp://example.com", "dummy_hash").await;
        assert!(result.is_err());
        if let Err(FetchError::UnsupportedScheme(scheme)) = result {
            assert_eq!(scheme, "ftp");
        }
    }
}
