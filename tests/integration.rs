use ignore_check::{Ignore, ignored};

#[test]
fn ignored_file() {
    assert!(ignored("target").unwrap());
}

#[test]
fn not_ignored_file() {
    assert!(!ignored("src/lib.rs").unwrap());
}

#[test]
fn reuse_ignore() {
    let ignore = Ignore::default();
    assert!(ignore.check("target"));
    assert!(!ignore.check("src/lib.rs"));
}
