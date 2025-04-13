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

use std::str::FromStr;

use hmt_manifest::{PackageIndexRef, ToolchainManifest};

const TEST_TOOLCHAIN_MANIFEST: &str = r#"
    [detector.solidity-detector-foundry]
    package-index = "https://hummanta.github.io/solidity-detector-foundry/package-index.toml"
"#;

fn normalize_toml(toml_str: &str) -> String {
    toml::Value::from_str(toml_str).unwrap().to_string()
}

#[test]
fn test_toolchain_manifest_to_toml() {
    let mut manifest = ToolchainManifest::new();
    let index_ref = PackageIndexRef::new(
        "https://hummanta.github.io/solidity-detector-foundry/package-index.toml".to_string(),
    );
    manifest.insert("detector".to_string(), "solidity-detector-foundry".to_string(), index_ref);

    let toml_str = toml::to_string(&manifest).expect("Failed to convert to TOML");

    assert_eq!(normalize_toml(&toml_str), normalize_toml(TEST_TOOLCHAIN_MANIFEST));
}

#[test]
fn test_toml_to_toolchain_manifest() {
    let toml_str = r#"
        [detector.solidity-detector-foundry]
        package-index = "https://hummanta.github.io/solidity-detector-foundry/package-index.toml"
    "#;

    let manifest: ToolchainManifest =
        toml::from_str(toml_str).expect("Failed to convert from TOML");

    // 断言 ToolchainManifest 中的值
    let expected_index_ref = PackageIndexRef::new(
        "https://hummanta.github.io/solidity-detector-foundry/package-index.toml".to_string(),
    );
    assert_eq!(manifest.get("detector", "solidity-detector-foundry").unwrap(), &expected_index_ref);
}
