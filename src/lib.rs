extern crate rand;

use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use std::cmp;
use std::fmt;
use std::ops::Deref;
use std::str::FromStr;

#[derive(Debug)]
pub struct WebTokenParseError(());

pub struct WebToken(String);

impl WebToken {
    pub fn new() -> Self {
        let mut rng = thread_rng();

        let s: String = rng.sample_iter(&Alphanumeric).take(64).collect();
        WebToken(s)
    }
}

impl fmt::Display for WebToken {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl fmt::Debug for WebToken {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl cmp::PartialEq for WebToken {
    fn eq(&self, other: &WebToken) -> bool {
        self.0 == other.0
    }
}

impl<'a> cmp::PartialEq<&'a str> for WebToken {
    fn eq(&self, other: &&'a str) -> bool {
        &self.0 == other
    }
}

impl cmp::PartialEq<String> for WebToken {
    fn eq(&self, other: &String) -> bool {
        &self.0 == other
    }
}

impl Deref for WebToken {
    type Target = String;

    fn deref(&self) -> &String {
        &self.0
    }
}

impl FromStr for WebToken {
    type Err = WebTokenParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() < 64 {
            return Err(WebTokenParseError(()));
        }

        for c in s.chars() {
            if !c.is_alphanumeric() {
                return Err(WebTokenParseError(()));
            }
        }

        Ok(WebToken(String::from(s)))
    }
}

#[cfg(test)]
mod tests {
    use super::WebToken;

    #[test]
    fn it_generates_a_random_token_of_64_chars() {
        assert_eq!(WebToken::new().len(), 64);
    }

    #[test]
    fn it_is_built_from_str() {
        "jfqgg1HhsYaBQtGqSLfISkIjIAwcI27bFAqS6Z3SKiiANdV9Ra25j9QzIIMTKT5l"
            .parse::<WebToken>()
            .unwrap();
    }

    #[test]
    #[should_panic]
    fn it_is_built_from_alphanums_only() {
        "jfqgg1HhsYaBQtGqSLfISkIjIAwcI27bFAqS6Z3SKiiANdV9Ra25j9QzIIMTKT5l___"
            .parse::<WebToken>()
            .unwrap();
    }

    #[test]
    #[should_panic]
    fn it_has_at_least_64_chars() {
        // 63 chars length str
        "jfqgg1HhsYaBQtGqSLfISkIjIAwcI27bFAqS6Z3SKiiANdV9Ra25j9QzIIMTKT5"
            .parse::<WebToken>()
            .unwrap();
    }

    #[test]
    fn it_is_comparable_to_str() {
        let token = WebToken::new();
        let s_str = &token.0[..]; // &str
        let s_string = token.0.clone(); // String
        assert_eq!(token, s_str);
        assert_eq!(token, s_string);
    }

    #[test]
    fn it_is_comparable_to_another_token() {
        let token = WebToken::new();
        let token1: WebToken = token.0.parse().unwrap();
        assert_eq!(token, token1);
    }
}
