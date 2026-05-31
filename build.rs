use std::process::Command;

fn main() {
    let output = Command::new("git")
        .args(["rev-parse", "HEAD"])
        .output()
        .expect("failed to run `git`");

    let commit = String::from_utf8(output.stdout)
        .unwrap_or_default()
        .trim()
        .to_string();

    println!("cargo:rustc-env=GIT_COMMIT_HASH={}", commit);
    println!("cargo:rerun-if-changed=.git/HEAD");
    println!("cargo:rerun-if-changed=.git/refs/");
}
