use regex::Regex;

pub fn check_username(username: &str) -> bool {
    let re = Regex::new(r"^[a-z0-9_-]+$").unwrap();
    re.is_match(username)
}
