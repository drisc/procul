mod register;

extern crate cursive;
extern crate kuchiki;
extern crate mammut;
extern crate toml;

use cursive::align::HAlign;
use cursive::theme::{BaseColor, BorderStyle, Color, ColorStyle, PaletteColor, Theme};
use cursive::traits::*;
use cursive::utils::markup::StyledString;
use cursive::views::{
    BoxView, Dialog, DummyView, EditView, LinearLayout, ListView, SelectView, TextArea, TextView,
};
use cursive::Cursive;

use kuchiki::traits::TendrilSink;

use mammut::entities::status::Status;
use mammut::status_builder::{StatusBuilder, Visibility::*};
use mammut::Mastodon;

fn main() {
    interface();
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
                    LinearLayout::horizontal().child(
                        Dialog::new()
                            .title("New Post")
                            .padding((1, 1, 1, 0))
                            .content(
                                TextArea::new()
                                    .with_id("post")
                                    .fixed_width(20)
                                    .fixed_height(5),
                            )
                            .button("Post", |s| {
                                s.call_on_id("post", |view: &mut TextArea| {
                                    post_status(&view.get_content().to_string());
                                    view.set_content("");
                                });
                            }),
                    ),
                )
                .child(DummyView.fixed_height(1))
                .child(BoxView::with_full_width(SelectView::new().with(|list| {
                    //let timeline = _mastodon.get_home_timeline();
                    let _mastodon = register::get_mastodon_data().unwrap();
                    let resp = _mastodon.get_home_timeline().unwrap();
                    for status in resp.initial_items {
                        let parser = kuchiki::parse_html();
                        let node_ref = parser.one(&status.content[..]);
                        let text = node_ref.text_contents();
                        let account_name = status.account.acct;
                        let status_string = format!("@{}: {}", account_name, &text);
                        let status_id = status.id;
                        list.add_item(status_string, status_id);
                        //println!("@{}: {}", account_name, &text);
                    }
                }))),
        )
        .h_align(HAlign::Center),
    );

    siv.run();
}

fn post_status(text: &str) {
    let _mastodon = register::get_mastodon_data().unwrap();
    _mastodon.new_status(StatusBuilder {
        status: text.to_string(),
        in_reply_to_id: None,
        media_ids: None,
        sensitive: Some(false),
        spoiler_text: None,
        visibility: Some(Public),
    });
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
