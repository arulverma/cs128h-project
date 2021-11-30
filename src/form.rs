pub fn splice_form_boundary<'a>(string: &'a mut String) -> &'a str {
    let first_boundary = string.find("Content-Type");

    if first_boundary.is_none() {
        return string;
    }

    string.drain(..first_boundary.unwrap());
    let string = string.trim_end_matches('\n');
    let string = string.trim_end_matches(|c| c != '\n');
    let string = string.trim_start_matches(|c| c != '\n');
    string
}
