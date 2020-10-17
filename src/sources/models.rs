use std::fmt::{Display, Formatter};
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub enum Source {
    Yahoo,
    Poloniex,
}

impl Display for Source {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        f.write_str(&format!("{:?}", self))
    }
}

impl FromStr for Source {
    type Err = ();

    fn from_str(s: &str) -> Result<Source, Self::Err> {
        match s {
            "Yahoo" => Ok(Source::Yahoo),
            "Poloniex" => Ok(Source::Poloniex),
            _ => Err(()),
        }
    }
}

mod tests {
    use crate::sources::models::Source;

    #[test]
    fn test_display() {
        assert_eq!(Source::Yahoo.to_string(), "Yahoo");
        assert_eq!(Source::Poloniex.to_string(), "Poloniex");
    }

    #[test]
    fn test_from_str() {
        let api: Source = "Yahoo".parse().unwrap();
        assert_eq!(api, Source::Yahoo);
    }
}
