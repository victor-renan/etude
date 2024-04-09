mod app;
mod ui;

use gtk::{gio, glib};
use app::App;

fn main() -> glib::ExitCode {

    gio::resources_register_include!("resources.gresource")
        .expect("Fail to load resources");

    App::run()
}