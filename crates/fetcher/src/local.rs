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

use crate::{checksum, errors::FetchResult, Fetcher};

/// Fetcher implementation for local file system
pub struct LocalFetcher;

#[async_trait]
impl Fetcher for LocalFetcher {
    async fn fetch(&self, url: &str, hash: &str) -> FetchResult<Vec<u8>> {
        // Remove "file://" prefix if present
        let path = url.trim_start_matches("file://");
        let data = tokio::fs::read(path).await?;
        checksum::verify(&data, hash)?;

        Ok(data)
    }

    fn supported_schemes(&self) -> Vec<&'static str> {
        vec!["file"]
    }
}

#[cfg(test)]
mod tests {
    use tempfile::NamedTempFile;
    use tokio::fs::write;

    use super::*;
    use crate::errors::FetchError;

    #[tokio::test]
    async fn test_local_fetcher_success() {
        let temp_file = NamedTempFile::new().unwrap();
        let dummy_data = b"test data";
        write(temp_file.path(), dummy_data).await.unwrap();

        let fetcher = LocalFetcher;
        let result = fetcher
            .fetch(
                &format!("file://{}", temp_file.path().display()),
                "916f0027a575074ce72a331777c3478d6513f786a591bd892da1a577bf2335f9",
            )
            .await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_local_fetcher_hash_mismatch() {
        let fetcher = LocalFetcher;
        let result = fetcher.fetch("file://dummy_path", "incorrect_hash").await;

        assert!(result.is_err());

        if let Err(FetchError::HashMismatch { expected, actual: _ }) = result {
            assert_eq!(expected, "incorrect_hash");
        }
    }
}
