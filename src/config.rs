extern crate config;

#[derive(Debug, Deserialize)]
pub struct Configuration {
    pub width: i32,
    pub height: i32,
    pub font: String,
}

impl Configuration {
    pub fn new() -> Self {
        // Read in config file.
        let mut settings = config::Config::default();
        let _x = settings.merge(config::File::with_name("config/console.toml"));
        settings.try_into().unwrap()
    }
}
