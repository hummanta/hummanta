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

pub type ManifestResult<T> = std::result::Result<T, ManifestError>;

#[derive(Debug, Error)]
pub enum ManifestError {
    #[error("Failed to deserialize the manifest: {0}")]
    DeserializeError(#[from] toml::de::Error),

    #[error("Failed to serialize the manifest: {0}")]
    SerializeError(#[from] toml::ser::Error),

    #[error("Manifest file not found at path: {0}")]
    FileNotFound(String),

    #[error("Invalid manifest format: {0}")]
    InvalidFormat(String),

    #[error("IO error occurred: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Unknown error: {0}")]
    Unknown(String),
}
