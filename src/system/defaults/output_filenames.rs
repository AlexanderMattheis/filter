use crate::system::basic::{paths, strings};

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
    let file_name = strings::concat(&paths::get_file_name(input_file_path), SEPARATOR);
    return create_output_path(&file_name, output_path);
}

fn create_output_path(file_name: &String, output_path: &String) -> String {
    let is_directory = match output_path.chars().last() {
        Some(last_char) => paths::is_separator(last_char),
        _ => false
    };

    if is_directory {
        return strings::concat(&output_path, &file_name);
    } else if cfg!(windows) {
        return strings::concat(&strings::concat(&output_path, "\\"), file_name);
    }

    return strings::concat(&strings::concat(&output_path, "/"), file_name);  // Unix
}

pub fn create_output_filepath(input_file_path: &String, output_path: &String) -> String {
    let file_name_with_extension = paths::get_file_name_with_extension(input_file_path);
    return create_output_path(&file_name_with_extension, output_path);
}
