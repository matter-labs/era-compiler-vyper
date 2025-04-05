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
zkvyper 'Simple.vy' --vyper '/path/to/vyper'
```

> Examples in the subsequent sections assume that *vyper* [is installed and available](./01-installation.md#installing-vyper) in the system path.
> If you prefer specifying the full path to *vyper*, use the `--vyper` option with the examples below.



### Input Files

*zkvyper* supports multiple input files. The following command compiles two Vyper files and prints the bytecode:

```shell
zkvyper 'Simple.vy' './Complex.vy'
```



### `--format` / `-f`

This option can be used for two purposes:
1. Switch to [combined JSON mode](./03-combined-json.md).
2. Select the desired output in [basic CLI mode](#basic-cli).

In basic CLI mode, the following selectors are available:

|       Selector       |  Source   |                                 Description                                                    |
|:--------------------:|:---------:|:----------------------------------------------------------------------------------------------:|
| combined_json        | both      | Switches to [combined JSON mode](./03-combined-json.md). Cannot be used with other selectors.
| ir_json              | vyper     | Vyper LLL IR that is used by *zkvyper* to produce LLVM IR.
| ast                  | vyper     | Abstract Syntax Tree (AST) of the Vyper source code.
| abi                  | vyper     | Application Binary Interface (ABI) of the Vyper contract.
| method_identifiers   | vyper     | Hashes of function signature of the Vyper contract.
| layout               | vyper     | Storage and code layouts of the Vyper contract.
| userdoc              | vyper     | User documentation of the Vyper contract.
| devdoc               | vyper     | Developer documentation of the Vyper contract.
| eravm_assembly       | zkvyper   | EraVM assembly of the Vyper contract.
| project_metadata     | zkvyper   | Project metadata of the Vyper contract.

> Some data above is produced by *vyper*, whereas the rest is produced by *zkvyper*, as designated in the *Source* column.

Usage:

```shell
zkvyper 'Simple.vy' --format 'ir_json,ast,abi,method_identifiers,layout,userdoc,devdoc,eravm_assembly,project_metadata'
```

Output:

```text
Contract `/Users/hedgarmac/src/era-compiler-tester//tests/vyper/simple/default.vy`:
0x0000000100200190000000110000c13d0000000a001001980000001d0000613d...(truncated)
{"seq":[(truncated)]}
{"contract_name":"/Users/hedgarmac/src/era-compiler-tester/tests/vyper/simple/default.vy","ast":{(truncated)}}
[{"inputs":[],"name":"first","outputs":[{"name":"","type":"uint8"}],"stateMutability":"pure","type":"function"},{"inputs":[],"name":"second","outputs":[{"name":"","type":"uint256"}],"stateMutability":"pure","type":"function"}]
{"first()":"0x3df4ddf4","second()":"0x5a8ac02d"}
{}
{}
{}
Contract `/Users/hedgarmac/src/era-compiler-tester//tests/vyper/simple/default.vy` assembly:
        .text
        .file   "default.vy"
        .globl  __entry
__entry:
.func_begin0:
        and!    1, r2, r0
        jump.ne @.BB0_9
        and!    code[@CPI0_1], r1, r0
        jump.eq @.BB0_10
... (truncated)

Project metadata:
{"evm_version":"cancun","llvm_options":[],"optimizer_settings":"M3B3","source_code_hash":[147,242,126,144,(truncated),22,153,132,218],"source_version":"0.4.1","zk_version":"1.5.10"}
```

The output order above is fixed and cannot be changed by the order of the selectors in the `--format` argument:

1. Bytecode
2. LLL IR JSON
3. AST
4. ABI
5. Method identifiers
6. Layout
7. User documentation
8. Developer documentation
9. EraVM assembly
10. Project metadata



### `--output-dir`

Specifies the output directory for build artifacts. Can only be used in [basic CLI](#basic-cli) and [combined JSON](./03-combined-json.md) modes.

Usage in basic CLI mode:

```shell
zkvyper 'Simple.vy' --output-dir './build/'
ls './build/'
```

Output:

```text
default.vy.zbin
```

Usage in combined JSON mode:

```shell
zkvyper 'Simple.vy' --format 'combined_json' --output-dir './build/'
ls './build/'
```

Output:

```text
combined.json
```



### `--overwrite`

Overwrites the output files if they already exist in the output directory. By default, *zkvyper* does not overwrite existing files.

Can only be used in combination with the [`--output-dir`](#--output-dir) option.

Usage:

```shell
zkvyper 'Simple.vy' --format 'combined_json' --output-dir './build/' --overwrite
```

If the `--overwrite` option is not specified and the output files already exist, *zkvyper* will print an error message and exit:

```text
Refusing to overwrite an existing file "./build/combined.json" (use --overwrite to force).
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



