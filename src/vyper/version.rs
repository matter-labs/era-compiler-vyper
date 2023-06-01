//!
//! The Vyper compiler version.
//!

///
/// The Vyper compiler version.
///
pub struct Version {
    /// The long version string.
    pub long: String,
    /// The short `semver`.
    pub default: semver::Version,
}

impl Version {
    ///
    /// A shortcut constructor.
    ///
    pub fn new(long: String, default: semver::Version) -> Self {
        Self { long, default }
    }
}
