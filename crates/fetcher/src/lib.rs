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

pub mod checksum;
pub mod errors;
pub mod factory;
pub mod local;
pub mod remote;

use std::sync::Arc;

use async_trait::async_trait;
use factory::FetcherFactory;
use local::LocalFetcher;
use once_cell::sync::Lazy;
use remote::RemoteFetcher;

use self::errors::FetchResult;

/// Global default fetcher factory instance with basic fetchers pre-registered
///
/// Note: You can create your own instance with `FetcherFactory::new()`
/// if you need custom configuration
pub static FETCHER_FACTORY: Lazy<FetcherFactory> = Lazy::new(|| {
    let mut factory = FetcherFactory::new();

    // Register default fetchers
    factory.register(Arc::new(LocalFetcher));
    factory.register(Arc::new(RemoteFetcher::new()));

    factory
});

/// Defines the common interface for all fetchers
#[async_trait]
pub trait Fetcher {
    /// Fetches content from source and verifies its hash
    async fn fetch(&self, url: &str, hash: &str) -> FetchResult<Vec<u8>>;

    /// Returns supported URL schemes (e.g., ["http", "https"])
    fn supported_schemes(&self) -> Vec<&'static str>;
}
