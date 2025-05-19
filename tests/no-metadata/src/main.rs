const MAJOR: u32 = const_pkg_version::major!();
const MINOR: u32 = const_pkg_version::minor!();
const PATCH: u32 = const_pkg_version::patch!();
const PRE_RELEASE: Option<&str> = const_pkg_version::pre_release!();
const BUILD_METADATA: Option<&str> = const_pkg_version::build_metadata!();
const VERSION: const_pkg_version::Version = const_pkg_version::version!();

fn main() {
    dbg!(MAJOR);
    dbg!(MINOR);
    dbg!(PATCH);
    dbg!(PRE_RELEASE);
    dbg!(BUILD_METADATA);
    dbg!(VERSION);
}
