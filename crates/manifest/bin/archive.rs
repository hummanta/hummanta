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

use std::{fs::File, path::Path};

use anyhow::{Context, Result};
use flate2::{write::GzEncoder, Compression};
use tar::Builder;

/// Archive the given input directory, and save it to output path
pub async fn archive(input_path: &Path, output_path: &Path) -> Result<()> {
    let file = File::create(output_path)
        .context(format!("Failed to create archive: {:?}", output_path))?;
    let encoder = GzEncoder::new(file, Compression::default());

    let mut tar = Builder::new(encoder);
    tar.append_dir_all("", input_path).context("Failed to add directory to archive")?;
    tar.finish().context("Failed to finish tar creation")?;

    Ok(())
}
