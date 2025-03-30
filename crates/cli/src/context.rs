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

use std::path::PathBuf;

/// Holds the state of the application.
#[derive(Default)]
pub struct Context {}

impl Context {
    /// Gets the current application version.
    pub fn version(&self) -> String {
        format!("v{}", env!("CARGO_PKG_VERSION"))
    }

    /// Gets the path to the Hummanta home directory.
    pub fn home_dir(&self) -> Option<PathBuf> {
        dirs::home_dir().map(|dir| dir.join(".hummanta"))
    }

    /// Gets the path to the toolchains directory.
    pub fn toolchains_dir(&self) -> Option<PathBuf> {
        self.home_dir().map(|dir| dir.join("toolchains"))
    }

    /// Gets the path to the manifests directory.
    pub fn manifests_dir(&self) -> Option<PathBuf> {
        self.home_dir().map(|dir| dir.join("manifests"))
    }
}
