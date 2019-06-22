extern crate mammut;
extern crate toml;
extern crate webbrowser;

use std::io::prelude::*;
use std::{error::Error, fs, io};

use self::mammut::{
    apps::{AppBuilder, Scopes},
    Mastodon, Registration,
};

#[allow(dead_code)]
fn main() -> Result<(), Box<Error>> {
    register()?;
    Ok(())
}

#[allow(dead_code)]
pub fn get_mastodon_data() -> Result<Mastodon, Box<Error>> {
    if let Ok(config) = fs::read_to_string("mastodon-data.toml") {
        Ok(Mastodon::from_data(toml::from_str(&config)?))
    } else {
        register()
    }
}

pub fn register() -> Result<Mastodon, Box<Error>> {
    let app = AppBuilder {
        client_name: "Procul",
        redirect_uris: "urn:ietf:wg:oauth:2.0:oob",
        scopes: Scopes::All,
        website: Some("https://drisc.io/wiki/procul"),
    };

    let mut instance_url = String::new();
    print!("Please enter your mastodon instance url: https://");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut instance_url).unwrap();
    let mut registration = Registration::new(format!("https://{}/", instance_url.trim()));
    registration.register(app)?;
    let url = registration.authorise()?;

    webbrowser::open(&url).expect("Unable to open browser");
    let input = read_line("Paste the returned authorization code: ")?;

    let code = input.trim();
    let mastodon = registration.create_access_token(code.to_string())?;

    // Save app data for using on the next run.
    let toml = toml::to_string(&*mastodon)?;
    fs::write("mastodon-data.toml", toml.as_bytes())?;

    Ok(mastodon)
}

pub fn read_line(message: &str) -> Result<String, Box<Error>> {
    println!("{}", message);

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    Ok(input)
}
