use reqwest::Client;
use serde::Serialize;
use std::collections::HashMap;

#[derive(Serialize)]
pub enum EmailBody<'request> {
    HtmlBody(&'request str),
    TextBody(&'request str),
}

#[derive(Serialize)]
#[serde(rename_all = "PascalCase")]
pub struct EmailJson<'client, 'request> {
    from: &'client str,
    to: &'request str,
    cc: Option<String>,
    bcc: Option<String>,
    subject: Option<String>,
    tag: Option<String>,
    text_body: &'request str,
    reply_to: Option<String>,
    headers: Option<Vec<String>>,
    track_opens: Option<bool>,
    track_links: Option<String>,
    metadata: Option<HashMap<String, String>>,
    attachments: Option<Vec<String>>,
    message_stream: Option<String>,
}

impl<'client, 'request> EmailJson<'client, 'request> {
    pub fn new(to: &'request str, from: &'client str, text_body: &'request str) -> Self {
        Self {
            from,
            to,
            cc: None,
            bcc: None,
            subject: None,
            tag: None,
            text_body,
            reply_to: None,
            headers: None,
            track_opens: None,
            track_links: None,
            metadata: None,
            attachments: None,
            message_stream: None,
        }
    }
}

pub struct PostmarkClient {
    http_client: Client,
    base_url: String,
    server_auth_token: String,
    from: String,
}

impl PostmarkClient {
    pub fn new(
        base_url: String,
        timeout: std::time::Duration,
        server_auth_token: String,
        from: String,
    ) -> Self {
        let http_client = Client::builder().timeout(timeout).build().unwrap();
        Self {
            http_client,
            base_url,
            server_auth_token,
            from,
        }
    }

    #[tracing::instrument(name = "clients::postmark::send_email", skip(self))]
    pub async fn send_email(&self, to: &str, message: &str) -> Result<(), reqwest::Error> {
        let url = format!("{}email", self.base_url);
        let json = EmailJson::new(to, &self.from, message);

        let response = self
            .http_client
            .post(&url)
            .header("Accept", "application/json")
            .header("Content-Type", "application/json")
            .header("X-Postmark-Server-Token", &self.server_auth_token)
            .body(serde_json::to_string(&json).unwrap())
            .send()
            .await?
            .error_for_status()?;
        dbg!(response);
        Ok(())
    }
}

/*
    #[tracing::instrument(name = "clients::postmark::send_templated_email", skip(self))]
    pub async fn send_templated_email(&self, to: String, template: String) -> Result<(), reqwest::Error> {
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
*/
