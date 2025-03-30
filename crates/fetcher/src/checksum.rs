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

use sha2::{Digest, Sha256};

use crate::errors::{FetchError, FetchResult};

/// Verifies SHA-256 hash of the data
pub fn verify(data: &[u8], expected_hash: &str) -> FetchResult<()> {
    let mut hasher = Sha256::new();
    hasher.update(data);
    let actual_hash = format!("{:x}", hasher.finalize());

    if actual_hash != expected_hash {
        return Err(FetchError::HashMismatch {
            expected: expected_hash.to_string(),
            actual: actual_hash,
        });
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_verify_success() {
        let data = b"test data";
        let expected_hash = "916f0027a575074ce72a331777c3478d6513f786a591bd892da1a577bf2335f9";
        assert!(verify(data, expected_hash).is_ok());
    }

    #[test]
    fn test_verify_failure() {
        let data = b"test data";
        let expected_hash = "incorrect_hash";
        let result = verify(data, expected_hash);

        assert!(result.is_err());

        if let Err(FetchError::HashMismatch { expected, actual }) = result {
            assert_eq!(expected, expected_hash);
            assert_ne!(actual, expected_hash);
        }
    }
}
