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

/// Parse a value from bytes of text.
pub trait FromSlice: Sized {
    /// The associated error which can be returned from parsing.
    type Err;

    /// Deserialize an instance of type `T` from bytes of text.
    fn from_slice(v: &[u8]) -> Result<Self, Self::Err>;
}
