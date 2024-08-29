//!
//! The Vyper compiler unit tests for warnings.
//!

#[cfg(not(target_arch = "aarch64"))]
#[test]
fn tx_origin_0_3_3() {
    tx_origin(semver::Version::new(0, 3, 3));
}
#[test]
fn tx_origin_0_3_9() {
    tx_origin(semver::Version::new(0, 3, 9));
}
#[test]
fn tx_origin_0_3_10() {
    tx_origin(semver::Version::new(0, 3, 10));
}
#[test]
fn tx_origin_0_4_0() {
    tx_origin(semver::Version::new(0, 4, 0));
}

fn tx_origin(version: semver::Version) {
    assert!(super::check_warning(
        "tests/regression/contracts/tx_origin.vy",
        &version,
        "You are checking for 'tx.origin', which may lead to unexpected behavior."
    )
    .expect("Test failure"));
}
