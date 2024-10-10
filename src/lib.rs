mod error;

use semver::Version;
use std::env;

pub use error::{Error, Result};

pub const CRATE_VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn crate_version() -> Result<Version> {
    match Version::parse(CRATE_VERSION) {
        Ok(version) => Ok(version),
        Err(err) => Err(Error::InvalidPackageVersion(err)),
    }
}
