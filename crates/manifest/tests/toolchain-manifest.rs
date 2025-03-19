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
    [detector.detector1]
        package = "package1"
        bin = "bin1"

    [compiler.compiler1]
        version = "1.0.0"

    [compiler.compiler1.target.x86_64-unknown-linux-gnu]
        url = "http://example.com"
        hash = "hash123"
"#;

fn normalize_toml(toml_str: &str) -> String {
    let value: toml::Value = toml_str.parse().unwrap();
    value.to_string()
}

#[test]
fn test_toolchain_manifest_to_toml() {
    let mut manifest = ToolchainManifest::new();

    // Add a PackageToolchain
    let package_toolchain =
        Toolchain::Package(PackageToolchain::new("package1".to_string(), Some("bin1".to_string())));
    manifest.insert("detector".to_string(), "detector1".to_string(), package_toolchain);

    // Add a ReleaseToolchain
    let mut target_info = HashMap::new();
    target_info.insert(
        "x86_64-unknown-linux-gnu".to_string(),
        TargetInfo::new("http://example.com".to_string(), "hash123".to_string()),
    );
    let release_toolchain =
        Toolchain::Release(ReleaseToolchain::new("1.0.0".to_string(), target_info));
    manifest.insert("compiler".to_string(), "compiler1".to_string(), release_toolchain);

    // Serialize to TOML
    let toml_string = toml::to_string(&manifest).expect("Failed to serialize to TOML");

    assert_eq!(normalize_toml(&toml_string), normalize_toml(TOOLCHAIN_TOML));
}

#[test]
fn test_toml_to_toolchain_manifest() {
    // Deserialize from TOML
    let manifest: ToolchainManifest = toml::from_str(TOOLCHAIN_TOML).expect("Failed to parse TOML");

    // Validate PackageToolchain
    let detector = manifest.get("detector", "detector1").expect("Missing detector1");
    if let Toolchain::Package(pkg) = detector {
        assert_eq!(pkg.package, "package1");
        assert_eq!(pkg.bin, Some("bin1".to_string()));
    } else {
        panic!("Expected PackageToolchain for detector1");
    }

    // Validate ReleaseToolchain
    let compiler = manifest.get("compiler", "compiler1").expect("Missing compiler1");
    if let Toolchain::Release(rel) = compiler {
        assert_eq!(rel.version, "1.0.0");
        let target_info =
            rel.get_target_info("x86_64-unknown-linux-gnu").expect("Missing target info");
        assert_eq!(target_info.url, "http://example.com");
        assert_eq!(target_info.hash, "hash123");
    } else {
        panic!("Expected ReleaseToolchain for compiler1");
    }
}
