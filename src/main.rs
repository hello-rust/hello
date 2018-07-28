#[macro_use]
extern crate structopt;
#[macro_use]
extern crate dotenv_codegen;
extern crate dotenv;
extern crate egg_mode;
extern crate failure;
extern crate fantoccini;
extern crate futures;
extern crate rawr;
extern crate rustc_serialize;
extern crate tokio_core;

mod platforms;

use std::env;

use dotenv::dotenv;
use failure::Error;
use structopt::StructOpt;

use platforms::*;

#[derive(StructOpt)]
#[structopt(name = "hello", about = "Share on social platforms")]
enum App {
    #[structopt(name = "hn")]
    Hackernews { title: String, url: String },
    #[structopt(name = "reddit")]
    Reddit {
        subreddit: String,
        title: String,
        url: String,
    },
    #[structopt(name = "twitter")]
    Twitter { text: String },
}

fn main() -> Result<(), Error> {
    dotenv().ok();

    // TODO: This should be done with inversion of control
    let app = App::from_args();
    match app {
        App::Hackernews { title, url } => {
            let credentials = hackernews::Credentials {
                username: env::var("HN_USERNAME")?,
                password: env::var("HN_PASSWORD")?,
            };

            let client = hackernews::Client::new(credentials);
            client.submit(title, url)
        }
        App::Reddit {
            subreddit,
            title,
            url,
        } => {
            let credentials = reddit::Credentials {
                client_id: env::var("REDDIT_CLIENT_ID")?,
                client_secret: env::var("REDDIT_CLIENT_SECRET")?,
                username: env::var("REDDIT_USERNAME")?,
                password: env::var("REDDIT_PASSWORD")?,
            };

            let client = reddit::Client::new(credentials);
            client.submit(subreddit, title, url)
        }
        App::Twitter { text } => {
            let consumer_key = dotenv!("TWITTER_CONSUMER_KEY").to_string();
            let consumer_secret = dotenv!("TWITTER_CONSUMER_SECRET").to_string();
            let credentials = match (
                env::var("TWITTER_ACCESS_KEY"),
                env::var("TWITTER_ACCESS_SECRET"),
            ) {
                // Already registered
                (Ok(access_token_key), Ok(access_token_secret)) => twitter::Credentials::new(
                    consumer_key,
                    consumer_secret,
                    access_token_key,
                    access_token_secret,
                ),
                // Not registerd yet. Requires OAuth dance
                _ => twitter::Credentials::load(consumer_key, consumer_secret)?,
            };

            let client = twitter::Client::new(credentials);
            client.submit(text)
        }
    }
}
