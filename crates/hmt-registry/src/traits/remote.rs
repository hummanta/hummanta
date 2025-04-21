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

use std::future::Future;

use hmt_manifest::{IndexManifest, PackageManifest, ReleaseManifest};

use crate::error::Result;

/// Fetches index, package, and release metadata from a remote registry.
pub trait RemoteMetadata {
    /// Fetches the index manifest for the given domain.
    fn fetch_index(&self, domain: &str) -> impl Future<Output = Result<IndexManifest>>;

    /// Fetches the package manifest for the given category and package name.
    fn fetch_package(
        &self,
        index: &IndexManifest,
        category: &str,
        name: &str,
    ) -> impl Future<Output = Result<PackageManifest>>;

    /// Fetches the release manifest for the specified version.
    fn fetch_release(
        &self,
        package: &PackageManifest,
        version: &str,
    ) -> impl Future<Output = Result<ReleaseManifest>>;
}
