use std::error::Error;
use std::fs;
use std::path::Path;
use std::str::FromStr;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let db_dir = Path::new(&manifest_dir).join("../target/sqlx-dev");
    fs::create_dir_all(&db_dir)?;

    let db_file = db_dir.join("dim_dev.db");
    let db_file = db_file.to_str().unwrap();
    println!("cargo::rustc-env=DATABASE_URL=sqlite://{db_file}");
    println!("cargo::warning=Generating {db_file:?} from latest migrations.");

    let pool = sqlx::sqlite::SqlitePoolOptions::new()
        .connect_with(
            sqlx::sqlite::SqliteConnectOptions::from_str(db_file)?.create_if_missing(true),
        )
        .await?;

    sqlx::migrate!().run(&pool).await.map_err(|e| {
        println!("cargo::error=Migration failed: {e:?}");
        e
    })?;

    println!("cargo::warning=Database ready at {db_file}.");

    println!("cargo::rerun-if-changed=build.rs");
    println!("cargo::rerun-if-changed=migrations/");

    Ok(())
}
