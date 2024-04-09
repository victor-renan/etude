use gtk::prelude::*;
use crate::ui::{Window};
use gtk::{Application, glib};

const APP_ID: &str = "org.etude";

pub struct App {}

impl App {
    pub fn build(app: &Application) {
        Window::new(app).present()
    }

    pub fn run() -> glib::ExitCode {
        let app = Application::builder()
            .application_id(APP_ID)
            .build();

        app.connect_activate(Self::build);

        app.run()
    }
}