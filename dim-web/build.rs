use std::path::Path;
use std::process::Command;

fn main() {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let workspace_root = Path::new(&manifest_dir).join("..");

    // Point sqlx macros at the shared DB created by dim-database's build script.
    let db_file = workspace_root.join("target/sqlx-dev/dim_dev.db");
    println!("cargo::rustc-env=DATABASE_URL=sqlite://{}", db_file.to_str().unwrap());

    // Build the UI.
    let ui_dir = workspace_root.join("ui");
    let ui_dir_str = ui_dir.to_str().unwrap();

    let install_ok = Command::new("yarn")
        .args(["--cwd", ui_dir_str, "install", "--frozen-lockfile"])
        .status()
        .map(|s| s.success())
        .unwrap_or(false);

    if install_ok {
        let build_ok = Command::new("yarn")
            .args(["--cwd", ui_dir_str, "build"])
            .status()
            .map(|s| s.success())
            .unwrap_or(false);
        if !build_ok {
            println!("cargo::warning=yarn build failed — UI will not be embedded.");
        }
    } else {
        println!("cargo::warning=yarn install failed — UI will not be embedded.");
    }

    if workspace_root.join("ui/build").exists() {
        println!("cargo::rustc-cfg=feature=\"embed_ui\"");
    } else {
        println!("cargo::warning=`ui/build` does not exist — UI will not be embedded.");
    }

    // Version metadata from git.
    let git_tag = Command::new("git")
        .args(["describe", "--abbrev=0"])
        .output()
        .map(|o| String::from_utf8(o.stdout).unwrap_or_default())
        .unwrap_or_default();
    println!("cargo::rustc-env=GIT_TAG={}", git_tag.trim());

    let git_sha = Command::new("git")
        .args(["rev-parse", "HEAD"])
        .output()
        .map(|o| String::from_utf8(o.stdout).unwrap_or_default())
        .unwrap_or_default();
    println!("cargo::rustc-env=GIT_SHA_256={}", git_sha.trim());

    // Rerun on UI source changes, commits, or branch switches.
    println!("cargo::rerun-if-changed=build.rs");
    println!("cargo::rerun-if-changed=../ui/src");
    println!("cargo::rerun-if-changed=../ui/package.json");
    println!("cargo::rerun-if-changed=../ui/yarn.lock");
    println!("cargo::rerun-if-changed=../.git/HEAD");
    println!("cargo::rerun-if-changed=../.git/refs/heads/");
}
