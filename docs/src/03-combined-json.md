# Combined JSON

Combined JSON is an I/O mode designed as a convenient way of using *zkvyper* from tools that calls it as a child process. In this mode, input data is provided by the user via CLI, and JSON output can be easily read by both humans and programs calling *zkvyper* as a child process.



## Usage

To use combined JSON, pass the `-f combined_json` option to *zkvyper*:

```shell
zkvyper './MyContract.vy' -f 'combined_json'
```

<div class="warning">
It is only possible to use Combined JSON with Vyper input, so the path to <b>vyper</b> must be always provided to *zkvyper*.
Support for other languages is planned for future releases.
</div>



## Output Format

The format below is a modification of the original combined JSON [output](https://docs.soliditylang.org/en/latest/using-the-compiler.html#output-description) format implemented by *vyper*. It means that there are:
- *zkvyper*-specific options that are not present in the original format: they are marked as *zkvyper* in the specification below.
- *vyper*-specific options that are not supported by *zkvyper*: they are not mentioned in the specification below.

```javascript
{
  // Required: Contract outputs.
  "contracts": {
    "MyContract.vy": {
      // Optional: Emitted if "hashes" selector is provided.
      "hashes": {/* ... */},
      // Optional: Emitted if "abi" selector is provided.
      "abi": [/* ... */],
      // Optional: Emitted if "metadata" selector is provided.
      "metadata": "/* ... */",
      // Optional: Emitted if "devdoc" selector is provided.
      "devdoc": {/* ... */},
      // Optional: Emitted if "userdoc" selector is provided.
      "userdoc": {/* ... */},
      // Optional: Emitted if "storage-layout" selector is provided.
      "storage-layout": {/* ... */},
      // Optional: Emitted if "transient-storage-layout" selector is provided.
      "transient-storage-layout": {/* ... */},
      // Required: Bytecode is always emitted.
      "bin": "0000008003000039000000400030043f0000000100200190000000130000c13d...",
      // Required: Bytecode is always emitted.
      "bin-runtime": "0000008003000039000000400030043f0000000100200190000000130000c13d...",
      // Required, zkvyper: Mapping between bytecode hashes and full contract identifiers (e.g. "MyContract.vy").
      "factory-deps": {/* ... */}
      // Required, zkvyper: Binary object format.
      // Tells whether the bytecode has been linked.
      // Possible values: "elf" (unlinked), "raw" (linked).
      "objectFormat": "elf"
    }
  },
  // Required: Version of vyper.
  "version": "0.4.0",
  // Required, zkvyper: Version of zkvyper.
  "zk_version": "1.5.8"
}
```