use crate::response_error::{GenericError, GenericResult};
use once_cell::sync::Lazy;
use regex::Regex;
use unic_ucd_category::GeneralCategory;

static EMAIL_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r#"(?:[a-z0-9!#$%&'*+/=?^_`{|}~-]+(?:\.[a-z0-9!#$%&'*+/=?^_`{|}~-]+)*|"(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21\x23-\x5b\x5d-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])*")@(?:(?:[a-z0-9](?:[a-z0-9-]*[a-z0-9])?\.)+[a-z0-9](?:[a-z0-9-]*[a-z0-9])?|\[(?:(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.){3}(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?|[a-z0-9-]*[a-z0-9]:(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21-\x5a\x53-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])+)\])"#).expect("Failed to parse email regex.")
});

pub fn validate_email(email: &str) -> GenericResult<()> {
    if !EMAIL_REGEX.is_match(email) {
        return Err(GenericError::MalformedRequest("Invalid email".into()));
    }
    Ok(())
}

pub fn validate_and_sanitize_string(string: &str, allow_new_line: bool) -> GenericResult<String> {
    let mut output = String::new();
    output.reserve(string.len());
    for ch in string.chars() {
        if ch == ' ' || (allow_new_line && ch == '\n') {
            output.push(ch);
        } else if allow_new_line && ch == '\r' {
            //do nothing, remove this character
        } else {
            let ctg = GeneralCategory::of(ch);
            if ctg.is_other() || ctg.is_separator() {
                return Err(GenericError::MalformedRequest(String::from("Input strings for user-supplied content must not contain non-printable characters, excepting newlines in some cases..")));
            } else {
                output.push(ch);
            }
        }
    }

    Ok(output)
}
