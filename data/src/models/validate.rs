use regex::Regex;

lazy_static! {
    pub static ref RE_USERNAME: Regex =
        Regex::new(r"^[a-z][a-z0-9_]{3,31}$").unwrap();
}
