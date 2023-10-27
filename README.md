# zkSync Era: Vyper Compiler

[![Logo](eraLogo.svg)](https://zksync.io/)

zkSync Era is a layer 2 rollup that uses zero-knowledge proofs to scale Ethereum without compromising on security
or decentralization. As it’s EVM-compatible (with Solidity/Vyper), 99% of Ethereum projects can redeploy without
needing to refactor or re-audit any code. zkSync Era also uses an LLVM-based compiler that will eventually enable
developers to write smart contracts in popular languages such as C++ and Rust.

This repository contains the EraVM Vyper compiler.

## System Requirements

Supported platforms:
- **Linux: x86_64**  
   MUSL-based static builds do not depend on system libraries and run on any recent Linux distribution.  
- **MacOS 11+: x86_64, arm64 (M1, M2)**
- **Windows: x86_64**  
   Only Windows 10 has been tested so far, but other versions should be OK as well.  

We recommend at least 4 GB of RAM available for the build process.

## Building

1. Install some tools system-wide:  
   1.a. `apt install cmake ninja-build clang-13 lld-13 parallel` on a Debian-based Linux, with optional `musl-tools` if you need a `musl` build.  
   1.b. `pacman -S cmake ninja clang lld parallel` on an Arch-based Linux.  
   1.c. On MacOS, install the [HomeBrew](https://brew.sh) package manager (being careful to install it as the appropriate user), then `brew install cmake ninja coreutils parallel`. Install your choice of a recent LLVM/[Clang](https://clang.llvm.org) compiler, e.g. via [Xcode](https://developer.apple.com/xcode/), [Apple’s Command Line Tools](https://developer.apple.com/library/archive/technotes/tn2339/_index.html), or your preferred package manager.  
   1.d. Their equivalents with other package managers.  

2. [Install Rust](https://www.rust-lang.org/tools/install)

   Currently we are not pinned to any specific version of Rust, so just install the latest stable build for your platform.  
   Also install the `musl` target if you are compiling on Linux in order to distribute the binary:  
   `rustup target add x86_64-unknown-linux-musl`  

3. Download [v0.3.3, v0.3.9, or v0.3.10 of the Vyper compiler](https://github.com/vyperlang/vyper/releases).  
   If it is not named exactly `vyper` in your `$PATH`, see the `--vyper` option below.  
   
4. Check out or clone the appropriate branch of this repository.  

5. Go to the project root and run `git checkout <ref>` with the tag, branch, or commit you want to build.  

6. Install the EraVM LLVM framework builder:  
   6.a. `cargo install compiler-llvm-builder` on MacOS, or Linux for personal use.  
   6.b. `cargo install compiler-llvm-builder --target x86_64-unknown-linux-musl` on Linux for distribution.  

   The builder is not the [EraVM LLVM framework](https://github.com/matter-labs/compiler-llvm) itself; it is just a tool that clones our repository and runs the sequence of build commands.  
   By default it is installed in `~/.cargo/bin/`, which is recommended to be added to your `$PATH`. Execute `zkevm-llvm --help` for more information.  
   If you need a specific branch of EraVM LLVM, change it in the `LLVM.lock` file at the root of this repository.  

7. Run the builder to clone and build the EraVM LLVM framework at this repository root:  
   7.1. `zkevm-llvm clone`  
   7.2. `zkevm-llvm build`  

8. Build the Vyper compiler executable:  
   8.a. `cargo build --release` on MacOS or Linux for personal use.  
   8.b. `cargo build --release --target x86_64-unknown-linux-musl` on Linux for distribution.  

9. If you need to move the built binary elsewhere, grab it from the build directory:  
   9.a. On MacOS or Linux for the default target: `./target/release/zkvyper`  
   9.b. On Linux, if you are building for the target `x86_64-unknown-linux-musl`: `./target/x86_64-unknown-linux-musl/release/zkvyper`  

## Usage

Check `./target/*/zkvyper --help` for compiler usage.  

A support version of the Vyper compiler must be available in `$PATH`, or its path must be passed explicitly with the `--vyper` option.

Supported versions:
- 0.3.3
- 0.3.9
- 0.3.10

For big projects it is more convenient to use the compiler via the Hardhat plugin. For single-file contracts, or small
projects, the CLI suffices.

## Unit testing

For running unit tests, `zkvyper` itself must also be available in `$PATH`, because it calls itself recursively to allow
compiling each contract in a separate process. To successfully run unit tests:

1. Run `cargo build --release`.
2. Move the binary from `./target/release/zkvyper` to a directory from `$PATH`, or add the target directory itself to `$PATH`.
3. Run `cargo test`.

## CLI reference

#### `--version`
Print the version and exit.  

#### `<input_files>`
Specify the input file paths.  
Multiple Vyper files can be passed in the default Vyper mode.  
LLVM IR mode currently supports only a single file.  

#### `-o`, `--output-dir <path>`
Create one file per component and contract/file at the specified directory, if given.  

#### `--overwrite`
Overwrite existing files (used together with -o).  

#### `-O`, `--optimization <level>`
Set the LLVM optimization parameter `-O[0 | 1 | 2 | 3 | s | z]`.  
Use `3` for best performance and `z` for minimal size.  

#### `--disable-vyper-optimizer`
Disable the `vyper` LLL IR optimizer.  

#### `--vyper <path>`
Specify the path to the `vyper` executable. By default, the one in `${PATH}` is used.  
In LLVM IR mode `vyper` is unused.  

#### `-f <format>`
An extra output format string.  
See `vyper --help` for available options including combined JSON mode.

#### `--llvm-ir`
Switch to LLVM IR mode.  
Only one input LLVM IR file is allowed.  
Cannot be used with combined JSON mode.  
Use this mode at your own risk, as LLVM IR input validation is not implemented.  

#### `--zkasm`
Switch to EraVM assembly mode.  
Only one input EraVM assembly file is allowed.  
Cannot be used with combined JSON mode.  
Use this mode at your own risk, as EraVM assembly input validation is not implemented.  

#### `--metadata-hash`
Set metadata hash mode: `keccak256` | `none`.
`keccak256` is enabled by default.

#### `--debug-output-dir <path>`
Dump all IR (LLL, LLVM IR, assembly) to files in the specified directory.  
Only for testing and debugging.  

#### `--llvm-verify-each`
Set the `verify-each` option in LLVM.
Only for testing and debugging.  

#### `--llvm-debug-logging`
Set the `debug-logging` option in LLVM.
Only for testing and debugging.  

#### `--recursive-process`
Run this process recursively and provide JSON input to compile a single contract.  
Only for usage from within the compiler.  

## Troubleshooting

- If you get a “failed to authenticate when downloading repository… if the git CLI succeeds then net.git-fetch-with-cli may help here” error, then prepending the `cargo` command with `CARGO_NET_GIT_FETCH_WITH_CLI=true` may help.
- On MacOS, `git config --global credential.helper osxkeychain` followed by cloning a repository manually with a personal access token may help.
- Unset any LLVM-related environment variables you may have set, especially `LLVM_SYS_<version>_PREFIX` (see e.g. [https://crates.io/crates/llvm-sys](https://crates.io/crates/llvm-sys) and [https://llvm.org/docs/GettingStarted.html#local-llvm-configuration](https://llvm.org/docs/GettingStarted.html#local-llvm-configuration)). To make sure: `set | grep LLVM`

## License

The Vyper compiler is distributed under the terms of either

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

## Resources

- [zkSync Era compiler toolchain documentation](https://era.zksync.io/docs/api/compiler-toolchain)
- [Vyper v0.3.3 documentation](https://vyper.readthedocs.io/en/v0.3.3/)
- [Vyper v0.3.9 documentation](https://vyper.readthedocs.io/en/v0.3.9/)
- [Vyper v0.3.10 documentation](https://vyper.readthedocs.io/en/v0.3.10/)
- [Vyper LLL IR](https://github.com/vyperlang/vyper/blob/master/vyper/ir/README.md)  

> Some parts of the Vyper documentation may be outdated.  
> Please contact the Vyper team for assistance.  

## Official Links

- [Website](https://zksync.io/)
- [GitHub](https://github.com/matter-labs)
- [Twitter](https://twitter.com/zksync)
- [Twitter for Devs](https://twitter.com/zkSyncDevs)
- [Discord](https://join.zksync.dev/)

## Disclaimer

zkSync Era has been through extensive testing and audits, and although it is live, it is still in alpha state and
will undergo further audits and bug bounty programs. We would love to hear our community's thoughts and suggestions
about it!
It's important to note that forking it now could potentially lead to missing important
security updates, critical features, and performance improvements.
