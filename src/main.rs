extern crate cursive;

use cursive::traits::*;
use cursive::view::{Offset, Position};
use cursive::views::{Dialog, OnEventView, TextView};
use cursive::Cursive;

fn main() {
    let mut app = Cursive::default();
    app.set_fps(60);
    app.add_global_callback('q', |s| s.quit());

    let mut menu_size = app.screen_size();
    let screen_size = app.screen_size();
    menu_size.y = 1;
    let menu = cursive::views::LinearLayout::horizontal()
        .fixed_size(menu_size);
    // .add_child() ??

    app.add_fullscreen_layer(menu);
    app.run();
}
