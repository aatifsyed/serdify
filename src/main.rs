use clap::Parser;
use serde_json::Value;
use std::{
    fs, io,
    path::{Path, PathBuf},
};

fn serdify(r: impl io::Read) -> anyhow::Result<String> {
    let v = serde_json::from_reader::<_, Value>(r)?;
    let s = serde_json::to_string_pretty(&v)?;
    Ok(s)
}

#[derive(Parser)]
struct Args {
    #[arg(name("FILE"))]
    files: Vec<PathBuf>,
}

fn main() -> anyhow::Result<()> {
    let Args { files } = Args::parse();
    if files.is_empty() || (files.len() == 1 && files[0] == Path::new("-")) {
        eprintln!("serdify: <stdin>");
        println!("{}", serdify(io::stdin())?);
    } else {
        for file in files {
            println!("serdify: {}", file.display());
            let s = serdify(fs::File::open(&file)?)?; // mind your drop guards!
            fs::write(file, s)?;
        }
    }
    Ok(())
}
