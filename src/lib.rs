extern crate rand;
#[cfg(feature = "diesel_support")]
#[macro_use]
extern crate diesel_derive_newtype;
#[cfg(feature = "serde_support")]
#[macro_use]
extern crate serde_derive;
#[cfg(feature = "serde_support")]
#[cfg(test)]
extern crate serde_json;

use rand::distributions::Alphanumeric;
use rand::{thread_rng, Rng};
use std::cmp;
use std::fmt;
use std::ops::Deref;
use std::str::FromStr;

#[derive(Debug)]
pub struct WebTokenParseError(());

#[cfg_attr(feature = "serde_support", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "diesel_support", derive(DieselNewType))]
#[derive(Clone)]
pub struct WebToken(String);

impl WebToken {
    pub fn new() -> Self {
        let rng = thread_rng();

        let s: String = rng
            .sample_iter(&Alphanumeric)
            .take(64)
            .map(char::from)
            .collect();
        WebToken(s)
    }
}

impl Default for WebToken {
    fn default() -> Self {
        Self::new()
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
    #[cfg(feature = "serde_support")]
    use serde_json;
    #[cfg(feature = "serde_support")]
    use std::str::FromStr;

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

    #[test]
    fn it_is_debug_display() {
        let s_token = "jfqgg1HhsYaBQtGqSLfISkIjIAwcI27bFAqS6Z3SKiiANdV9Ra25j9QzIIMTKT5l";
        let token: WebToken = s_token.parse().unwrap();

        assert_eq!(format!("{:?}", token), s_token);
        assert_eq!(format!("{}", token), s_token);
    }

    #[cfg(feature = "serde_support")]
    #[test]
    fn it_is_serde() {
        #[derive(Serialize, Deserialize)]
        struct WebTokenTest {
            token: WebToken,
        }

        let token = WebTokenTest {
            token: WebToken::from_str(
                "AmJ8JS6Jt47UiV8QaCmpgBdWawLeHVuKpbReOV6uKPLqnnfbqbMAXAXxTnWYZ7RR",
            )
            .unwrap(),
        };

        let serialized = serde_json::to_string(&token).unwrap();
        assert_eq!(
            r#"{"token":"AmJ8JS6Jt47UiV8QaCmpgBdWawLeHVuKpbReOV6uKPLqnnfbqbMAXAXxTnWYZ7RR"}"#,
            serialized
        );

        let deserialized: WebTokenTest = serde_json::from_str(&serialized).unwrap();
        assert_eq!(token.token, deserialized.token)
    }
}
