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

/// FetchContext is used to store context information related to fetch
/// operations, including the URL, checksum, and its corresponding checksum URL.
pub struct FetchContext {
    /// The URL to fetch data from.
    pub url: String,
    /// The optional checksum for verifying the integrity of the fetched data.
    pub checksum: Option<String>,
    /// The optional URL where the checksum can be fetched from.
    pub checksum_url: Option<String>,
}

impl FetchContext {
    /// Creates new instance with the specified URL.
    pub fn new(url: &str) -> Self {
        Self { url: url.to_string(), checksum: None, checksum_url: None }
    }

    /// Sets the checksum.
    pub fn checksum(mut self, checksum: &str) -> Self {
        self.checksum = Some(checksum.to_string());
        self
    }

    /// Sets the checksum url.
    pub fn checksum_url(mut self, checksum_url: &str) -> Self {
        self.checksum_url = Some(checksum_url.to_string());
        self
    }
}
