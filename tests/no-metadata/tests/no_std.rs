#![no_std]

use assert2::assert;

#[test]
fn test() {
	assert!(const { const_pkg_version::major!() } == 1024);
	assert!(const { const_pkg_version::minor!() } == 9);
	assert!(const { const_pkg_version::patch!() } == 591);
	assert!(const { const_pkg_version::pre_release!() } == None);
	assert!(const { const_pkg_version::build_metadata!() } == None);
	assert!(const { const_pkg_version::version!() } == const_pkg_version::Version {
		major: 1024,
		minor: 9,
		patch: 591,
		pre_release: None,
		build_metadata: None,
	});
}
