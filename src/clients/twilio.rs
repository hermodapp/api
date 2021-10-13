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

    use super::TwilioClient;

    #[tokio::test]
    async fn sms_works() {
        let client = TwilioClient::new(
            "https://api.twilio.com/2010-04-01/".to_string(),
            Duration::from_secs(5),
            "ACb10051bdd7ea0a98dfdb32ab7077a461".to_string(),
            "54ec400e965e1b00c9e5b2573ab840fe".to_string(),
            "+15078892204".to_string(),
        );
        // client
        //     .send_sms(
        //         "‭‭+19857104842‬".to_string(),
        //         "Stop going down on insta.".to_string(),
        //     )
        //     .await
        //     .unwrap();
    }

    #[tokio::test]
    async fn call_works() {
        let client = TwilioClient::new(
            "https://api.twilio.com/2010-04-01/".to_string(),
            Duration::from_secs(5),
            "ACb10051bdd7ea0a98dfdb32ab7077a461".to_string(),
            "54ec400e965e1b00c9e5b2573ab840fe".to_string(),
            "+15078892204".to_string(),
        );
        // client
        //     .send_call("‭+16788961797‬".to_string(), "Hello, world!".to_string())
        //     .await
        //     .unwrap();
    }
}
