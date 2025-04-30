# Installing the ZKsync Vyper Compiler Toolchain

To compile contracts for ZKsync, you need the ZKsync Vyper compiler toolchain.
It consists of two components:

1. The main component: [*zkvyper*](https://github.com/matter-labs/era-compiler-vyper/releases).
2. The additional component: [*vyper*](https://github.com/vyperlang/vyper/releases), which produces Vyper artifacts used by *zkvyper*.



## System Requirements

It is recommended to have at least 4 GB of RAM to compile large projects. The compilation process is parallelized by default, so the number of threads used is
equal to the number of CPU cores.

> Large projects can consume a lot of RAM during compilation on machines with a high number of cores.
> If you encounter memory issues, consider reducing the number of threads using the `--threads` option.

The table below outlines the supported platforms and architectures:

| CPU/OS | MacOS | Linux | Windows |
|:------:|:-----:|:-----:|:-------:|
| x86_64 |   ✅   |   ✅   |    ✅    |
| arm64  |   ✅   |   ✅   |    ❌    |

> Please avoid using outdated distributions of operating systems, as they may lack the necessary dependencies or include outdated versions of them.
> *zkvyper* is only tested on recent versions of popular distributions, such as MacOS 11.0 and Windows 10.

<div class="warning">
<a href="https://musl.libc.org">musl</a>-based builds are deprecated, but they are still supported to preserve tooling compatibility.<br>
Starting from <b>zkvyper</b> v1.5.4, we are shipping builds statically linked with <a href="https://www.gnu.org/software/libc/">the GNU C library</a>.
</div>



## Versioning

The *zkvyper* versioning scheme does not yet follow the [Semantic Versioning](https://semver.org) specification. Instead, its major and minor versions match those of the EraVM protocol for which *zkvyper* produces bytecode. The patch version is incremented with each release, regardless of whether breaking changes are introduced. Therefore, please consult the changelog before updating the compiler.

> We recommend always using the latest version of *zkvyper* and *vyper* to benefit from the latest features and bug fixes.



## Installing *zkvyper*

You can install the ZKsync Vyper compiler toolchain using the following methods:

1. Use Foundry, Hardhat, or other popular toolkits, so they will manage the compiler installation and their dependencies for you. See [Ethereum Development Toolkits](#ethereum-development-toolkits).
2. Download pre-built binaries of [*vyper*](https://github.com/vyperlang/vyper/releases) and [*zkvyper*](https://github.com/matter-labs/era-compiler-vyper/releases). See [Static Executables](#static-executables).
3. Build *zkvyper* from sources. See [Building from Source](#building-from-source).

> For small projects, learning and research purposes, *zkvyper* and *vyper* executables without a toolkit are sufficient.



## Installing *vyper*

Running *zkvyper* requires the [Vyper compiler *vyper*](https://github.com/vyperlang/vyper/releases), which is called by *zkvyper* as a child process. To point *zkvyper* to the location of *vyper*, use one of the following methods:

1. Add the location of *vyper* to the environment variable `PATH`. 
  
   For example, if you have downloaded *vyper* to the directory `/home/username/opt`,
   you can execute the following command, or append it to the configuration file of your shell:

    ```shell
    export PATH="/home/username/opt:${PATH}"
    ```

2. Alternatively, when you run *zkvyper*, provide the full path to *vyper* using the `--vyper` option.

   For example, if `vyper` is located in your current working directory, you can point to it with this command:

    ```shell
    zkvyper --vyper './vyper' --bin 'Greeter.vy'
    ```

> The second option is more convenient if you are using different versions of *vyper* for different projects.
> *zkvyper* only supports *vyper* of versions 0.3.3 and 0.3.9 and newer, but does not support versions from 0.3.4 to 0.3.8.



## Ethereum Development Toolkits

For large codebases, it is more convenient to use the ZKsync compiler toolchain via toolkits like Foundry and Hardhat.
These tools manage the compiler executables and their dependencies, and provide additional features like incremental compilation and caching.

The ZKsync toolchain is supported by the following toolkits:

*TODO: Add links to the tutorials*



## Static Executables

We ship *zkvyper* binaries on the [releases page of `matter-labs/era-compiler-vyper` repository](https://github.com/matter-labs/era-compiler-vyper/releases). 
This repository maintains intuitive and stable naming for the executables and provides a changelog for each release. Tools using *zkvyper* will download the binaries from this repository and cache them locally.

<div class="warning">
The <a href="https://github.com/matter-labs/era-compiler-vyper">matter-labs/era-compiler-vyper</a> repository only contains builds for versions 1.4.0 and newer.<br>
You can download older versions from <a href="https://github.com/matter-labs/zkvyper-bin/tree/main">the main branch</a> or <a href="https://github.com/matter-labs/zkvyper-bin/releases">the releases page</a> of the deprecated repository for zkvyper executables.<br>
If any of your projects are still using the old locations, please change their download URLs to <a href="https://github.com/matter-labs/era-compiler-vyper/releases">the new one</a>.
</div>

> All binaries are statically linked and must work on all recent platforms without issues.
> *zkvyper* is fully written in Rust, aiming to minimize incompatibilities with the environment.



## Building from Source

> Please consider using the pre-built executables before building from source.
> Building from source is only necessary for development, research, and debugging purposes.
> Deployment and production use cases should rely only on [the officially released executables](#static-executables).

1. Install the necessary system-wide dependencies.

   * For Linux (Debian):

    ```shell
    apt install cmake ninja-build curl git libssl-dev pkg-config clang lld
    ```

   * For Linux (Arch):

    ```shell
    pacman -Syu which cmake ninja curl git pkg-config clang lld
    ```

   * For MacOS:

     1. Install the *Homebrew* package manager by following the instructions at [brew.sh](https://brew.sh).
     2. Install the necessary system-wide dependencies:

        ```shell
        brew install cmake ninja coreutils
        ```

     3. Install a recent build of the LLVM/[Clang](https://clang.llvm.org) compiler using one of the following tools:
        * [Xcode](https://developer.apple.com/xcode/)
        * [Apple’s Command Line Tools](https://developer.apple.com/library/archive/technotes/tn2339/_index.html)
        * Your preferred package manager.

2. Install Rust.

   The easiest way to do it is following the latest [official instructions](https://www.rust-lang.org/tools/install).

> The Rust version used for building is pinned in the [rust-toolchain.toml](../rust-toolchain.toml) file at the repository root.
> *cargo* will automatically download the pinned version of *rustc* when you start building the project.

3. Clone and checkout this repository.

   ```shell
   git clone https://github.com/matter-labs/era-compiler-vyper --recursive
   ```

   By default, submodules checkout is disabled to prevent cloning large repositories via `cargo`.
   If you're building locally, ensure all submodules are checked out with:
   ```shell
   git submodule update --recursive --checkout
   ```
    
4. Install the ZKsync LLVM framework builder. This tool clones the [repository of ZKsync LLVM Framework](https://github.com/matter-labs/era-compiler-llvm) and runs a sequence of build commands tuned for the needs of ZKsync compiler toolchain.

    ```shell
    cargo install compiler-llvm-builder
    ```

    To fine-tune your build of ZKsync LLVM framework, refer to the section [Fine tuning ZKsync LLVM build](#fine-tuning-zksync-llvm-build)

> Always use the latest version of the builder to benefit from the latest features and bug fixes.
> To check for new versions and update the builder, simply run `cargo install compiler-llvm-builder` again, even if you have already installed the builder.
> The builder is not the ZKsync LLVM framework itself, but a tool to build it.
> By default, it is installed in `~/.cargo/bin/`, which is usually added to your `PATH` during the Rust installation process.

5. Clone and build the ZKsync LLVM framework using the `zksync-llvm` tool.
  
   ```shell
   # Navigate to the root of your local copy of this repository.
   cd era-compiler-vyper
   # Build the ZKsync LLVM framework.
   zksync-llvm build
   ```
  
   For more information and available build options, run `zksync-llvm build --help`.
   
   You can also clone and build LLVM framework outside of the repository root.
   In this case, do the following:

   1. Provide an `LLVM.lock` file in the directory where you run `zksync-llvm`:
      ```text
      url = "https://github.com/matter-labs/era-compiler-llvm"
      branch = "main"
      ```
   2. Ensure that `LLVM.lock` selects the correct branch of the [ZKsync LLVM Framework repository](https://github.com/matter-labs/era-compiler-llvm).
   3. Before proceeding to the next step, set the environment variable `LLVM_SYS_191_PREFIX` to the path of the directory with the LLVM build artifacts.
      Typically, it ends with `target-llvm/build-final`, which is the default LLVM target directory of the LLVM builder. For example:

      ```shell
      export LLVM_SYS_191_PREFIX=~/repositories/era-compiler-vyper/target-llvm/build-final 
      ```

6. Build the *zkvyper* executable.

    ```shell
    cargo build --release
    ```
   
    The *zkvyper* executable will appear at `./target/release/zkvyper`, where you can run it directly or move it to another location.

    If *cargo* cannot find the LLVM build artifacts, return to the previous step and ensure that the `LLVM_SYS_191_PREFIX` environment variable is set to the absolute path of the directory `target-llvm/build-final`.



## Tuning the ZKsync LLVM build

* For more information and available build options, run `zksync-llvm build --help`.
* Use the `--use-ccache` option to speed up the build process if you have [ccache](https://ccache.dev) installed.
* To build ZKsync LLVM framework using specific C and C++ compilers, pass additional arguments to [CMake](https://cmake.org/) using the `--extra-args` option:

  ```shell
  # Pay special attention to character escaping.

  zksync-llvm build \
    --use-ccache \
    --extra-args \
      '\-DCMAKE_C_COMPILER=/opt/homebrew/Cellar/llvm@18/18.1.8/bin/clang' \
      '\-DCMAKE_BUILD_TYPE=Release' \
      '\-DCMAKE_CXX_COMPILER=/opt/homebrew/Cellar/llvm@18/18.1.8/bin/clang++' 
  ```

### Building LLVM manually

* If you prefer building [your ZKsync LLVM](https://github.com/matter-labs/era-compiler-llvm) manually, include the following flags in your CMake command:

  ```shell
  # We recommended using the latest version of CMake.

  -DLLVM_TARGETS_TO_BUILD='EraVM;EVM'
  -DLLVM_ENABLE_PROJECTS='lld'
  -DBUILD_SHARED_LIBS='Off'
  ```

> For most users, the [ZKsync LLVM builder](#building-from-source) is the recommended way to build the ZKsync LLVM framework.
> This section exists for the ZKsync toolchain developers and researchers with specific requirements and experience with the LLVM framework.
> We are going to present a more detailed guide for LLVM contributors in the future.
