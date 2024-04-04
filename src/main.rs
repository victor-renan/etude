use gtk::prelude::*;

const APP_ID: &str = "org.etude.main";

fn main() -> gtk::glib::ExitCode {

    gtk::gio::resources_register_include!("resources.gresource")
        .expect("Fail to load resources");

    let app = gtk::Application::builder()
        .application_id(APP_ID).build();

    app.connect_activate(build_ui);
    
    app.run()
}

fn build_ui(app: &gtk::Application) {
    
}