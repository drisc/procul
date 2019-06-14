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
                "q" | "quit" => {
                    println!("Quitting Procul");
                    std::process::exit(0);
                },
                "n" => {
                    println!("Entering Post Mode");
                    new_status();
                },
                "tl" => {
                    println!("Fetching timeline posts...");
                    get_timeline();
                }
                _ => println!("Invalid input")
            },
            Err(_err) => {
                println!("Unknown Error");
            }
        }
    }
}

fn get_timeline() -> Result<(), Box<error::Error>> {
    let _mastodon = register::get_mastodon_data()?;
    let tl = println!("{:?}", _mastodon.get_home_timeline()?.initial_items);
    Ok(())
}

fn public_post() -> Result<(), Box<error::Error>> {
    let mut rl = Editor::<()>::new();
    let _mastodon = register::get_mastodon_data()?;

    loop {
        let input = rl.readline("[Public]>> ");
        match input {
            Ok(line) => {
                _mastodon.new_status(StatusBuilder {
                    status: String::from(line),
                    in_reply_to_id: None, //Some(101892808492601451),
                    media_ids: None,
                    sensitive: Some(false),
                    spoiler_text: None,//Some("CW Text".to_string()),
                    visibility: Some(Public),
                }).expect("Couldn't post status");
                println!("Status Posted!");
                terminal();
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


fn unlisted_post() -> Result<(), Box<error::Error>> {
    let mut rl = Editor::<()>::new();
    let _mastodon = register::get_mastodon_data()?;

    loop {
        let input = rl.readline("[Unlisted]>> ");
        match input {
            Ok(line) => {
                _mastodon.new_status(StatusBuilder {
                    status: String::from(line),
                    in_reply_to_id: None,
                    media_ids: None,
                    sensitive: Some(false),
                    spoiler_text: None,
                    visibility: Some(Unlisted),
                }).expect("Couldn't post status");
                println!("Status Posted!");
                terminal();
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


fn new_status() -> Result<(), Box<error::Error>> {

    let mut rl = Editor::<()>::new();
    let _mastodon = register::get_mastodon_data()?;

    loop {
        let input = rl.readline("[New Status]>> ");
        match input {
            Ok(line) => match line.as_ref() {
                ":exit" => {
                    println!("Returning to terminal...");
                    terminal();
                },
                ":public" => {
                    public_post();
                },
                ":unlisted" => {
                    unlisted_post();
                },
                _ => {
                    println!("Options: ':public', ':unlisted'");
                    new_status();
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
