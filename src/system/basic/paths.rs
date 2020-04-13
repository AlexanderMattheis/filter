use std::path::Path;

use crate::system::basic::strings;

pub fn get_file_name(path: &String) -> String {
    let file_name_option = match Path::new(path).file_stem() {
        Some(file_name) => file_name.to_str(),
        _ => None
    };

    return match file_name_option {
        Some(file_name) => file_name.to_string(),
        _ => String::from("")
    };
}

pub fn get_file_extension(path: &String) -> String {
    let file_name_option = match Path::new(path).extension() {
        Some(file_name) => file_name.to_str(),
        _ => None
    };

    return match file_name_option {
        Some(file_name) => file_name.to_string(),
        _ => String::from("")
    };
}

pub fn get_file_name_with_extension(path: &String) -> String {
    return strings::concat(&strings::concat(&get_file_name(path), "."), &get_file_extension(path));
}

pub fn is_separator(last_char: char) -> bool {
    if cfg!(windows) && last_char == '\\' || cfg!(unix) && last_char == '/' {
        return true;
    }

    return false;
}