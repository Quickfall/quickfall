use std::process::Command;

fn main() {
    let out = Command::new("git")
        .args(["rev-parse", "HEAD"])
        .output()
        .unwrap();

    let git_hash = String::from_utf8(out.stdout).unwrap();

    println!("cargo:rustc-env=GIT_HASH={}", git_hash.trim());
}
