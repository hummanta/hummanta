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

use hmt_manifest::IndexManifest;

const TOOLCHAINS_INDEX_TOML: &str = "solidity = \"solidity.toml\"\n";

#[test]
fn test_index_manifest_to_toml() {
    let mut manifest = IndexManifest::new();
    manifest.insert("solidity".to_string(), "solidity.toml".into());

    let toml_string = toml::to_string(&manifest).expect("Failed to serialize to TOML");

    assert_eq!(toml_string, TOOLCHAINS_INDEX_TOML);
}

#[test]
fn test_toml_to_index_manifest() {
    let manifest: IndexManifest =
        toml::from_str(TOOLCHAINS_INDEX_TOML).expect("Failed to parse TOML");

    assert_eq!(manifest.get("solidity"), Some(&"solidity.toml".into()));
}
