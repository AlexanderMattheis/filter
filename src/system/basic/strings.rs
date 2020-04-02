pub fn concat(a: &String, b: &'static str) -> String {
    let mut concatenated = String::from(a);
    concatenated.push_str(b);
    return concatenated;
}