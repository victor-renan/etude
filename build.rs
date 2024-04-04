fn main() {
    glib_build_tools::compile_resources(
        &["data/resources"],
        "data/resources.gresource.xml",
        "resources.gresource",
    );
}