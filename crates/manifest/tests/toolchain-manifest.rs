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

use hummanta_manifest::{
    PackageToolchain, ReleaseToolchain, TargetInfo, Toolchain, ToolchainManifest,
};
use std::collections::HashMap;

const TOOLCHAIN_TOML: &str = r#"
[[detector]]
name = "detector1"
package = "package1"
bin = "bin1"

[[compiler]]
name = "compiler1"
version = "1.0.0"

[compiler.target.x86_64]
url = "http://example.com"
hash = "hash123"
"#;

#[test]
fn test_toolchain_manifest_to_toml() {
    let mut manifest = ToolchainManifest::new();

    let detector = Toolchain::Package(PackageToolchain::new(
        "detector1".to_string(),
        "package1".to_string(),
        Some("bin1".to_string()),
    ));
    manifest.add_detector(detector);

    let mut targets = HashMap::new();
    targets.insert(
        "x86_64".to_string(),
        TargetInfo::new("http://example.com".to_string(), "hash123".to_string()),
    );
    let compiler = Toolchain::Release(ReleaseToolchain::new(
        "compiler1".to_string(),
        "1.0.0".to_string(),
        targets,
    ));
    manifest.add_compiler(compiler);

    let toml_string = toml::to_string(&manifest).expect("Failed to serialize to TOML");

    assert_eq!(toml_string.trim(), TOOLCHAIN_TOML.trim());
}

#[test]
fn test_toml_to_toolchain_manifest() {
    let manifest: ToolchainManifest = toml::from_str(TOOLCHAIN_TOML).expect("Failed to parse TOML");

    assert_eq!(manifest.detector.len(), 1);
    assert_eq!(manifest.detector[0].name(), "detector1");

    assert_eq!(manifest.compiler.len(), 1);
    assert_eq!(manifest.compiler[0].name(), "compiler1");

    if let Toolchain::Release(compiler) = &manifest.compiler[0] {
        let target_info = compiler.get_target_info("x86_64").expect("Target info not found");
        assert_eq!(target_info.url, "http://example.com");
        assert_eq!(target_info.hash, "hash123");
    } else {
        panic!("Expected a ReleaseToolchain");
    }
}
