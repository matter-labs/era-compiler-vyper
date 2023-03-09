# The `zkvyper` changelog

## [1.3.3] - 2023-03-09

### Added

- The contract metadata hash to the end of bytecode
- More optimizations

### Changed

- The optimizer settings to support multiple modes
- The optimizer now optimizes for performance instead of size by default

## [1.3.2] - 2023-02-14

### Added

- The LLVM build commit ID to the `--version` output
- More LLVM optimizations

### Removed

- The `long_version` field from the combined JSON output

### Fixed

- Calls now only copy `min(output_size, return_data_size)` of the return data

## [1.3.1] - 2023-02-06

### Changed

- Some ABI data layout parameters

## [1.3.0] - 2023-02-02

### Added

- The `--llvm-ir` compilation mode

### Changed

- System contract calls now use remaining ergs instead of 0
- The LLVM optimization manager to the new one
- The contract ABI to match that of zkEVM v1.3
- Moved the event decoding to the system contracts
- Simplified the CLI arguments used for debugging

### Removed

- The `extcodesize` check at the beginning of runtime code

### Fixed

- The non-zero initial return data size value
- `msg.value >= 2^128` now set the call status code to zero
- `BALANCE` now returns 0 if `address >= 2^160`
- `KECCAK256` now returns an empty error in case of revert
- `SIGNEXTEND` now returns the original value if `bytes >= 31`
- `CODESIZE` is forbidden in Yul runtime code
- `RETURNDATACOPY` now reverts on attempt to copy from beyond the return data
- `RETURN` and `REVERT` offsets and lengths are now clamped to `2^32 - 1`
- Only block hashes of the last 256 blocks are now accessible
- `ptr.pack` is not optimized out by LLVM anymore

## [1.2.2] - 2022-12-16

### Added

- More LLVM optimizations

### Changed

- Updated LLVM to v15.0.4

### Fixed

- The crash with some uncovered LLVM IR nodes
- The missing check for `msg.value` > `2^128 - 1`

## [1.2.1] - 2022-12-01

### Added

- The option to dump IRs to files
- More contract size optimizations
- The Windows platform support

### Changed

- The `CODECOPY` instruction now produces a compile-time error in the runtime code
- The `CALLCODE` instruction now emits a compile-time error

### Fixed

- The `BYTE` instruction overflow
- The forwarder constructor unhandled error

## [1.2.0] - 2022-10-10

### Added

- Many improvements for the memory security and EVM-compatibility
- Optimizations for the heap allocation
- Support for optimizations for the calldata and returndata forwarding
- More LLVM optimizations

### Changed

- System contract calls now require a system call flag
- The handling of `msg.value` is made more robust
- Failed system contract calls now do bubble-up the reverts

## [1.1.4] - 2022-09-02

### Added

- The compiler versions to the output JSON

### Changed

- Unsupported instructions `PC`, `EXTCODECOPY`, `SELFDESTRUCT` now produce compile-time errors

### Fixed

- Bloating the array of immutables with zero values

## [1.1.3] - 2022-08-16

### Added

- Support for the `BASEFEE` instruction

## [1.1.2] - 2022-08-08

### Added

- Better compatibility of opcodes `GASLIMIT`, `GASPRICE`, `CHAINID`, `DIFFICULTY`, `COINBASE` etc.

## [1.1.1] - 2022-07-28

### Added

- Support for the *SELECT* instruction, used by min/max built-ins

## [1.1.0] - 2022-07-16

### Added

- The initial release
