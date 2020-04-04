pub fn concat(a: &String, b: &str) -> String {
    let mut concatenated = String::from(a);
    concatenated.push_str(b);
    return concatenated;
}

pub fn concat_with_static(a: &String, b: &'static str) -> String {
    let mut concatenated = String::from(a);
    concatenated.push_str(b);
    return concatenated;
}