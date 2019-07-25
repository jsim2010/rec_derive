use std::fs;
use std::process::Command;

#[test]
fn readme_updated() {
    let actual_readme = fs::read_to_string("README.md").expect("reading README");
    let desired_readme = String::from_utf8_lossy(
        &Command::new("cargo")
            .arg("readme")
            .output()
            .expect("executing 'cargo readme'")
            .stdout,
    )
    .into_owned();

    assert_eq!(actual_readme, desired_readme);
}

#[test]
fn fmt_ok() {
    Command::new("cargo")
        .args(&["fmt", "--", "--check"])
        .status()
        .expect("cargo fmt command failed");
}
