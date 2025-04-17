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

use hmt_fetcher::errors::FetchError;
use hmt_manifest::ManifestError;
use std::io;
use thiserror::Error;

pub type Result<T> = std::result::Result<T, RegistryError>;

#[derive(Error, Debug)]
pub enum RegistryError {
    #[error("Fetch error: {0}")]
    FetchError(#[from] FetchError),

    #[error("I/O error: {0}")]
    IoError(#[from] io::Error),

    #[error("toml parse error: {0}")]
    TomlError(#[from] toml::de::Error),

    #[error("manifest not found: {0}")]
    ManifestNotFound(String),

    #[error("invalid registry path: {0}")]
    InvalidPath(String),

    #[error("unsupported registry protocol: {0}")]
    UnsupportedProtocol(String),

    #[error("package not found: {0}")]
    PackageNotFound(String),

    #[error("release version not found: {0} v{1}")]
    ReleaseNotFound(String, String),

    #[error("Manifest error: {0}")]
    ManifestError(#[from] ManifestError),

    #[error("Failed to unpack archive: {0}")]
    UnpackError(String),

    #[error("Failed to remove installation directory for '{0}")]
    RemoveError(String),

    #[error("other error: {0}")]
    Other(String),
}
