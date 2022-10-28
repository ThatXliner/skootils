use reqwest;
use std::collections::HashMap;
use tl;
const BASE_URL: &str = "https://learn.vcs.net";
pub fn add(left: usize, right: usize) -> usize {
    left + right
}
struct Session {
    client: reqwest::Client,
}
impl Session {
    async fn new(username: String, password: String) {
        let client = reqwest::Client::new();
        let dom = tl::parse(
            client
                .get(BASE_URL + "/login/index.php")
                .send()
                .await?
                .text()
                .await?
                .to_str(),
            tl::ParserOptions::default(),
        )
        .expect("Could not parse document");
        // get login token from BASE_URL
        let login_token = dom
            .query_selector(r#"[name="logintoken"]"#)
            .and_then(|mut iter| iter.next())
            .expect("No login token found")
            .get(dom.parser())
            .unwrap()
            .as_tag()
            .unwrap()
            .attributes()
            .get("value")
            .expect("No login token found");
        let mut auth = HashMap::new();
        auth.insert("username", username);
        auth.insert("password", password);
        auth.insert("logintoken", login_token);
        client
            .post(BASE_URL + "/login/index.php")
            .form(&auth)
            .send()
            .await?;
        Session { client }
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
