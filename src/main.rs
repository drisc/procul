mod register;

extern crate cursive;
extern crate kuchiki;
extern crate mammut;
extern crate rustyline;
extern crate toml;

use cursive::align::HAlign;
use cursive::theme::{BaseColor, BorderStyle, Color, ColorStyle, PaletteColor, Theme};
use cursive::traits::*;
use cursive::views::{
    Dialog, DummyView, EditView, LinearLayout, ListView, SelectView, TextArea, TextView,
};
use cursive::Cursive;

use kuchiki::traits::TendrilSink;

use mammut::entities::status::Status;
use mammut::status_builder::{StatusBuilder, Visibility::*};
use mammut::Mastodon;
use rustyline::{error::ReadlineError, Editor};

fn main() {
    interface();
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
                }
                "n" => {
                    println!("Entering Post Mode");
                    new_status();
                }
                "tl" => {
                    println!("Fetching timeline posts...");
                    let _mastodon = register::get_mastodon_data().unwrap();
                    view_home_tl(_mastodon.clone());
                }
                _ => println!("Invalid input"),
            },
            Err(_err) => {
                println!("Unknown Error");
            }
        }
    }
}

#[allow(dead_code)]
pub fn interface() {
    let mut siv = Cursive::default();
    siv.add_global_callback('q', |s| s.quit());

    let _mastodon = register::get_mastodon_data().unwrap();

    //siv.add_fullscreen_layer(Canvas::new(()).with_draw(draw).full_width().full_height());
    let theme = theme_terminal(&siv);
    siv.set_theme(theme);

    let _timeline = view_home_tl(register::get_mastodon_data().unwrap());

    siv.add_fullscreen_layer(
        Dialog::around(
            LinearLayout::vertical()
                .child(
                    LinearLayout::horizontal()
                        .child(
                            TextView::new(get_account_username(_mastodon.clone()))
                                .h_align(HAlign::Left)
                                .fixed_width(30),
                        )
                        .child(
                            TextView::new(get_instance_name(_mastodon.clone()))
                                .h_align(HAlign::Center)
                                .full_width(),
                        )
                        .child(TextView::new("").h_align(HAlign::Right).fixed_width(30)),
                )
                .child(
                    Dialog::new()
                        .title("New Post")
                        .padding((1, 1, 1, 0))
                        .content(
                            TextArea::new()
                                .with_id("post")
                                .fixed_width(20)
                                .fixed_height(5),
                        )
                        .button("Post", move |s| {
                            s.call_on_id("post", |view: &mut EditView| {
                                _mastodon.new_status(StatusBuilder {
                                    status: view.get_content().to_string(),
                                    in_reply_to_id: None,
                                    media_ids: None,
                                    sensitive: Some(false),
                                    spoiler_text: None,
                                    visibility: Some(Public),
                                });
                            });
                        }),
                )
                .child(DummyView.fixed_height(1)),
        )
        .h_align(HAlign::Center),
    );

    siv.run();
}

fn theme_terminal(siv: &Cursive) -> Theme {
    let mut theme = siv.current_theme().clone();
    theme.palette[PaletteColor::Background] = Color::TerminalDefault;

    theme
}

// Get Instance Info
fn get_instance_name(client: Mastodon) -> String {
    let instance_name = client.instance().unwrap().title;
    return instance_name;
}

fn get_account_username(client: Mastodon) -> String {
    let account_name = format!(
        "{} <@{}>",
        client.verify_credentials().unwrap().display_name,
        client.verify_credentials().unwrap().acct
    );
    return account_name;
}

fn display_timeline(timeline: &Vec<Status>) {
    for status in timeline.iter() {
        let parser = kuchiki::parse_html();
        let node_ref = parser.one(&status.content[..]);
        let text = node_ref.text_contents();
        let account_name = &status.account.display_name;
        println!("@{}: {}", account_name, &text);
    }
}

fn view_home_tl(client: Mastodon) {
    let timeline = match client.get_home_timeline() {
        Ok(timeline) => timeline,
        Err(e) => {
            println!("Could not view timeline: ");
            println!("{:?}", e);
            return;
        }
    };
    display_timeline(&timeline.initial_items);
}

#[allow(dead_code)]
fn unlisted_post() {
    let mut rl = Editor::<()>::new();
    let _mastodon = register::get_mastodon_data().unwrap();

    loop {
        let input = rl.readline("[Unlisted]>> ");
        match input {
            Ok(line) => {
                _mastodon
                    .new_status(StatusBuilder {
                        status: String::from(line),
                        in_reply_to_id: None,
                        media_ids: None,
                        sensitive: Some(false),
                        spoiler_text: None,
                        visibility: Some(Unlisted),
                    })
                    .expect("Couldn't post status");
                println!("Status Posted!");
                terminal();
            }
            Err(ReadlineError::Interrupted) => {
                println!("CTRL+C");
                std::process::exit(1);
            }
            Err(_err) => {
                println!("No input");
            }
        }
    }
}

fn new_status() {
    let mut rl = Editor::<()>::new();
    let _mastodon = register::get_mastodon_data().unwrap();

    loop {
        let input = rl.readline("[New Status]>> ");
        match input {
            Ok(line) => match line.as_ref() {
                ":exit" => {
                    println!("Returning to terminal...");
                    terminal();
                }
                ":public" => {
                    //public_post();
                }
                ":unlisted" => {
                    unlisted_post();
                }
                _ => {
                    println!("Options: ':public', ':unlisted'");
                }
            },
            Err(ReadlineError::Interrupted) => {
                println!("CTRL+C");
                std::process::exit(1);
            }
            Err(_err) => {
                println!("No input");
            }
        }
    }
}
