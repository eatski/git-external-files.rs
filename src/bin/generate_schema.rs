use std::{fs::File, io::Write};

use git_external_files_lib::input::{get_schema};

#[tokio::main]
async fn main() -> Result<(),Box<dyn std::error::Error>>{
    let mut file = File::create("./schema.json")?;
    let schema = get_schema()?;
    file.write_all(schema.as_bytes())?;
    file.flush()?;
    Ok(())
}