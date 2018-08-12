use failure::Error;
use slack_api as slack;

pub struct Credentials {
    pub token: String,
}

pub struct Client {
    credentials: Credentials,
}

impl Client {
    pub fn new(credentials: Credentials) -> Client {
        Client { credentials }
    }

    pub fn submit(&self, channel: String, text: String) -> Result<(), Error> {
        let client = slack::default_client()?;

        slack::chat::post_message(
            &client,
            &self.credentials.token,
            &slack::chat::PostMessageRequest {
                channel: &channel,
                text: &text,
                ..slack::chat::PostMessageRequest::default()
            },
        )?;
        Ok(())
    }
}
