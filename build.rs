fn main() {
    if std::env::var("CARGO_CFG_WINDOWS").is_ok() {
        let mut res = winresource::WindowsResource::new();
        // Since we don't have an .ico file, we won't set one yet.
        // But we can set other metadata that will be embedded.
        res.set_icon("app_icon.ico"); 
        res.compile().expect("Failed to compile Windows resources");
    }
}
