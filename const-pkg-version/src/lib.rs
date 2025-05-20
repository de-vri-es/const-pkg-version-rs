//! Constant expressions for your package version.
//!
//! # Example
//!
//! ```
//! use const_pkg_version::Version;
//! const VERSION_FULL: Version = const_pkg_version::version!();
//!
//! const VERSION_MAJOR: u32 = const_pkg_version::major!();
//! const VERSION_MINOR: u32 = const_pkg_version::minor!();
//! const VERSION_PATCH: u32 = const_pkg_version::patch!();
//! const VERSION_PRE: Option<&str> = const_pkg_version::pre_release!();
//! const BUILD_METADATA: Option<&str> = const_pkg_version::build_metadata!();
//! ```
//!
//! # Features
//!
//! * `debug`: Implements [`Debug`] for [`Version`] (enabled by default).
//! * `defmt`: Implements [`defmt::Format`] for [`Version`].
//! * `serde`: Implements [`serde::Deserialize`] and [`serde::Serialize`] for [`Version`].
//! * `semver`: Implements [`TryFrom<crate::Version>`] for [`semver::Version`].
#![no_std]
#![warn(missing_docs)]

#[doc(hidden)]
pub mod __reexports {
    pub use const_pkg_version_macros as macros;
}

/// A full package version (major, minor, patch, pre-release and build-metadata).
#[derive(Copy, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "debug", derive(Debug))]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[cfg_attr(feature = "serde", derive(serde::Deserialize, serde::Serialize))]
pub struct Version<'a> {
    /// The major version of the package.
    pub major: u32,

    /// The minor version of the package.
    pub minor: u32,

    /// The patch version of the package.
    pub patch: u32,

    /// The pre-release version of the package.
    pub pre_release: Option<&'a str>,

    /// The build metadata of the package.
    pub build_metadata: Option<&'a str>,
}

/// Get the major version of your package in a constant expression.
///
/// # Example
/// ```
/// const VERSION_MAJOR: u32 = const_pkg_version::major!();
/// ```
///
/// You can also limit the value to something lower than a `u32`.
/// This will generate a compile error if the version is too high:
/// ```
/// const VERSION_MAJOR: u8 = const_pkg_version::major!();
/// ```
#[macro_export]
macro_rules! major {
    () => {
        $crate::__reexports::macros::major!($crate)
    }
}

/// Get the minor version of your package in a constant expression.
///
/// # Example
/// ```
/// const VERSION_MINOR: u32 = const_pkg_version::minor!();
/// ```
///
/// You can also limit the value to something lower than a `u32`.
/// This will generate a compile error if the version is too high:
/// ```
/// const VERSION_MINOR: u8 = const_pkg_version::minor!();
/// ```
#[macro_export]
macro_rules! minor {
    () => {
        $crate::__reexports::macros::minor!($crate)
    }
}

/// Get the patch version of your package in a constant expression.
///
/// # Example
/// ```
/// const VERSION_PATCH: u32 = const_pkg_version::patch!();
/// ```
///
/// You can also limit the value to something lower than a `u32`.
/// This will generate a compile error if the version is too high:
/// ```
/// const VERSION_PATCH: u8 = const_pkg_version::patch!();
/// ```
#[macro_export]
macro_rules! patch {
    () => {
        $crate::__reexports::macros::patch!($crate)
    }
}

#[macro_export]
#[deprecated(note = "who are you calling a p'takh?")]
#[doc(hidden)]
macro_rules! ptach {
    () => {
        $crate::__reexports::macros::patch!($crate)
    }
}

/// Get the pre-release version of your package in a constant expression.
///
/// Evaluates to `Some(str)` if your package has a pre-release version.
/// Evaluates to `None` otherwise.
///
/// # Example
/// ```
/// const VERSION_PRE: Option<&str> = const_pkg_version::pre_release!();
/// ```
#[macro_export]
macro_rules! pre_release {
    () => {
        $crate::__reexports::macros::pre_release!($crate)
    }
}

/// Get the build metadata of your package in a constant expression.
///
/// Evaluates to `Some(str)` if your package has a build metadata string in the version.
/// Evaluates to `None` otherwise.
///
/// # Example
/// ```
/// const VERSION_PRE: Option<&str> = const_pkg_version::pre_release!();
/// ```
#[macro_export]
macro_rules! build_metadata {
    () => {
        $crate::__reexports::macros::build_metadata!($crate)
    }
}

/// Get the full version information of your package in a constant expression.
///
/// Evaluates to a [`Version`] object.
///
/// # Example
/// ```
/// use const_pkg_version::Version;
/// const VERSION_FULL: Version = const_pkg_version::version!();
/// ```
#[macro_export]
macro_rules! version {
    () => {
        $crate::__reexports::macros::full!($crate)
    }
}

#[cfg(feature = "semver")]
impl<'a> TryFrom<Version<'a>> for semver::Version {
    type Error = semver::Error;

    fn try_from(input: Version<'a>) -> Result<Self, Self::Error> {
        Self::try_from(&input)
    }
}

#[cfg(feature = "semver")]
impl TryFrom<&Version<'_>> for semver::Version {
    type Error = semver::Error;

    fn try_from(input: &Version<'_>) -> Result<Self, Self::Error> {
        Ok(Self {
            major: input.major.into(),
            minor: input.minor.into(),
            patch: input.patch.into(),
            pre: input.pre_release.unwrap_or("").parse()?,
            build: input.build_metadata.unwrap_or("").parse()?,
        })
    }
}
