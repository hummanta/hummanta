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

use hmt_manifest::PackageManifest;

use crate::error::Result;

/// A trait for managing package operations,
/// including adding, removing, and listing package manifests.
pub trait PackageManager {
    /// Adds a package identified by the given domain.
    fn add(&mut self, domain: &str) -> impl Future<Output = Result<()>>;

    /// Removes a package identified by the given domain.
    fn remove(&mut self, domain: &str) -> Result<()>;

    /// Lists all packages managed by the package manager.
    fn list(&self) -> Result<Vec<PackageManifest>>;
}
