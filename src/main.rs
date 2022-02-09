use std::{fs::{File, read_to_string}, io::Write};
use clap::Parser;
use std::path::Path;

use git_external_files_lib::{input::DependenciesJson, fetch::fetch_row_content};


/// Simple program to greet a person
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(short)]
    output: String,
    #[clap(short, default_value = "external.json")]
    config: String,
}


#[tokio::main]
async fn main() -> Result<(),Box<dyn std::error::Error>>{
    let args = Args::parse();
    let output = Path::new(args.output.as_str());
    let inputs = read_to_string(args.config.as_str())?;
    let dep_json : DependenciesJson = serde_json::from_str(inputs.as_str())?;
    for (name,dependency) in dep_json.dependencies {
        let res = fetch_row_content(dependency).await?;
        let mut file = File::create(output.join(name.as_str()))?;
        file.write_all(&res.as_bytes())?;
        file.flush()?;
    } 
    Ok(())
}