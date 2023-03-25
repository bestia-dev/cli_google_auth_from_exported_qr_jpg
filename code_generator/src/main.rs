use std::io::Result;
fn main() -> Result<()> {
    println!("build.rs start");
    let mut config = prost_build::Config::new();
    config.out_dir("/home/rustdevuser/rustprojects/cli_google_auth_from_exported_qr_jpg/code_generator");
    config.compile_protos(&["src/gauth_migration.proto"], &["src"])?;
    
    println!("build.rs end");
    Ok(())
}
