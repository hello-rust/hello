use failure::Error;
use fantoccini::{Client as FantocciniClient, Locator};
use futures::future::Future;
use tokio_core;

pub struct Credentials {
    pub username: String,
    pub password: String,
}

pub struct Client {
    credentials: Credentials,
}

impl Client {
    pub fn new(credentials: Credentials) -> Client {
        Client { credentials }
    }

    pub fn submit(&self, title: String, url: String) -> Result<(), Error> {
        let mut core = tokio_core::reactor::Core::new()?;
        let c = FantocciniClient::new("http://localhost:4444", &core.handle());
        let c = core.run(c)?;

        {
            // we want to have a reference to c so we can use it in the and_thens below
            let c = &c;

            // We're dealing with unnamed forms, so we have to select the elements ourselves.
            let f = c.goto("https://news.ycombinator.com/submit")
                .and_then(move |_| c.form(Locator::Css("form:first-of-type")))
                .and_then(|f| f.set_by_name("acct", &self.credentials.username))
                .and_then(|f| f.set_by_name("pw", &self.credentials.password))
                .and_then(|f| f.submit())
                .and_then(move |_| c.current_url())
                .and_then(|url| {
                    assert_eq!(url.as_ref(), "https://news.ycombinator.com/submit");
                    Ok(())
                })
                .and_then(move |_| c.form(Locator::Css("form:first-of-type")))
                .and_then(|f| f.set_by_name("title", &title))
                .and_then(|f| f.set_by_name("url", &url))
                .and_then(|f| f.submit());

            core.run(f)?;
        }

        // drop the client to delete the browser session
        if let Some(fin) = c.close() {
            // and wait for cleanup to finish
            core.run(fin)?;
        }

        Ok(())
    }
}
