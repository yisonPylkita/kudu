extern crate cursive;

use cursive::traits::*;
use cursive::view::{Offset, Position};
use cursive::views::{Dialog, OnEventView, TextView, LinearLayout, StackView};
use cursive::Cursive;

fn main() {
    let mut app = Cursive::default();
    app.set_fps(60);
    app.add_global_callback('q', |s| s.quit());

    let screen_size = app.screen_size();

    let app_l = cursive::views::LinearLayout::vertical()
        .child(TextView::new("F1").fixed_width(screen_size.x))
        .child(TextView::new("FILES_HERE").fixed_width(20));

    app.add_fullscreen_layer(app_l);
    app.run();
}
