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

use crate::{error::Result, traits::Client};

pub struct FileRegistryClient {
    base_path: String,
}

impl FileRegistryClient {
    pub fn new(base_path: &str) -> Self {
        Self { base_path: base_path.to_string() }
    }
}

impl Client for FileRegistryClient {
    fn fetch(&self, _path: &str) -> Result<Vec<u8>> {
        todo!()
    }
}
