use failure::Error as FailureError;
use fantoccini::{Client as FantocciniClient, Locator};
use futures::Future;
use std::process::Command;
use tokio_core;

pub struct Credentials {
    pub username: String,
    pub password: String,
}

pub struct Client {
    credentials: Credentials,
}

// impl From<fantoccini::error::CmdError> for failure::Error {
//     fn from(e: fantoccini::error::CmdError) -> failure::Error {
//         Err(format_err!("Error when running browser: {}", e.cause()))
//     }
// }

// impl From<fantoccini::error::NewSessionError> for failure::Error {
//     fn from(e: fantoccini::error::NewSessionError) -> failure::Error {
//         Err(format_err!("Error when starting browser session: {}", e.cause()))
//     }
// }

impl Client {
    pub fn new(credentials: Credentials) -> Client {
        Client { credentials }
    }

    pub fn submit(&self, title: String, url: String) -> Result<(), FailureError> {
        Ok(())
        // TODO
        // Command::new("geckodriver")
        //     .spawn()
        //     .map_err(|e| format_err!("{:?}: Cannot start geckodriver. Did you install it?", e))?;

        // let mut core = tokio_core::reactor::Core::new()?;

        // let c = FantocciniClient::new("http://localhost:4444").await?;
        // c.goto("https://news.ycombinator.com/submit").await?;
        // c.close().await

        // // We're dealing with unnamed forms, so we have to select the elements ourselves.
        // let form = c.form(Locator::Css("form:first-of-type")).await?;
        // form.set_by_name("acct", &self.credentials.username);

        //     form.set_by_name("pw", &self.credentials.password);
        //     form.submit();
        //     let url = c.current_url();

        // .and_then(|url| {
        //     assert_eq!(url.as_ref(), "https://news.ycombinator.com/submit");
        //     Ok(())
        // })
        // .and_then(move |_| c.form(Locator::Css("form:first-of-type")))
        // .and_then(|f| f.set_by_name("title", &title))
        // .and_then(|f| f.set_by_name("url", &url))
        // .and_then(|f| f.submit());

        //     core.run(f)?;

        // // drop the client to delete the browser session
        // if let Some(fin) = c.close() {
        //     // and wait for cleanup to finish
        //     core.run(fin)?;
        // }
    }
}
