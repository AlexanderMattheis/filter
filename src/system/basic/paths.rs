use std::path::Path;

pub fn get_file_name(path: &String) -> String {
    let filename_option = match Path::new(path).file_stem() {
        Some(filename) => filename.to_str(),
        _ => None
    };

    return match filename_option {
        Some(filename) => filename.to_string(),
        _ => String::from("")
    };
}

pub fn is_separator(last_char: char) -> bool {
    if cfg!(windows) && last_char == '\\' || cfg!(unix) && last_char == '/' {
        return true;
    }

    return false;
}