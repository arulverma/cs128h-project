pub fn splice_form_boundary<'a>(string: &'a mut String) -> &'a str {
    let first_boundary = string.find("Content-Type");

    if first_boundary.is_none() {
        return string;
    }

    string.drain(..first_boundary.unwrap());
    let string = string.strip_suffix(|c| c != '\n').unwrap();
    let string = string.strip_prefix(|c| c != '\n').unwrap();
    string
}
