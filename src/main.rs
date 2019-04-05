mod register;

extern crate mammut;
extern crate rustyline;
extern crate toml;

use std::error;
use mammut::status_builder::StatusBuilder;

use rustyline::{error::ReadlineError,
                Editor};


fn main()  {
    new_status();
}


fn new_status() -> Result<(), Box<error::Error>>{

    let mastodon = register::get_mastodon_data()?;
    let mut rl = Editor::<()>::new();

    loop {

        let new_status = rl.readline(">> ");
        match new_status {
            Ok(line) => {
                let status = StatusBuilder::new(line);
                mastodon.new_status(status).expect("Couldn't post status");
                println!("Status Posted!");
            }
            Err(ReadlineError::Interrupted) => {
                println!("Closing Procul...");
            }
            Err(_) => {
                println!("No input");
            }
        }

    }
}
