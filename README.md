<div align="center">

# Hummanta Compiler Infrastructure

![GitHub Workflow Status](https://img.shields.io/github/actions/workflow/status/hummanta/hummanta/ci.yml?branch=main)
[![License](https://img.shields.io/github/license/hummanta/hummanta)](https://github.com/hummanta/hummanta/blob/main/LICENSE)
[![GitHub contributors](https://img.shields.io/github/contributors/hummanta/hummanta)](https://github.com/hummanta/hummanta/graphs/contributors)
[![GitHub issues](https://img.shields.io/github/issues/hummanta/hummanta)](https://github.com/hummanta/hummanta/issues)
[![GitHub downloads](https://img.shields.io/github/downloads/hummanta/hummanta/total)](https://github.com/hummanta/hummanta/releases/latest)

[Introduction](#introduction) •
[Installation](#installation) •
[Development](#development) •
[Contributing](#contributing)

</div>

## Introduction

The Hummanta project is a collection of modular, reusable compiler and toolchain technologies for smart contract programming languages, written in Rust, using LLVM as compiler backend. For more details, see our [documentation](https://hummanta.github.io/docs/).

## Installation

Prebuilt binaries Windows, Linux and macOS can be downloaded from the [Github release page](https://github.com/hummanta/hummanta/releases/latest). If there is no distro package available for Hummanta in your preferred manager, you need [Rust and cargo](https://www.rust-lang.org/tools/install) to build it.

### Install from source:

1. Clone the repository with `git clone https://github.com/hummanta/hummanta.git`
2. From the `hummanta` directory, run `cargo build --release` to build the application in release mode.
3. After a successful compilation, launch the executable with: `target/release/hummanta`.

### Install with cargo

To get the latest bug fixes and features, you can install the development version from git. However, this is not fully tested. That means you're probably going to have more bugs despite having the latest bug fixes.

```
cargo install --git https://github.com/hummanta/hummanta
```

This will download the Hummanta CLI from the main branch, and install it in Cargo's global binary directory (`~/.cargo/bin/` by default).

Run `hummanta --help` for a list of all the available commands. Furthermore, you can run `hummanta <command> --help` to get help with a specific command.

## Development

To build this project, you will need to install the following pre-requisites: [Git](https://git-scm.com/downloads), [Rust](https://www.rust-lang.org/tools/install) and [Just](https://github.com/casey/just).

After cloning the repository, you can simply run `just` in the package directory to list all available commands. For your first local build, please run `just install` command to install the dependencies for this project.

## Contributing

If anything feels off, or if you feel that some functionality is missing, please
check out the [contributing page](CONTRIBUTING.md).
There you will find instructions for sharing your feedback, building the project
locally, and submitting pull requests to the project.

## License

Copyright (c) The Hummanta Authors. All rights reserved.

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
