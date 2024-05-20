use std::fs;

fn main() {
    let dir = fs::canonicalize(env!("CARGO_MANIFEST_DIR"))
        .unwrap()
        // Go back a directory from `./host`
        .parent()
        .unwrap()
        // Use guest directory.
        .join("guest");

    let mut cmd = risc0_build::cargo_command("build", &[]);
    cmd.args([
        "--manifest-path",
        dir.join("Cargo.toml").as_os_str().to_str().unwrap(),
        // TODO might want to conditionally build in release
        "--release",
        // "--target-dir",
        // dir.join("test").as_os_str().to_str().unwrap(),
    ]);

    // Execute the build command.
    cmd.status().unwrap();
}
