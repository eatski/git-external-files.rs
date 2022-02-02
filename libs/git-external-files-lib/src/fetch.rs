use crate::input::{Dependency};

pub async fn fetch_row_content(dependency: Dependency) -> Result<String,Box<dyn std::error::Error>>{
    let client = reqwest::Client::new();
    let target = format!("https://raw.githubusercontent.com/{}/{}{}",
        dependency.repository, 
        dependency.version,
        dependency.path);
    let content = client.get(target)
        .header(reqwest::header::USER_AGENT, "TODO")
        .send()
        .await?
        .text()
        .await?;
    Ok(content)
}