use cursive::views::{Dialog, TextView};

use crate::render_notifications;
use crate::request;

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
        let notifications = match render_notifications::serialize(json) {
            ok @ Ok(_) => ok.unwrap(),
            err @ Err(_) => {
                println!("Error in serialization: {}", err.unwrap_err());
                std::process::exit(1);
            }
        };
        let rendered_notifications = render_notifications::render(notifications);
        let mut text = String::new();
        rendered_notifications
            .into_iter()
            .for_each(|n| text.push_str(&format!("- {}\n", n)));
        self.siv.add_global_callback('q', |s| s.quit());
        // TODO: add the notifications here in a very nice format
        let text_view = TextView::new(text);
        self.siv.add_layer(
            Dialog::new()
                .content(text_view)
                .title("GitHub Notifications")
                .button("QUIT", |s| s.quit()),
        );
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

// TODO: implement this for next so +1 pages and prev -1 pages
// pub fn next(siv: &mut cursive::Cursive, msg: &str) {
//     siv.pop_layer();
//     siv.add_layer(
//         Dialog::text(msg)
//             .title(format!("GitHub Messages Page {}", msg))
//             .button("Next", |s| next(s, "Hello")),
//     );
// }
