// Integration test for SigmaOS-security-utility
// Verifies the crate builds and links as a dependency.
#![allow(unused_imports)]

use security_utility as _;

#[test]
fn crate_links_and_runs() {
    assert_eq!(2 + 2, 4);
}
