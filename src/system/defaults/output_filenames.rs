use crate::system::basic::{strings, paths};

pub struct ChannelOutputNames;

pub const SEPARATOR: &'static str = "_";

impl ChannelOutputNames {
    pub const RED: &'static str = "red";
    pub const GREEN: &'static str = "green";
    pub const BLUE: &'static str = "blue";
    pub const ALPHA: &'static str = "alpha";
    pub const LUMINANCE: &'static str = "luminance";
}

pub fn create_prefix(input_file_path: &String, output_path: &String) -> String {
    let filename = strings::concat(&paths::get_file_name(input_file_path), SEPARATOR);

    let is_directory = match output_path.chars().last() {
        Some(last_char) => paths::is_separator(last_char),
        _ => false
    };

    if is_directory {
        return strings::concat(&output_path, &filename);
    } else if cfg!(windows) {
        return strings::concat(&strings::concat(&output_path, "\\"), &filename);
    }

    return strings::concat(&strings::concat(&output_path, "/"), &filename);  // Unix
}
