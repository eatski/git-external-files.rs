use std::{collections::HashMap};

use schemars::{JsonSchema, schema_for};
use serde::Deserialize;
use serde_json::{to_string};

#[derive(JsonSchema,Deserialize)]
pub struct DependenciesJson {
    pub dependencies: HashMap<String,Dependency>
}

#[derive(JsonSchema,Deserialize)]
pub struct Dependency {
    pub repository: String,
    pub version: String,
    pub path: String
}

pub fn get_schema() -> Result<String,std::io::Error> {
    Ok(to_string(&schema_for!(DependenciesJson))?)
}