#[tokio::main]
async fn main() {}

#[cfg(test)]
mod test {
    use super::*;
    use serde::Deserialize;
    use std::collections::HashMap;
    use tokio::sync::OnceCell;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    #[derive(Debug, serde::Deserialize)]
    pub struct Account {
        name: String,
    }

    static HTTP_SERVER: OnceCell<MockServer> = OnceCell::const_new();

    async fn setup() -> MockServer {
        println!("setup the server now");

        let http_server = MockServer::start().await;

        Mock::given(path("/account"))
            .and(method("GET"))
            .respond_with(
                ResponseTemplate::new(200).set_body_raw(
                    r#"
                {"name": "testAccount"}
                "#
                    .as_bytes()
                    .to_owned(),
                    "application/json",
                ),
            )
            .mount(&http_server)
            .await;

        Mock::given(path("/contact"))
            .and(method("GET"))
            .respond_with(
                ResponseTemplate::new(400).set_body_raw(
                    r#"
                {"first name": "eric", "last name": "wang"}
                "#
                    .as_bytes()
                    .to_owned(),
                    "application/json",
                ),
            )
            .mount(&http_server)
            .await;

        Mock::given(path("/account"))
            .and(method("POST"))
            .respond_with(ResponseTemplate::new(200))
            .mount(&http_server)
            .await;

        http_server
    }

    #[tokio::test]
    async fn test_http_get_account_200() {
        let http_server = HTTP_SERVER.get_or_init(setup).await;

        let res: Account = reqwest::get(&format!("{}/account", &http_server.uri()))
            .await
            .unwrap()
            .json::<Account>()
            .await
            .unwrap();

        assert_eq!(res.name, "testAccount");
    }

    #[tokio::test]
    async fn test_http_get_contact_400() {
        let http_server = HTTP_SERVER.get_or_init(setup).await;

        let res: HashMap<String, String> = reqwest::Client::new()
            .get(&format!("{}/contact", &http_server.uri()))
            .send()
            .await
            .unwrap()
            .json::<HashMap<String, String>>()
            .await
            .unwrap();

        assert_eq!(
            res.get(&"first name".to_string()),
            Some(&"eric".to_string())
        );
    }

    #[tokio::test]
    async fn test_http_post_account_200() {
        let http_server = HTTP_SERVER.get_or_init(setup).await;

        let res = reqwest::Client::new()
            .post(&format!("{}/account", &http_server.uri()))
            .send()
            .await
            .unwrap();

        println!("test_http_post_account_200 is {:?}", res);
        assert_eq!(res.status().as_u16(), 200);
    }
}
