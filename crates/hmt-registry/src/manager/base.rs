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

#![allow(unused)]

use std::{marker::PhantomData, str::FromStr};

use hmt_fetcher::FetchContext;
use hmt_manifest::{IndexManifest, PackageManifest, ReleaseManifest};
use hmt_utils::bytes::FromSlice;

use crate::{
    error::{RegistryError, Result},
    traits::{LocalStatus, PackageKind, PackageManager, RemoteMetadata},
    RegistryClient,
};

pub struct Manager<T: PackageKind> {
    registry: RegistryClient,
    kind: PhantomData<T>,
}

impl<T: PackageKind> Manager<T> {
    pub fn new(registry: RegistryClient) -> Self {
        Self { registry, kind: PhantomData }
    }
}

impl<T: PackageKind> PackageManager for Manager<T> {
    fn add(&self, name: &str, version: Option<&str>) -> Result<()> {
        todo!()
    }

    fn remove(&self, name: &str, version: Option<&str>) -> Result<()> {
        todo!()
    }

    fn list(&self) -> Result<Vec<PackageManifest>> {
        todo!()
    }
}

impl<T: PackageKind> RemoteMetadata for Manager<T> {
    /// Fetches the index manifest for the given package name.
    /// eg. https://hummanta.github.io/registry/toolchains/solidity.toml
    async fn fetch_index(&self, name: &str) -> Result<IndexManifest> {
        let index = self.registry.index().await?;

        let path = index
            .get(T::kind(), name)
            .ok_or_else(|| RegistryError::PackageNotFound(name.to_string()))?;

        let context = FetchContext::new(path);
        let bytes = self.registry.fetch(&context).await?;
        let manifest = IndexManifest::from_slice(&bytes)?;

        Ok(manifest)
    }

    /// Fetches the package manifest for the given category and package name.
    /// eg. https://hummanta.github.io/solidity-detector-foundry/manifests/index.toml
    async fn fetch_package(&self, category: &str, name: &str) -> Result<PackageManifest> {
        let index = self.fetch_index(name).await?;

        let registry = index
            .get(category, name)
            .ok_or_else(|| RegistryError::PackageNotFound(name.to_string()))?
            .trim_end_matches('/');
        let url = format!("{registry}/manifests/index.toml");

        let context = FetchContext::new(&url);
        let bytes = self.registry.fetch(&context).await?;
        let manifest = PackageManifest::from_slice(&bytes)?;

        Ok(manifest)
    }

    /// Fetches the release manifest for the specified category, name and version.
    /// eg. https://hummanta.github.io/solidity-detector-foundry/manifests/release-v1.0.0.toml
    async fn fetch_release(
        &self,
        category: &str,
        name: &str,
        version: &str,
    ) -> Result<ReleaseManifest> {
        let package = self.fetch_package(category, name).await?;

        let path = package
            .get_releases()
            .get(version)
            .ok_or_else(|| RegistryError::ReleaseNotFound(name.to_string(), version.to_string()))?;
        let url = format!("{}/{}", package.package.homepage.trim_end_matches('/'), path);

        let context = FetchContext::new(&url);
        let bytes = self.registry.fetch(&context).await?;
        let manifest = ReleaseManifest::from_slice(&bytes)?;

        Ok(manifest)
    }
}

impl<T: PackageKind> LocalStatus for Manager<T> {
    fn is_installed(&self, name: &str, version: &str) -> bool {
        todo!()
    }

    fn local_versions(&self, name: &str) -> Result<Vec<String>> {
        todo!()
    }
}
