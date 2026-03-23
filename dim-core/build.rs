use std::path::Path;

fn main() {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let workspace_root = Path::new(&manifest_dir).join("..");

    let db_file = workspace_root.join("target/sqlx-dev/dim_dev.db");
    println!("cargo::rustc-env=DATABASE_URL=sqlite://{}", db_file.to_str().unwrap());

    println!("cargo::rerun-if-changed=build.rs");
}
