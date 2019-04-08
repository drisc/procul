mod register;

extern crate mammut;
extern crate rustyline;
extern crate toml;

use std::{
    error};
use mammut::status_builder::{
    StatusBuilder,
    Visibility::*};

use rustyline::{error::ReadlineError,
                Editor};


fn main() {
    terminal();
}

fn terminal() {
    let mut rl = Editor::<()>::new();
    loop {
        let input = rl.readline("[Procul]>> ");
        match input {
            Ok(line) => match line.as_ref() {
                "quit" => {
                    println!("Quitting Procul");
                    std::process::exit(0);
                },
                "nsp" => {
                    println!("Entering Post Mode");
                    new_status();
                },
                _ => println!("No input")
            },
            Err(_err) => {
                println!("No input");
            }
        }
    }
}

fn new_status() -> Result<(), Box<error::Error>>{

    let mut rl = Editor::<()>::new();
    let mastodon = register::get_mastodon_data()?;

    loop {
        let input = rl.readline("[New Status]>> ");
        match input {
            Ok(line) => match line.as_ref() {
                ":exit" => {
                    println!("Returning to terminal...");
                    terminal();
                },
                _ => {
                    mastodon.new_status(StatusBuilder {
                        status: String::from(line),
                        in_reply_to_id: None,
                        media_ids: None,
                        sensitive: Some(false),
                        spoiler_text: None,//Some("CW Text".to_string()),
                        visibility: Some(Public),
                    }).expect("Couldn't post status");
                    println!("Status Posted!");
                    terminal();
                }
            },
            Err(ReadlineError::Interrupted) => {
                println!("CTRL+C");
                std::process::exit(1);
            },
            Err(_err) => {
                println!("No input");
            }
        }
    }
}
