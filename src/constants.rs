use once_cell::sync::Lazy;

pub const ENGLISH: &str = "AaBbCcDdEeFfGgHhIiJjKkLlMmNnOoPpQqRrSsTtUuVvWwXxYyZz";

pub const FONT: Lazy<Vec<u8>> = Lazy::new(|| include_bytes!("roboto-mono-stripped.ttf").to_vec());
