# Command Line Interface (CLI)

The CLI of *zkvyper* is designed with resemblance to the CLI of *vyper*. There are two input/output (I/O) modes in the *zkvyper* interface:

- [Basic CLI](#basic-cli)
- [Combined JSON](./03-combined-json.md)

> All toolkits using *zkvyper* must be operating in combined JSON mode and follow [its specification](./03-combined-json.md).
> It will make the toolkits more robust and future-proof, as the combined JSON mode is the most versatile and used for the majority of popular projects.

This page focuses on the basic CLI mode. For more information on combined JSON, see [this page](./03-combined-json.md).



## Basic CLI

Basic CLI mode is the simplest way to compile a file with the source code.

To compile a basic Vyper contract, make sure that [the *vyper* compiler](#--vyper) is present in your environment and run [the example](#--vyper).

The rest of this section describes the available CLI options and their usage. You may also check out `zkvyper --help` for a quick reference.



### `--vyper`

Specifies the path to the *vyper* compiler. Useful when the *vyper* compiler is not available in the system path.

Usage:

```shell
zkvyper './Simple.vy' --vyper '/path/to/vyper'
```

> Examples in the subsequent sections assume that *vyper* [is installed and available](./01-installation.md#installing-vyper) in the system path.
> If you prefer specifying the full path to *vyper*, use the `--vyper` option with the examples below.



### Input Files

*zkvyper* supports multiple input files. The following command compiles two Vyper files and prints the bytecode:

```shell
zkvyper './Simple.vy' './Complex.vy' --bin
```



### `--output-dir`

Specifies the output directory for build artifacts. Can only be used with [basic CLI](#basic-cli) and [combined JSON](./04-combined-json.md) modes.

Usage in basic CLI mode:

```shell
zkvyper './Simple.vy' --asm --metadata --output-dir './build/'
ls './build/Simple.vy'
```

Output:

```text
Compiler run successful. Artifact(s) can be found in directory "build".
...
Test.zasm       Test.zbin       Test_meta.json
```

Usage in combined JSON mode:

```shell
zkvyper './Simple.vy' --combined-json 'bin,asm,metadata' --output-dir './build/'
ls './build/'
```

Output:

```text
Compiler run successful. Artifact(s) can be found in directory "build".
...
combined.json
```



### `--overwrite`

Overwrites the output files if they already exist in the output directory. By default, *zkvyper* does not overwrite existing files.

Can only be used in combination with the [`--output-dir`](#--output-dir) option.

Usage:

```shell
zkvyper './Simple.vy' --combined-json 'bin,asm,metadata' --output-dir './build/' --overwrite
```

If the `--overwrite` option is not specified and the output files already exist, *zkvyper* will print an error message and exit:

```text
Error: Refusing to overwrite an existing file "build/combined.json" (use --overwrite to force).
```



### `--version`

Prints the version of *zkvyper* and the hash of the LLVM commit it was built with.

Usage:

```shell
zkvyper --version
```



### `--help`

Prints the help message.

Usage:

```shell
zkvyper --help
```



## Compilation Settings



### `--optimization / -O`

Sets the optimization level of the LLVM optimizer. Available values are:

| Level | Meaning                      | Hints                                            |
|:-----:|:----------------------------:|:------------------------------------------------:|
| 0     | No optimization              | Best compilation speed: for active development
| 1     | Performance: basic           | For optimization research
| 2     | Performance: default         | For optimization research
| 3     | Performance: aggressive      | Default value. Best performance: for production
| s     | Size: default                | For optimization research
| z     | Size: aggressive             | Best size: for contracts with size constraints

For most cases, it is fine to use the default value of `3`. You should only use the level `z` if you are ready to deliberately sacrifice performance and optimize for size.

> Large contracts may hit the EraVM or EVM bytecode size limit. In this case, it is recommended to use the [`--fallback-Oz`](#--fallback-oz) option rather than set the `z` level.



### `--fallback-Oz`

Sets the optimization level to `z` for contracts that failed to compile due to overrunning the bytecode size constraints.

Under the hood, this option automatically triggers recompilation of contracts with level `z`. Contracts that were successfully compiled with [the original `--optimization` setting](#--optimization---o) are not recompiled.

> It is recommended to have this option always enabled to prevent compilation failures due to bytecode size constraints. There are no known downsides to using this option.



### `--llvm-options`

Specifies additional options for the LLVM framework. The argument must be a single quoted string following a `=` separator.

Usage:

```shell
zkvyper './Simple.vy' --llvm-options='-eravm-jump-table-density-threshold=10'
```

> The `--llvm-options` option is experimental and must only be used by experienced users. All supported options will be documented in the future.



### `--evm-version`

Specifies the EVM version *vyper* will produce artifacts for. Only LLL IR is known to be affected by this option. For instance, if the EVM version is set to *cancun*, the LLL IR may contain `MCOPY` instructions.

> EVM version only affects IR artifacts produced by *vyper* and does not affect EraVM bytecode produced by *zkvyper*.

The following values are allowed, however have in mind that newer EVM versions are only supported by newer versions of *vyper*:
- homestead
- tangerineWhistle
- spuriousDragon
- byzantium
- constantinople
- petersburg
- istanbul
- berlin
- london
- paris
- shanghai
- cancun
- prague

Usage:

```shell
zkvyper './Simple.vy' --evm-version 'cancun'
```



### `--metadata-hash`

Specifies the hash function used for contract metadata.

The following values are allowed:

|     Value    |  Size  | Padding | Reference |
|:------------:|:------:|:-------:|:---------:|
| none         |  0 B   | 0-32 B  | 
| keccak256    | 32 B   | 0-32 B  | [SHA-3 Wikipedia Page](https://en.wikipedia.org/wiki/SHA-3)
| ipfs         | 44 B   | 20-52 B | [IPFS Documentation](https://docs.ipfs.tech/)

The default value is `keccak256`.

> EraVM requires its bytecode size to be an odd number of 32-byte words. If the size after appending the hash does not satisfy this requirement, the hash is *prepended* with zeros according to the *Padding* column in the table above.

Usage:

```shell
zkvyper './Simple.vy' --metadata-hash 'ipfs'
```



### `--suppress-warnings`

Tells the compiler to suppress specified warnings. The option accepts multiple string arguments, so make sure they are properly separated by whitespace.

Only one warning can be suppressed with this option: [`txorigin`](https://docs.zksync.io/build/tooling/foundry/migration-guide/testing#origin-address).

Usage:

```shell
zkvyper './Simple.vy' --suppress-warnings 'txorigin'
```



## Multi-Language Support

*zkvyper* supports input in multiple programming languages:

- [Vyper](https://vyperlang.org/)
- [LLVM IR](https://llvm.org/docs/LangRef.html)
- [EraVM assembly](https://docs.zksync.io/zk-stack/components/compiler/specification/binary-layout)

The following sections outline how to use *zkvyper* with these languages.



### `--llvm-ir`

Enables the LLVM IR mode. In this mode, input is expected to be in the LLVM IR language. The output works the same way as with Vyper input.

Unlike *vyper*, *zkvyper* is an LLVM-based compiler toolchain, so it uses LLVM IR as an intermediate representation. It is not recommended to write LLVM IR manually, but it can be useful for debugging and optimization purposes. LLVM IR is more low-level than Vyper LLL in the ZKsync compiler toolchain IR hierarchy, so *vyper* is not used for compilation.

Usage:

```shell
zkvyper --llvm-ir './Simple.ll' --bin
```

Output:

```text
======= Simple.ll =======
Binary:
000000000002004b000000070000613d0000002001000039000000000010043f...
```



### `--eravm-assembly`

Enables the EraVM Assembly mode. In this mode, input is expected to be in the EraVM assembly language. The output works the same way as with Vyper input.

EraVM assembly is a representation the closest to EraVM bytecode. It is not recommended to write EraVM assembly manually, but it can be even more useful for debugging and optimization purposes than LLVM IR.

For the EraVM assembly specification, visit the [EraVM documentation](https://docs.zksync.io/zk-stack/components/compiler/specification/binary-layout).

Usage:

```shell
zkvyper --eravm-assembly './Simple.zasm' --bin
```

Output:

```text
======= Simple.zasm =======
Binary:
000000000120008c000000070000613d00000020010000390000000000100435...
```



## Integrated Tooling

*zkvyper* includes several tools provided by the LLVM framework out of the box, such as disassembler and linker. The following sections describe the usage of these tools.

> The mode-altering CLI options are mutually exclusive. This means that only one of the options below can be enabled at a time:
> - `-f`
> - `--llvm-ir`
> - `--eravm-assembly`
> - `--disassemble`



### `--disassemble`

Enables the disassembler mode.

*zkvyper* includes an LLVM-based disassembler that can be used to disassemble compiled bytecode.

The disassembler input must be files with a hexadecimal string. The disassembler output is a human-readable representation of the bytecode, also known as EraVM assembly.

Usage:

```shell
cat './input.zbin'
```

Output:

```text
0x0000008003000039000000400030043f0000000100200190000000140000c13d00000000020...
```

```shell
zkvyper --disassemble './input.zbin'
```

Output:

```text
File `input.zbin` disassembly:

       0: 00 00 00 80 03 00 00 39       add     128, r0, r3
       8: 00 00 00 40 00 30 04 3f       stm.h   64, r3
      10: 00 00 00 01 00 20 01 90       and!    1, r2, r0
      18: 00 00 00 14 00 00 c1 3d       jump.ne 20
      20: 00 00 00 00 02 01 00 19       add     r1, r0, r2
      28: 00 00 00 0b 00 20 01 98       and!    code[11], r2, r0
      30: 00 00 00 23 00 00 61 3d       jump.eq 35
      38: 00 00 00 00 01 01 04 3b       ldp     r1, r1
```



## Debugging



### `--debug-output-dir`

Specifies the directory to store intermediate build artifacts. The artifacts can be useful for debugging and research.

The directory is created if it does not exist. If artifacts are already present in the directory, they are overwritten.

The intermediate build artifacts can be:

| Name            | File extension   |
|:---------------:|:----------------:|
| LLL             | *lll*            |
| LLVM IR         | *ll*             |
| EraVM assembly  | *zasm*           |

Usage:

```shell
zkvyper './Simple.vy' --debug-output-dir './debug/'
ls './debug/'
```

Output:

```text
Compiler run successful. No output requested. Use flags --metadata, --asm, --bin.
...
Simple.vy.C.runtime.optimized.ll
Simple.vy.C.runtime.unoptimized.ll
Simple.vy.C.yul
Simple.vy.C.zasm
Simple.vy.Test.runtime.optimized.ll
Simple.vy.Test.runtime.unoptimized.ll
Simple.vy.Test.yul
Simple.vy.Test.zasm
```

The output file name is constructed as follows: `<ContractPath>.<ContractName>.<Modifiers>.<Extension>`.



### `--llvm-verify-each`

Enables the verification of the LLVM IR after each optimization pass. This option is useful for debugging and research purposes.

Usage:

```shell
zkvyper './Simple.vy' --llvm-verify-each
```



### `--llvm-debug-logging`

Enables the debug logging of the LLVM IR optimization passes. This option is useful for debugging and research purposes.

Usage:

```shell
zkvyper './Simple.vy' --llvm-debug-logging
```