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

use std::marker::PhantomData;

use hmt_manifest::{IndexManifest, PackageManifest, ReleaseManifest};

use crate::{
    error::Result,
    traits::{LocalStatus, PackageKind, PackageManager, RemoteMetadata},
    RegistryClient,
};

pub struct Manager<T: PackageKind> {
    client: RegistryClient,
    kind: PhantomData<T>,
}

impl<T: PackageKind> Manager<T> {
    pub fn new(client: RegistryClient) -> Self {
        Self { client, kind: PhantomData }
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
    fn fetch_index(&self) -> Result<IndexManifest> {
        todo!()
    }

    fn fetch_package(&self, name: &str) -> Result<PackageManifest> {
        todo!()
    }

    fn fetch_release(&self, name: &str, version: &str) -> Result<ReleaseManifest> {
        todo!()
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
