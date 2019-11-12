// some sharing helpers

use base64;
use deunicode::deunicode_char;
use regex::Regex;
use std::collections::HashMap;

lazy_static! {
    static ref RE_SPC: Regex = Regex::new(r"[^a-zA-Z0-9]").unwrap(); // let fail in test
}

// slug, better to show url: ty as type, for rut|item|collect
pub fn gen_slug(text: &str) -> String {
    let ts =  chrono::Utc::now().timestamp();
    format!("{}-{}", slugify(text), ts)
}

// credit to https://github.com/Stebalien/slug-rs/blob/master/src/lib.rs
pub fn slugify<S: AsRef<str>>(s: S) -> String {
    _slugify(s.as_ref())
}

fn _slugify(s: &str) -> String {
    let mut slug: Vec<u8> = Vec::with_capacity(s.len());
    // Starts with true to avoid leading -
    let mut prev_is_dash = true;
    {
        let mut push_char = |x: u8| {
            match x {
                b'a'..=b'z' | b'0'..=b'9' => {
                    prev_is_dash = false;
                    slug.push(x);
                }
                b'A'..=b'Z' => {
                    prev_is_dash = false;
                    slug.push(x - b'A' + b'a'); // u8
                }
                _ => {
                    if !prev_is_dash {
                        slug.push(b'-');
                        prev_is_dash = true;
                    }
                }
            }
        };

        for c in s.chars() {
            if c.is_ascii() {
                (push_char)(c as u8);
            } else {
                for &cx in deunicode_char(c).unwrap_or("-").as_bytes() {
                    (push_char)(cx);
                }
            }
        }
    }

    // It's not really unsafe in practice, we know we have ASCII
    let mut string = unsafe { String::from_utf8_unchecked(slug) };
    if string.ends_with('-') {
        string.pop();
    }
    // We likely reserved more space than needed.
    string.shrink_to_fit();
    string
}

// extract host of url
lazy_static! {
    static ref RE_S: Regex =
        Regex::new(r"https?://").unwrap(); // let fail in test
}
lazy_static! {
    static ref RE_P: Regex =
        Regex::new(r"/.*").unwrap(); // let fail in test
}

pub fn get_host(s: &str) -> String {
    let url = RE_S.replace_all(s, "");
    let host = RE_P.replace_all(&url, "").replace("www.", "");
    host
}
