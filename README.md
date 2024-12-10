# ZKsync Era: Vyper Compiler

[![Logo](eraLogo.svg)](https://zksync.io/)

ZKsync Era is a layer 2 rollup that uses zero-knowledge proofs to scale Ethereum without compromising on security
or decentralization. As it’s EVM-compatible (with Solidity/Vyper), 99% of Ethereum projects can redeploy without
needing to refactor or re-audit any code. ZKsync Era also uses an LLVM-based compiler that will eventually enable
developers to write smart contracts in popular languages such as C++ and Rust.

This repository contains the ZKsync Vyper compiler.

## Installation

To install the *zkvyper* compiler, follow the [installation guide](./docs/src/01-installation.md).

For local development, [build zkvyper from sources](./docs/src/01-installation.md#building-from-source).

## Usage

For the detailed usage guide, see the [comprehensive documentation](./docs/src/02-command-line-interface.md).

## Testing

To run the unit and CLI tests, execute the following command from the repository root:

```shell
cargo test
```

## Documentation

Documentation is using [mdBook](https://github.com/rust-lang/mdBook) utility and its sources available in the `docs/` directory.
To build the documentation, follow the [instructions](./docs/README.md).

The deployed versioned builds can be found [here](https://matter-labs.github.io/era-compiler-vyper/latest/).

## Troubleshooting

If you have multiple LLVM builds in your system, ensure that you choose the correct one to build the compiler.
The environment variable `LLVM_SYS_170_PREFIX` sets the path to the directory with LLVM build artifacts, which typically ends with `target-llvm/build-final`.
For example:

```shell
export LLVM_SYS_170_PREFIX=~/repositories/era-llvm/target-llvm/build-final 
```

If you suspect that the compiler is not using the correct LLVM build, check by running `set | grep LLVM`, and reset all LLVM-related environment variables.

For reference, see [llvm-sys](https://crates.io/crates/llvm-sys) and [Local LLVM Configuration Guide](https://llvm.org/docs/GettingStarted.html#local-llvm-configuration).

## License

The Vyper compiler is distributed under the terms of either

- Apache License, Version 2.0, ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

## Resources

- [zkvyper documentation](https://matter-labs.github.io/era-compiler-vyper/latest/)
- [ZKsync Era compiler toolchain documentation](https://docs.zksync.io/zk-stack/components/compiler/toolchain)
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
- [Twitter for Devs](https://twitter.com/ZKsyncDevs)
- [Discord](https://join.zksync.dev/)

## Disclaimer

ZKsync Era has been through extensive testing and audits, and although it is live, it is still in alpha state and
will undergo further audits and bug bounty programs. We would love to hear our community's thoughts and suggestions
about it!
It's important to note that forking it now could potentially lead to missing important
security updates, critical features, and performance improvements.
