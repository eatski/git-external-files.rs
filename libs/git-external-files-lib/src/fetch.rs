use crate::input::{Dependency};

pub async fn fetch_row_content(dependency: Dependency) -> Result<String,Box<dyn std::error::Error>>{
    let client = reqwest::Client::new();
    let target = format!(
        "https://raw.githubusercontent.com/{}/{}{}",
        dependency.repository, 
        dependency.version,
        dependency.path
    );
    let content = client
        .get(target)
        .header(reqwest::header::USER_AGENT, "TODO")
        .send()
        .await?
        .text()
        .await?;
    Ok(content)
}

#[cfg(test)]
mod test {
    use crate::input::Dependency;
    use super::fetch_row_content;

    #[tokio::test]
    async fn it_works() -> Result<(),Box<dyn std::error::Error>>{
        let res = fetch_row_content(Dependency { 
            repository: "itskihaga/testee".to_string(), 
            version: "v0.0.1".to_string() , 
            path: "/README.md".to_string() }
        ).await?;
        assert_eq!(res,"# testee".to_string());
        Ok(())
    }
}