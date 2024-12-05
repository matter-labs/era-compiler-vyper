# Combined JSON

Combined JSON is an I/O mode designed as a convenient way of using *zkvyper* from tooling that call it as a child process. In this mode, input data is provided via the CLI, and JSON output can be easily read by both humans and programs.



## Usage

To enable combined JSON, pass the `-f combined_json` option to *zkvyper*:

```shell
zkvyper './MyContract.vy' -f 'combined_json'
```

<div class="warning">
It is only possible to use Combined JSON with Vyper input, so the path to <b>vyper</b> must be always provided to *zkvyper*.
Support for other languages is planned for future releases.
</div>



## Output Format

The format below is a modification of the original combined JSON output format implemented by *vyper*. It means that there are:
- *zkvyper*-specific options that are not present in the original format: they are marked as *zkvyper* in the specification below.
- *vyper*-specific options that are not supported by *zkvyper*: they are not mentioned in the specification below.

> *zkvyper* always produces absolute contract paths in combined JSON output. It was done for unification purposes, as various versions of *vyper* are known to produce either absolute or relative paths.

```javascript
{
  "<absolute-path>/MyContract.vy": {
    // The bytecode as a hexadecimal string.
    "bin": "0000008003000039000000400030043f0000000100200190000000130000c13d...",
    // For EraVM, same as "bin".
    "bin-runtime": "0000008003000039000000400030043f0000000100200190000000130000c13d...",
    // Hashes of function signatures.
    "method_identifiers": {/* ... */},
    // Contract ABI specification.
    "abi": [/* ... */],
    // Storage layout.
    "layout": {/* ... */},
    // Developer documentation.
    "devdoc": {/* ... */},
    // User documentation.
    "userdoc": {/* ... */},
    // zkvyper: Optional bytecode hash of the minimal proxy, if the contract uses "create_minimal_proxy_to".
    "factory_deps": {
      "01000035999a1d871cf4d876ed735fa6a8f3bbeb3f94d210bf4520ed94f35654": "__VYPER_MINIMAL_PROXY_CONTRACT"
    },
    // zkvyper: Warnings produced during compilation.
    "warnings": [/* ... */]
  },
  // zkvyper: Metadata preimage whose hash can be appended to the bytecode.
  "extra_data": {
    // LLVM optimizer settings.
    // The format is "M{level}B{level}", where M = LLVM middle-end, B = LLVM back-end, and levels: 0-3 | s | z.
    "optimizer_settings": "M3B3",
    // LLVM extra options.
    "llvm_options": [/* ... */],
    // EVM version passed to the vyper compiler.
    "evm_version": "cancun",
    // Byte-array hash of the whole project's source code.
    "source_code_hash": [147,242,126,144,/* ... */22,153,132,218],
    // Version of vyper.
    "source_version": "0.4.0",
    // Version of zkvyper.
    "zk_version": "1.5.8"
  },
  // Version of vyper.
  "version": "0.4.0",
  // zkvyper: Version of zkvyper.
  "zk_version": "1.5.8"
}
```