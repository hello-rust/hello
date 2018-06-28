#[macro_use]
extern crate structopt;
#[macro_use]
extern crate dotenv_codegen;
extern crate dotenv;
extern crate egg_mode;
extern crate failure;
extern crate rawr;
extern crate tokio_core;

mod platforms;

use std::env;

use dotenv::dotenv;
use failure::Error;
use structopt::StructOpt;

use platforms::reddit;
use platforms::twitter;

#[derive(StructOpt)]
#[structopt(name = "submit", about = "Share on social platforms")]
enum App {
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
