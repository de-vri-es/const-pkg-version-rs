use assert2::assert;

#[test]
fn test() {
	assert!(const { const_pkg_version::major!() } == 42);
	assert!(const { const_pkg_version::minor!() } == 129);
	assert!(const { const_pkg_version::patch!() } == 316);
	assert!(const { const_pkg_version::pre_release!() } == Some("pre-thingy"));
	assert!(const { const_pkg_version::build_metadata!() } == Some("built-with-love"));
	assert!(const { const_pkg_version::version!() } == const_pkg_version::Version {
		major: 42,
		minor: 129,
		patch: 316,
		pre_release: Some("pre-thingy"),
		build_metadata: Some("built-with-love"),
	});
}
