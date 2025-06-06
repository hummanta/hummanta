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

use async_trait::async_trait;

use crate::{context::FetchContext, errors::FetchResult};

/// Defines the common interface for all fetchers
#[async_trait]
pub trait Fetcher {
    /// Fetches content from source and verifies its hash
    async fn fetch(&self, context: &FetchContext) -> FetchResult<Vec<u8>>;

    /// Returns supported URL schemes (e.g., ["http", "https"])
    fn supported_schemes(&self) -> Vec<&'static str>;
}