## Other I/O Modes

To switch to combined JSON mode, use [the `--format` option](#--format---f) with the `combined_json` argument:

```shell
zkvyper 'Simple.vy' --format 'combined_json'
```

The mode-altering CLI options are mutually exclusive. This means that only one of the options below can be enabled at a time:
- `--format` / `-f`
- `--llvm-ir`
- `--eravm-assembly`
- `--disassemble`



## *zkvyper* Compilation Settings

The options in this section are only configuring the *zkvyper* compiler and do not affect the underlying *vyper* compiler.



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



### `--metadata-hash`

Specifies the hash function used for project metadata appended to the end of bytecode.

<div class="warning">
For security reasons, the source code of all input Vyper contracts is hashed together, so a change in one contract will affect the metadata hash appended to the bytecode of all contracts, even if there are no dependency relations between them.
It may be changed in the future, so each contract will be hashed separately, as it is done by the *vyper* compiler.
</div>

The following values are allowed: `none`, `ipfs`.

The default value is `ipfs`.

> EraVM requires its bytecode size to be an odd number of 32-byte words. If the size after appending the hash does not satisfy this requirement, the metadata is *prepended* with zeros.

Usage:

```shell
zkvyper 'Simple.vy' --metadata-hash 'ipfs'
```

Output:

```text
Contract `Simple.vy`:
0x0000000100200190000000110000c13d0000000a001001980000001d0000613d000000000101043b000000e0011002700000000b0010009c000000160000613d0000000c0010009c0000001d00
...
a2646970667358221220cabf07f8316a1b55f55aa859b4e4c910f226ab11ab9a786f3a90acb586be0406657679706572781a7a6b76797065723a312e352e31303b76797065723a302e342e30004c
```

The byte array starting with `a2` at the end of the bytecode is a CBOR-encoded compiler version data and an optional metadata hash.

JSON representation of a CBOR payload:

```javascript
{
    // Optional: included if `--metadata-hash` is set to `ipfs`.
    "ipfs": "1220cabf07f8316a1b55f55aa859b4e4c910f226ab11ab9a786f3a90acb586be0406",

    // Required: consists of semicolon-separated pairs of colon-separated compiler names and versions.
    // `zkvyper:<version>` is always included.
    // `vyper:<version>` is included for Vyper contracts, but not included for LLVM IR and EraVM assembly contracts.
    "vyper": "zkvyper:1.5.10;vyper:0.4.1"
}
```

For more information on these formats, see the [CBOR](https://cbor.io/) and [IPFS](https://docs.ipfs.tech/) documentation.



### `--no-bytecode-metadata`

Disables the CBOR metadata that is appended at the end of bytecode. This option is useful for debugging and research purposes.

> It is not recommended to use this option in production, as it is not possible to verify contracts deployed without metadata.

Usage:

```shell
zkvyper 'Simple.vy' --no-bytecode-metadata
```



### `--suppress-warnings`

Tells the compiler to suppress specified warnings. The option accepts multiple string arguments, so make sure they are properly separated by whitespace.

Only one warning can be suppressed with this option: [`txorigin`](https://docs.zksync.io/build/tooling/foundry/migration-guide/testing#origin-address).

Usage:

```shell
zkvyper 'Simple.vy' --suppress-warnings 'txorigin'
```



### `--llvm-options`

Specifies additional options for the LLVM framework. The argument must be a single quoted string following a `=` separator.

Usage:

```shell
zkvyper 'Simple.vy' --llvm-options='-eravm-jump-table-density-threshold=10'
```

> The `--llvm-options` option is experimental and must only be used by experienced users. All supported options will be documented in the future.



## *vyper* Compilation Settings

The options in this section are only configuring *vyper*, so they are passed directly to its child process, and do not affect the *zkvyper* compiler.



### `--evm-version`

Specifies the EVM version *vyper* will produce artifacts for. Only LLL IR is known to be affected by this option. For instance, if the EVM version is set to *cancun*, the LLL IR may contain `mcopy` instructions.

> EVM version only affects IR artifacts produced by *vyper* and does not affect EraVM bytecode produced by *zkvyper*.

The following values are allowed by *zkvyper*:
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

> However, have in mind that many of them are not supported by *vyper*, or only supported by its newer versions.
> For instance, the `--help` message of *vyper* v0.4.x only declares the following EVM versions as supported: *london*, *paris*, *shanghai*, *cancun*.
> For the full list of supported EVM versions, refer to [the official *vyper* documentation](https://docs.vyperlang.org/en/stable/).

Usage:

```shell
zkvyper 'Simple.vy' --evm-version 'cancun'
```



### `--disable-vyper-optimizer`

Disables the optimizer of the *vyper* compiler.

The optimizer is enabled by default for *vyper* v0.3.x. For *vyper* v0.4.x it is disabled and cannot be enabled, as the optimized LLL IR is not compatible with *zkvyper*.

> *zkvyper* relies on the LLVM optimizer, so the *vyper* optimizer is not affecting the size or performance of the final bytecode significantly.

Usage:

```shell
zkvyper 'Simple.vy' --disable-vyper-optimizer
```



### `--enable-decimals`

Enables [decimals](https://docs.vyperlang.org/en/stable/types.html#decimals) in *vyper* v0.4.x.

Usage:

```shell
zkvyper 'Simple.vy' --enable-decimals
```



### `--search-paths`

Passes additional [search paths](https://docs.vyperlang.org/en/stable/structure-of-a-contract.html#searching-for-imports) to *vyper*.

Usage:

```shell
zkvyper 'Simple.vy' --search-paths '/path/to/libraries-1/' '/path/to/libraries-2/'
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
zkvyper --llvm-ir './Simple.ll'
```

Output:

```text
Contract `<absolute-path>/Simple.ll`:
0x000000000002004b000000070000613d000000200100003900000000001004...
```



### `--eravm-assembly`

Enables the EraVM Assembly mode. In this mode, input is expected to be in the EraVM assembly language. The output works the same way as with Vyper input.

EraVM assembly is a representation the closest to EraVM bytecode. It is not recommended to write EraVM assembly manually, but it can be even more useful for debugging and optimization purposes than LLVM IR.

For the EraVM assembly specification, visit the [EraVM documentation](https://docs.zksync.io/zk-stack/components/compiler/specification/binary-layout).

Usage:

```shell
zkvyper --eravm-assembly './Simple.zasm'
```

Output:

```text
Contract `<absolute-path>/Simple.zasm`:
0x000000000120008c000000070000613d00000020010000390000000000100435000000000001043500000005010000410000000c0001042e0000002a01000039000000000010043500000004010000410000000c0001042e0000000b000004320000000c0001042e0000000d00010430000000000000000000000000000000000000000000000000000000000000000000000020000000000000000000000000000000000000000000000000000000000000004000000000000000000000000043bbf1d8e1b7b1d452f006fe83028ba3b7853f9ea8a4635f4c584fe1dc6429b5
```



## Integrated Tooling

*zkvyper* includes several tools provided by the LLVM framework out of the box, such as disassembler and linker. The following sections describe the usage of these tools.



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
zkvyper 'Simple.vy' --debug-output-dir './debug/'
ls './debug/'
```

Output:

```text
<absolute-path-with-underscores>_Simple.vy.lll
<absolute-path-with-underscores>_Simple.vy.runtime.optimized.ll
<absolute-path-with-underscores>_Simple.vy.runtime.unoptimized.ll
<absolute-path-with-underscores>_Simple.vy.zasm
```

The output file name is constructed as follows: `<AbsoluteContractPathWithUnderscores>_<ContractName>.<Modifiers>.<Extension>`.



### `--llvm-verify-each`

Enables the verification of the LLVM IR after each optimization pass. This option is useful for debugging and research purposes.

Usage:

```shell
zkvyper 'Simple.vy' --llvm-verify-each
```



### `--llvm-debug-logging`

Enables the debug logging of the LLVM IR optimization passes. This option is useful for debugging and research purposes.

Usage:

```shell
zkvyper 'Simple.vy' --llvm-debug-logging
```