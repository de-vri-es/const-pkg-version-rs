# const-pkg-version

Constant expression macros for your package version.

## Example

```rust
use const_pkg_version::Version;
const VERSION_FULL: Version = const_pkg_version::version!();

const VERSION_MAJOR: u32 = const_pkg_version::major!();
const VERSION_MINOR: u32 = const_pkg_version::minor!();
const VERSION_PATCH: u32 = const_pkg_version::patch!();
const VERSION_PRE: Option<&str> = const_pkg_version::pre_release!();
const BUILD_METADATA: Option<&str> = const_pkg_version::build_metadata!();
```

## Features

* `debug`: Implements [`Debug`] for [`Version`] (enabled by default).
* `defmt`: Implements [`defmt::Format`] for [`Version`].
* `serde`: Implements [`serde::Deserialize`] and [`serde::Serialize`] for [`Version`].
* `semver`: Implements [`TryFrom<crate::Version>`][TryFrom] for [`semver::Version`].

[`Version`]: https://docs.rs/const-pkg-version/latest/const_pkg_version/struct.Version.html
[`Debug`]: https://doc.rust-lang.org/stable/std/fmt/trait.Debug.html
[`defmt::Format`]: https://docs.rs/defmt/latest/defmt/trait.Format.html
[`serde::Deserialize`]: https://docs.rs/serde/latest/serde/trait.Deserialize.html
[`serde::Serialize`]: https://docs.rs/serde/latest/serde/trait.Serialize.html
[`semver::Version`]: https://docs.rs/semver/latest/semver/struct.Version.html
