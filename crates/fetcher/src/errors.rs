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

use thiserror::Error;

/// Result type alias for fetcher operations
pub type FetchResult<T> = Result<T, FetchError>;

/// Unified error type for all fetcher operations
#[derive(Error, Debug)]
pub enum FetchError {
    #[error("Invalid URL: {0}")]
    InvalidUrl(String),

    #[error("File operation failed: {0}")]
    FileError(#[from] std::io::Error),

    #[error("Network request failed: {0}")]
    NetworkError(#[from] reqwest::Error),

    #[error("Hash mismatch - expected: {expected}, actual: {actual}")]
    HashMismatch { expected: String, actual: String },

    #[error("Unsupported scheme: {0}")]
    UnsupportedScheme(String),
}
