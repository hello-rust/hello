use failure::Error;
use openssl::pkey::Private;
use openssl::rsa::Rsa;
use rand::{thread_rng, RngCore};
use reqwest;
use std::fmt::Write;

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

    fn gen_key_url(base_url: String) -> Result<String, String> {
        let mut rng = thread_rng();
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
        let app_name = "Discourse TUI";
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

    /// If we don't have an access token already, try to register.
    pub fn load(base_url: String) -> Result<Credentials, Error> {
        let url = Credentials::gen_key_url(base_url)
            .map_err(|e| format_err!("Cannot retrieve Discourse API URL: {:?}", e))?;

        println!(
            "Go to this URL to retrieve your Discourse credentials: {}",
            url
        );
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
