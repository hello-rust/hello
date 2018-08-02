use failure::{err_msg, Error};
use openssl::pkey::Private;
use openssl::rsa::Rsa;
use rand::{self, RngCore};
use reqwest;
use std::env::current_exe;
use std::fmt::Write;
use system_uri;

pub struct Credentials {
    pub api_key: String,
    pub api_username: String,
}

impl Credentials {
    pub fn new(api_key: String, api_username: String) -> Credentials {
        Credentials {
            api_key,
            api_username,
        }
    }

    fn register_schema(schema: String) -> Result<(), Error> {
        let rng = rand::thread_rng();
        let exec = String::from(current_exe()?.to_str().ok_or(err_msg(
            "Cannot retrieve filesystem path to the `hello` executable",
        ))?);
        let app = system_uri::App::new(
            "de.matthias-endler.hello".to_string(),
            "Matthias Endler".to_string(),
            "Hello Rust".to_string(),
            exec,
            None,
        );

        println!("Installing ourselves under {}", schema);

        system_uri::install(&app, &[schema.clone()]).map_err(|e| {
            format_err!(
                "Cannot install system uri handler for discourse schema: {}",
                e,
            )
        })?;

        println!("Trying to open {}test", schema);
        system_uri::open(format!("{}test", schema)).map_err(|e| {
            format_err!(
                "Cannot open test URI using discourse system uri handler: {}",
                e,
            )
        })?;
        println!("Open succeeded 😄, everything is fine 🎉!");
        Ok(())
    }

    fn get_auth_url(base_url: String) -> Result<String, String> {
        let mut rng = rand::thread_rng();
        let scopes = "read,write";
        let mut client_id_bytes = [0u8; 16];
        rng.fill_bytes(&mut client_id_bytes);
        let mut client_id = String::new();
        for &byte in client_id_bytes.iter() {
            write!(&mut client_id, "{:02x}", byte).expect("Unable to write");
        }
        let mut nonce_bytes = [0u8; 8];
        rng.fill_bytes(&mut nonce_bytes);
        let mut nonce = String::new();
        for &byte in nonce_bytes.iter() {
            write!(nonce, "{:02x}", byte).expect("Unable to write");
        }

        let auth_redirect = "discourse://auth_redirect";
        let app_name = "Hello Rust";
        let public_key: String;
        let keypair: Rsa<Private>;
        match Rsa::generate(2048) {
            Ok(kp) => {
                keypair = kp;
                match keypair.public_key_to_pem() {
                    Ok(pem) => public_key = String::from_utf8(pem).unwrap(),
                    Err(estack) => return Err(format!("{}", estack)),
                }
            }
            Err(estack) => return Err(format!("{}", estack)),
        }
        let client = reqwest::Client::new();
        let mut temp_url = base_url.to_owned();
        temp_url.push_str("/user-api-key/new");
        let req: reqwest::Request;
        match client
            .post(&temp_url)
            .query(&[
                ("scopes", scopes),
                ("client_id", &client_id),
                ("nonce", &nonce),
                ("auth_redirect", auth_redirect),
                ("application_name", app_name),
                ("public_key", &public_key),
            ])
            .build()
        {
            Ok(r) => req = r,
            Err(_) => return Err("Error building query url".to_string()),
        }
        let req_url = req.url().clone().into_string();
        // let partial_api = PartialApi {
        //     pem: String::from_utf8(keypair.private_key_to_pem().unwrap()).unwrap(),
        //     client_id: client_id,
        //     base_url: base_url,
        //     api_authorize_url: req_url,
        // };
        // Ok(partial_api)
        Ok(req_url)
    }

    fn register(base_url: String) -> Result<(), Error> {
        Credentials::register_schema("discourse://".to_string())?;
        let url = Credentials::get_auth_url(base_url)
            .map_err(|e| format_err!("Cannot retrieve Discourse API URL: {:?}", e))?;
        system_uri::open(url.clone())
            .map_err(|e| format_err!("Cannot open discourse API URI: {}", e))?;

        println!(
            "Click on this URL to retrieve your Discourse credentials: {}",
            url
        );
        Ok(())
    }

    /// If we don't have an access token already, try to register.
    pub fn load(base_url: String) -> Result<Credentials, Error> {
        Credentials::register(base_url)?;

        let api_key = String::from("dummy");
        let api_username = String::from("dummy");
        Ok(Credentials {
            api_key,
            api_username,
        })
    }
}

pub struct Client {
    credentials: Credentials,
}

impl Client {
    pub fn new(credentials: Credentials) -> Client {
        Client { credentials }
    }

    pub fn submit(&self, topic: String, content: String) -> Result<(), Error> {
        println!("Submit");
        Ok(())
    }
}
