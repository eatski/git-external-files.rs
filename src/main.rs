use std::{fs::{File, read_to_string}, io::Write};

use git_external_files_lib::{input::DependenciesJson, fetch::fetch_row_content};
#[tokio::main]
async fn main() -> Result<(),Box<dyn std::error::Error>>{
    let inputs = read_to_string("./sample.json")?;
    let dep_json : DependenciesJson = serde_json::from_str(inputs.as_str())?;
    for (name,dependency) in dep_json.dependencies {
        let res = fetch_row_content(dependency).await?;
        let mut file = File::create(format!("./output/{}",name))?;
        file.write_all(&res.as_bytes())?;
        file.flush()?;
    } 
    Ok(())
}