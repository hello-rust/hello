#[macro_use]
extern crate structopt;
#[macro_use]
extern crate dotenv_codegen;
extern crate dotenv;
extern crate failure;
extern crate rawr;

mod platforms;

use platforms::reddit;

use dotenv::dotenv;
use failure::Error;
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(name = "submit", about = "Share on social platforms")]
enum App {
    #[structopt(name = "reddit")]
    Reddit {
        subreddit: String,
        title: String,
        url: String,
    },
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
                client_id: dotenv!("REDDIT_CLIENT_ID").to_string(),
                client_secret: dotenv!("REDDIT_CLIENT_SECRET").to_string(),
                username: dotenv!("REDDIT_USERNAME").to_string(),
                password: dotenv!("REDDIT_PASSWORD").to_string(),
            };

            let client = reddit::Client::new(credentials);
            client.submit(subreddit, title, url)
        }
    }
}
