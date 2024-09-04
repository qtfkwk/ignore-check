use ignore_check::ignored;

#[test]
fn ignored_file() {
    assert!(ignored("target").unwrap());
}

#[test]
fn not_ignored_file() {
    assert!(!ignored("src/lib.rs").unwrap());
}
