use thiserror::Error as ThisError;

#[derive(ThisError, Debug)]
pub enum Error {
    #[error("The package version is invalid, it seems that the environment variable CARGO_PKG_VERSION, is not compatible with semver.")]
    InvalidPackageVersion(#[from] semver::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
