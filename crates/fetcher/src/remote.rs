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
use reqwest::Client;

use crate::{
    checksum::verify,
    context::FetchContext,
    errors::{FetchError, FetchResult},
    traits::Fetcher,
};

/// Fetcher implementation for HTTP/HTTPS resources
pub struct RemoteFetcher {
    client: Client,
}

impl RemoteFetcher {
    /// Creates a new RemoteFetcher with default client
    pub fn new() -> Self {
        Self { client: Client::new() }
    }

    pub async fn get(&self, url: &str) -> FetchResult<Vec<u8>> {
        let response = self.client.get(url).send().await?;

        if !response.status().is_success() {
            return Err(FetchError::NetworkError(response.error_for_status().unwrap_err()));
        }

        Ok(response.bytes().await?.to_vec())
    }
}

#[async_trait]
impl Fetcher for RemoteFetcher {
    async fn fetch(&self, context: &FetchContext) -> FetchResult<Vec<u8>> {
        // Download main content
        let data = self.get(&context.url).await?;

        // Resolve checksum and verify checksum if provided
        if let Some(checksum) = match &context.checksum_url {
            Some(url) => Some(self.get(url).await?),
            None => context.checksum.as_ref().map(|s| s.as_bytes().to_vec()),
        } {
            verify(&data, std::str::from_utf8(&checksum).unwrap())?;
        }

        Ok(data)
    }

    fn supported_schemes(&self) -> Vec<&'static str> {
        vec!["http", "https"]
    }
}

impl Default for RemoteFetcher {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;
    use tokio::{
        io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
        net::TcpListener,
    };

    use super::*;

    async fn start_mock_server() -> String {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();
        let url = format!("http://{}", addr);

        tokio::spawn(async move {
            let (mut socket, _) = listener.accept().await.unwrap();
            let mut reader = BufReader::new(&mut socket);
            let mut request = String::new();
            reader.read_line(&mut request).await.unwrap();

            let response = "HTTP/1.1 200 OK\r\n\
                          Content-Length: 9\r\n\
                          \r\n\
                          test data";
            socket.write_all(response.as_bytes()).await.unwrap();
        });

        url
    }

    #[tokio::test]
    async fn test_remote_fetcher_success() {
        let url = start_mock_server().await;
        let context = FetchContext::new(&url)
            .checksum("916f0027a575074ce72a331777c3478d6513f786a591bd892da1a577bf2335f9");

        let fetcher = Arc::new(RemoteFetcher::new());
        let result = fetcher.fetch(&context).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_remote_fetcher_network_error() {
        let context = FetchContext::new("http://invalid-url").checksum("dummy_hash");

        let fetcher = Arc::new(RemoteFetcher::new());
        let result = fetcher.fetch(&context).await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_remote_fetcher_hash_mismatch() {
        let url = start_mock_server().await;
        let context = FetchContext::new(&url).checksum("incorrect_hash");

        let fetcher = Arc::new(RemoteFetcher::new());
        let result = fetcher.fetch(&context).await;

        assert!(result.is_err());

        if let Err(FetchError::HashMismatch { expected, actual: _ }) = result {
            assert_eq!(expected, "incorrect_hash");
        }
    }
}
