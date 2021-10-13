use reqwest::Client;

pub struct TwilioClient {
    http_client: Client,
    base_url: String,
    account_sid: String,
    auth_token: String,
    from: String,
}

impl TwilioClient {
    pub fn new(
        base_url: String,
        timeout: std::time::Duration,
        account_sid: String,
        auth_token: String,
        from: String,
    ) -> Self {
        let http_client = Client::builder().timeout(timeout).build().unwrap();
        Self {
            http_client,
            base_url,
            account_sid,
            auth_token,
            from,
        }
    }

    pub async fn send_sms(&self, to: String, message: String) -> Result<(), reqwest::Error> {
        let url = format!("{}Accounts/{}/Messages", self.base_url, &self.account_sid);
        let message = urlencoding::encode(&message);
        let body = format!("Body={}&To={}&From={}", message, to, self.from);

        self.http_client
            .post(&url)
            .header("Content-Type", "application/x-www-form-urlencoded")
            .basic_auth(&self.account_sid, Some(&self.auth_token))
            .body(body)
            .send()
            .await?
            .error_for_status()?;
        Ok(())
    }

    pub async fn send_call(&self, to: String, message: String) -> Result<(), reqwest::Error> {
        let url = format!("{}Accounts/{}/Calls.json", self.base_url, &self.account_sid);
        let message = urlencoding::encode(&message);
        let twiml = format!("<Response><Say>{}</Say></Response>", message);
        let body = format!("Twiml={}&To={}&From={}", twiml, to, self.from);

        self.http_client
            .post(&url)
            .header("Content-Type", "application/x-www-form-urlencoded")
            .basic_auth(&self.account_sid, Some(&self.auth_token))
            .body(body)
            .send()
            .await?
            .error_for_status()?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use wiremock::{
        matchers::{header, method, path},
        Mock, MockServer, Request, ResponseTemplate,
    };

    use super::TwilioClient;

    struct SendSmsBodyMatcher;

    impl wiremock::Match for SendSmsBodyMatcher {
        fn matches(&self, request: &Request) -> bool {
            let result = String::from_utf8(request.body.clone());
            if let Ok(body) = result {
                body.contains(&format!("Body={}", urlencoding::encode("Hello, World!")))
                    && body.contains(&format!("To={}", "+12321231234"))
                    && body.contains(&format!("From={}", "+12321231234"))
            } else {
                false
            }
        }
    }

    struct SendCallBodyMatcher;

    impl wiremock::Match for SendCallBodyMatcher {
        fn matches(&self, request: &Request) -> bool {
            let result = String::from_utf8(request.body.clone());
            if let Ok(body) = result {
                let message = urlencoding::encode("Hello, World!");
                let twiml = format!("<Response><Say>{}</Say></Response>", message);
                body.contains(&format!("Twiml={}", twiml))
                    && body.contains(&format!("To={}", "+12321231234"))
                    && body.contains(&format!("From={}", "+12321231234"))
            } else {
                false
            }
        }
    }

    fn twilio_client(base_url: String) -> TwilioClient {
        TwilioClient::new(
            base_url + "/",
            Duration::from_secs(1),
            "account_sid".to_string(),
            "auth_token".to_string(),
            "+12321231234".to_string(),
        )
    }

    #[tokio::test]
    async fn send_sms_sends_the_correct_http_request() {
        let mock_server = MockServer::start().await;
        let message = "Hello, World!";
        let to = "+12321231234";
        let client = twilio_client(mock_server.uri());
        let url = format!("Accounts/{}/Messages", client.account_sid,);

        Mock::given(header("Content-Type", "application/x-www-form-urlencoded"))
            .and(path(url))
            .and(method("POST"))
            .and(SendSmsBodyMatcher)
            .respond_with(ResponseTemplate::new(200))
            .expect(1)
            .mount(&mock_server)
            .await;

        let _ = client
            .send_sms(to.to_string(), message.to_string())
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn send_call_sends_the_correct_http_request() {
        let mock_server = MockServer::start().await;
        let message = "Hello, World!";
        let to = "+12321231234";
        let client = twilio_client(mock_server.uri());
        let url = format!("Accounts/{}/Calls.json", client.account_sid,);

        Mock::given(header("Content-Type", "application/x-www-form-urlencoded"))
            .and(path(url))
            .and(method("POST"))
            .and(SendCallBodyMatcher)
            .respond_with(ResponseTemplate::new(200))
            .expect(1)
            .mount(&mock_server)
            .await;

        let _ = client
            .send_call(to.to_string(), message.to_string())
            .await
            .unwrap();
    }
}
