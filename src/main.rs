extern crate mammut;
extern crate rustyline;
extern crate toml;

use std::fs::File;
use std::io;
use std::io::prelude::*;

use rustyline::error::ReadlineError;
use rustyline::Editor;

use mammut::apps::{AppBuilder, Scopes};
use mammut::{Data, Mastodon, Registration, StatusBuilder};

const MASTODON_DATA: &str = "mastodon-data.toml";

fn main() {
    let mastodon = match File::open(MASTODON_DATA) {
        Ok(f) => load_config(f),
        Err(_) => register(),
    };

    let mut rl = Editor::<()>::new();

    loop {
        let new_status = rl.readline(">> ");
        match new_status {
            Ok(line) => {
                let status = StatusBuilder::new(line);
                //println!("StatusBuilder = {:#?}", status);
                mastodon.new_status(status).expect("Couldn't post status");
                print!("Status Posted!");
            }
            Err(ReadlineError::Interrupted) => {
                println!("Closing Parviderm...");
                break;
            }
            Err(_) => {
                println!("No input");
                break;
            }
        }
    }
}

fn register() -> Mastodon {
    let app = AppBuilder {
        client_name: "Paviderm",
        redirect_uris: "urn:ietf:wg:oauth:2.0:oob",
        scopes: Scopes::All,
        website: Some("https://drisc.io/wiki/paviderm"),
    };

    print!("Enter instance name: ");
    io::stdout().flush().unwrap();
    let mut instance_name = String::new();
    io::stdin()
        .read_line(&mut instance_name)
        .expect("Invalid URL");

    let mut registration = Registration::new(instance_name);
    registration.register(app).unwrap();;
    let url = registration.authorise().unwrap();

    print!("Click this link to authorize on Mastodon: {}", url);
    io::stdout().flush().unwrap();
    print!("Paste the returned authorization code: ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    let code = input.trim();
    let mastodon = registration.create_access_token(code.to_string()).unwrap();

    // Save app data for using on the next run.
    let toml = toml::to_string(&*mastodon).unwrap();
    let mut file = File::create(MASTODON_DATA).unwrap();
    file.write_all(toml.as_bytes()).unwrap();

    mastodon
}

fn load_config(mut file: File) -> Mastodon {
    let mut config = String::new();
    file.read_to_string(&mut config).unwrap();
    let data: Data = toml::from_str(&config).unwrap();
    Mastodon::from_data(data)
}
