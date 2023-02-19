use clap::builder::StyledStr;
use cursive::theme::Style;
use cursive::utils::markup::StyledString;
use cursive::views::ListView;
use cursive::views::ScrollView;
use cursive::views::TextView;

use crate::render_notifications;
use crate::request;

const PAGE_SIZE: usize = 10;

pub struct Cli {
    pub siv: cursive::CursiveRunnable,
}

impl Cli {
    pub fn new() -> Self {
        Self {
            siv: cursive::default(),
        }
    }
    pub fn init(&mut self) {
        let res_json = request::get_notifications();
        let json = match res_json {
            Ok(json) => json,
            Err(e) => {
                println!("Error in request: {}", e);
                std::process::exit(1);
            }
        };
        // check if there are any notifications
        if json.is_empty() {
            println!("No notifications");
            std::process::exit(0);
        }
        // handle error in de serialization
        let notifications = match render_notifications::serialize(&json) {
            ok @ Ok(_) => ok.unwrap(),
            err @ Err(_) => {
                println!(
                    "Error in serialization: {}\nThe JSON was: {}",
                    err.unwrap_err(),
                    json
                );
                std::process::exit(1);
            }
        };
        self.siv.add_global_callback('q', |s| s.quit());
        let mut text: String = String::new();
        // set the heading
        text.push_str("NOTIFICATIONS\n\n");
        for notification in notifications {
            text.push_str(&(notification.text() + "\n"));
        }
        text.push_str("\nPress q to quit");
        self.siv.add_layer(ScrollView::new(TextView::new(text)));
    }
    pub fn run(&mut self) {
        self.siv.run();
    }
}

impl Default for Cli {
    fn default() -> Self {
        Self::new()
    }
}
