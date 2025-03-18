# Manifests

This directory contains configuration files related to toolchains and target platforms for the Hummanta project.

## Directory Structure

### `toolchains/`

Contains configuration files for different programming languages supported by Hummanta.

- `solidity.toml`: Toolchain configuration for Solidity.
- `rust.toml`: Toolchain configuration for Rust. [WIP]
- `index.toml`: An index file that lists all supported toolchains and their versions.

For more information on the toolchain concept, please refer to the [toolchain documentation](https://hummanta.github.io/docs/concepts/toolchain.html).

### `targets/` [WIP]

Contains configuration files for different virtual machine targets supported by Hummanta.

- `index.toml`: Index file for all supported target platforms.
- `evm.toml`: Configuration for the Ethereum Virtual Machine (EVM).
- `move-vm.toml`: Configuration for the Move Virtual Machine.
- `polka-vm.toml`: Configuration for the Polkadot Virtual Machine (PolkaVM).
- `svm.toml`: Configuration for the Solana Virtual Machine (SVM).

For more information on the target concept, please refer to the [target documentation](https://hummanta.github.io/docs/concepts/target.html).

## Usage

### Adding a New Toolchain

1. Create a new `.toml` file for the toolchain inside the `toolchains/` folder.
2. Add the necessary configuration details for the toolchain, including package names, binaries, and other relevant information.
3. Update the `index.toml` file to include the new toolchain.

### Adding a New Target

1. Update the appropriate `.toml` file inside the `targets/` folder (e.g., `evm.toml`, `move-vm.toml`, etc.).
2. Specify the relevant platform information, such as the URL for downloading the package, hash, and any platform-specific details.
