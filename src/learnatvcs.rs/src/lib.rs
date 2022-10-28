use anyhow::Result; // TODO: Handle erros normally
use reqwest;
use std::collections::HashMap;
use tl;
const BASE_URL: &str = "https://learn.vcs.net";
pub fn add(left: usize, right: usize) -> usize {
    left + right
}
pub struct Session {
    client: reqwest::Client,
}
impl Session {
    pub async fn new(username: String, password: String) -> Result<Self> {
        let client = reqwest::Client::new();
        let i = client
            .get(format!("{}/login/index.php", &BASE_URL))
            .send()
            .await?
            .text()
            .await?;
        let dom = tl::parse(i.as_str(), tl::ParserOptions::default())?;
        // get login token from BASE_URL
        let login_token = dom
            .query_selector(r#"[name="logintoken"]"#)
            .and_then(|mut iter| iter.next())
            .unwrap()
            .get(dom.parser())
            .unwrap()
            .as_tag()
            .unwrap()
            .attributes()
            .get("value")
            .unwrap()
            .unwrap()
            .as_utf8_str()
            .into_owned();
        let mut auth = HashMap::new();
        auth.insert("username", username);
        auth.insert("password", password);
        auth.insert("logintoken", login_token);
        client
            .post(format!("{}/login/index.php", BASE_URL))
            .form(&auth)
            .send()
            .await?;
        Ok(Session { client })
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
