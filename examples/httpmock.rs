#[tokio::main]
async fn main() {}

#[cfg(test)]
mod test {
    use super::*;
    use tokio::sync::OnceCell;
    use wiremock::matchers::{method, path};
    use wiremock::{Mock, MockServer, ResponseTemplate};

    static HTTP_SERVER: OnceCell<MockServer> = OnceCell::const_new();

    async fn setup() -> MockServer {
        println!("setup the server now");

        let http_server = MockServer::start().await;

        Mock::given(path("/account"))
            .and(method("GET"))
            .respond_with(ResponseTemplate::new(200))
            .mount(&http_server)
            .await;

        Mock::given(path("/contact"))
            .and(method("GET"))
            .respond_with(ResponseTemplate::new(400))
            .mount(&http_server)
            .await;

        http_server
    }

    #[tokio::test]
    async fn test_http_get_account_200() {
        let http_server = HTTP_SERVER.get_or_init(setup).await;

        let res = reqwest::Client::new()
            .get(&format!("{}/account", &http_server.uri()))
            .send()
            .await
            .unwrap();

        println!("test_http_get_account_200 is {:?}", res);

        assert_eq!(res.status().as_u16(), 200);
    }

    #[tokio::test]
    async fn test_http_get_contact_400() {
        let http_server = HTTP_SERVER.get_or_init(setup).await;

        let res = reqwest::Client::new()
            .get(&format!("{}/contact", &http_server.uri()))
            .send()
            .await
            .unwrap();

        println!("test_http_get_contact_400 is {:?}", res);
        assert_eq!(res.status().as_u16(), 400);
    }
}
