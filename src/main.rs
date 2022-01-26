use std::{collections::HashMap, fs::{File, read_to_string}, io::Write};
use schemars::{JsonSchema, schema_for};
use serde::Deserialize;
use serde_json::{to_vec};

#[tokio::main]
async fn main() -> Result<(),Box<dyn std::error::Error>>{

    let client = reqwest::Client::new();
    let inputs = read_to_string("./sample.json")?;
    let dep_json : DependenciesJson = serde_json::from_str(inputs.as_str())?;
    for (name,dependency) in dep_json.dependencies {
        let target = format!("https://raw.githubusercontent.com/{}/{}{}",
        dependency.repository, 
        dependency.version,
        dependency.path);
        println!("{}",target);
        let res = client.get(target)
            .header(reqwest::header::USER_AGENT, "TODO")
            .send()
            .await?
            .bytes()
            .await?;
        let mut file = File::create(format!("./output/{}",name))?;
        file.write_all(&res)?;
        file.flush()?;
    }    
    let mut file = File::create("./dist/schema.json")?;
    let schema = to_vec(&schema_for!(DependenciesJson))?;
    file.write_all(&schema)?;
    file.flush()?;
    Ok(())
}
#[derive(Deserialize,Debug)]
struct GithubTagIO {
    pub name: String
}
#[derive(Deserialize,Debug)]
struct GithubContentIO {
    pub content: String
}


#[derive(JsonSchema,Deserialize)]
struct DependenciesJson {
    pub dependencies: HashMap<String,Dependency>
}

#[derive(JsonSchema,Deserialize)]
struct Dependency {
    pub repository: String,
    pub version: String,
    pub path: String
}