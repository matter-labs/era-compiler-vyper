//!
//! The Vyper compiler unit tests for warnings.
//!

#![cfg(test)]

#[test]
fn ecrecover() {
    let source_code = r#"
@external
@view
def test(hash: bytes32, v: uint256, r:uint256, s:uint256) -> address:
    return ecrecover(hash, v, r, s)
"#;

    assert!(super::check_warning(
        source_code,
        "Warning: It looks like you are using 'ecrecover' to validate a signature of a user account."
    )
    .expect("Test failure"));
}

#[test]
fn extcodesize() {
    let source_code = r#"
@external
def test(addr: address) -> bool:
    return addr.is_contract
"#;

    assert!(super::check_warning(
        source_code,
        "Warning: Your code or one of its dependencies uses the 'extcodesize' instruction, which is"
    )
    .expect("Test failure"));
}

#[test]
fn tx_origin() {
    let source_code = r#"
@external
def test() -> address:
    return tx.origin
"#;

    assert!(super::check_warning(
    source_code,
    "Warning: You are checking for 'tx.origin' in your code, which might lead to unexpected behavior."
)
.expect("Test failure"));
}
