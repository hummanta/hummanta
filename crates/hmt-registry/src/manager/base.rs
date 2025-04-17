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

use std::{
    marker::PhantomData,
    path::{Path, PathBuf},
    str::FromStr,
};

use hmt_fetcher::FetchContext;
use hmt_manifest::{
    Entry, IndexManifest, InstalledManifest, ManifestFile, PackageManifest, ReleaseManifest,
};
use hmt_utils::{archive, bytes::FromSlice};

use crate::{
    error::{RegistryError, Result},
    traits::{LocalStatus, PackageKind, PackageManager, RemoteMetadata},
    RegistryClient,
};

/// A generic manager for handling package operations,
/// with a registry client, cache, and installation root.
pub struct Manager<T: PackageKind> {
    /// The registry client used for interacting with the registry.
    registry: RegistryClient,
    /// The cache of installed manifests.
    cache: InstalledManifest,
    /// The root path where packages are installed.
    install_root: PathBuf,
    /// A marker type used to specify the package kind.
    _marker: PhantomData<T>,
}

impl<T: PackageKind> Manager<T> {
    /// Creates a new package manager with the given registry client
    /// and install root, loading or initializing the cache.
    pub fn new(registry: RegistryClient, install_root: PathBuf) -> Self {
        let path = install_root.join("installed.toml");
        let cache = match InstalledManifest::load(path) {
            Ok(manifest) => manifest,
            Err(_) => InstalledManifest::new(),
        };

        Self { registry, cache, install_root, _marker: PhantomData }
    }

    /// Returns the installation path for a package with the given domain.
    fn install_path(&self, domain: &str) -> PathBuf {
        self.install_root.join(T::kind()).join(domain)
    }

    /// Returns the path to the installed manifest cache file.
    fn cache_path(&self) -> PathBuf {
        self.install_root.join("installed.toml")
    }
}

impl<T: PackageKind> PackageManager for Manager<T> {
    /// Add a package to the system and update the cache.
    async fn add(&mut self, domain: &str) -> Result<()> {
        let index = self.fetch_index(domain).await?;
        let install_path = self.install_path(domain);

        // Iterate over the index entries to fetch and install packages
        for (category, name) in index.entries() {
            let package = self.fetch_package(category, name).await?;
            let version = package.latest.clone();
            let release = self.fetch_release(category, name, &version).await?;

            // Skip the package if it doesn't support the current target platform
            if !release.supports_target(target_triple::TARGET) {
                eprintln!("{name} does not support current target platform, skipping.");
                continue;
            }

            // Get the appropriate artifact for the target platform
            let artifact = release
                .get_artifact(target_triple::TARGET)
                .expect("Artifact should exist if platform is supported");

            // Fetch and verify the checksum
            let context = FetchContext::new(&artifact.url).checksum(&artifact.hash);
            let data = self.registry.fetch(&context).await?;

            // Unpack the file and extract its contents to the target directory
            archive::unpack(&data, &install_path).map_err(|e| {
                eprintln!("ERROR: {}", e);
                RegistryError::UnpackError(name.to_string())
            })?;

            // Now, update cache to reflect the new installation
            let entry = Entry::new(version, package.package.description.clone());
            self.cache.insert(T::kind(), domain, category, name, entry);
            self.cache.save(self.cache_path())?;
        }

        Ok(())
    }

    fn remove(&mut self, domain: &str) -> Result<()> {
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
        let url = format!("{}/manifests/{}", package.package.homepage.trim_end_matches('/'), path);

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
