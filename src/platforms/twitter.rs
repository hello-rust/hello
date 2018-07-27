use egg_mode;
use egg_mode::tweet::DraftTweet;
use failure::Error;
use std::io;
use tokio_core::reactor::Core;

pub struct Credentials {
    pub token: egg_mode::Token,
}

impl Credentials {
    pub fn new(
        consumer_key: String,
        consumer_secret: String,
        access_token_key: String,
        access_token_secret: String,
    ) -> Credentials {
        let con_token = egg_mode::KeyPair::new(consumer_key, consumer_secret);
        let access_token = egg_mode::KeyPair::new(access_token_key, access_token_secret);
        let token = egg_mode::Token::Access {
            consumer: con_token,
            access: access_token,
        };
        Credentials { token }
    }

    /// If we don't have an access token already (e.g. if the application is not
    /// registered, grab one via OAuth.
    pub fn load(consumer_key: String, consumer_secret: String) -> Result<Credentials, Error> {
        let mut core = Core::new().unwrap();
        let handle = core.handle();

        let con_token = egg_mode::KeyPair::new(consumer_key, consumer_secret);

        let request_token = core.run(egg_mode::request_token(&con_token, "oob", &handle))?;

        println!("Go to the following URL, sign in, and give me the PIN that comes back:");
        println!("{}", egg_mode::authorize_url(&request_token));
        println!("Type in PIN here:");

        let mut pin = String::new();
        io::stdin().read_line(&mut pin)?;

        let (token, _user_id, _screen_name) = core.run(egg_mode::access_token(
            con_token,
            &request_token,
            pin,
            &handle,
        ))?;

        match token {
            egg_mode::Token::Access {
                access: ref access_token,
                ..
            } => {
                println!("Please add the following to your `.env` file:");
                println!("TWITTER_ACCESS_KEY={}", &access_token.key);
                println!("TWITTER_ACCESS_SECRET={}", &access_token.secret);
            }
            _ => unreachable!(),
        }

        Ok(Credentials { token })
    }
}

pub struct Client {
    credentials: Credentials,
}

impl Client {
    pub fn new(credentials: Credentials) -> Client {
        Client { credentials }
    }

    pub fn submit(&self, text: String) -> Result<(), Error> {
        let mut core = Core::new()?;
        let handle = core.handle();

        let draft = DraftTweet::new(text);
        core.run(draft.send(&self.credentials.token, &handle))?;
        Ok(())
    }
}
