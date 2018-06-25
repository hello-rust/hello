use failure::Error;
use rawr::options::LinkPost;
use rawr::prelude::*;

pub struct Credentials {
    pub client_id: String,
    pub client_secret: String,
    pub username: String,
    pub password: String,
}

pub struct Client {
    inner: RedditClient,
}

impl Client {
    pub fn new(credentials: Credentials) -> Client {
        let inner = RedditClient::new(
            "Hello Rust",
            PasswordAuthenticator::new(
                &credentials.client_id,
                &credentials.client_secret,
                &credentials.username,
                &credentials.password,
            ),
        );
        Client { inner }
    }

    pub fn submit(&self, subreddit: String, title: String, url: String) -> Result<(), Error> {
        let subreddit = self.inner.subreddit(&subreddit);
        let post = LinkPost::new(&title, &url);
        subreddit.submit_link(post).expect("Could not submit link!");
        Ok(())
    }
}
