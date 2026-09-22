use libaustralis::aurora::assets::{Asset, AssetType};
use libaustralis::utils::TextureFormat;
use std::env;

fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let args: Vec<String> = env::args().collect();

    if args.len() != 3 {
        eprintln!("Usage: aurora-cover-maker <input-image> <output-asset>");
        std::process::exit(1);
    }

    let input = &args[1];
    let output = &args[2];

    let mut asset = Asset::new();

    asset.import_image(
        input,
        AssetType::Boxart,
        Some(TextureFormat::BC3),
    )?;

    asset.write_file(output)?;

    println!("Created: {}", output);
    Ok(())
}
