//! Serde helper for fields Home Assistant accepts as a single string or a list
//! of strings (`cv.ensure_list`).

use serde::{Deserialize, Deserializer};

/// Deserializes a string or a list of strings into a `Vec<String>`.
pub(crate) fn deserialize<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<Vec<String>, D::Error> {
    #[derive(Deserialize)]
    #[serde(untagged)]
    enum StringOrList {
        String(String),
        List(Vec<String>),
    }

    Ok(match StringOrList::deserialize(deserializer)? {
        StringOrList::String(value) => vec![value],
        StringOrList::List(values) => values,
    })
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use serde::Deserialize;

    #[derive(Debug, Deserialize)]
    struct Test {
        #[serde(default, deserialize_with = "super::deserialize")]
        values: Vec<String>,
    }

    fn parse(json: &str) -> Result<Vec<String>, serde_json::Error> {
        serde_json::from_str::<Test>(json).map(|test| test.values)
    }

    #[test]
    fn accepts_string_list_and_absence() {
        assert_eq!(parse(r#"{"values": "a"}"#).unwrap(), ["a"]);
        assert_eq!(parse(r#"{"values": ["a", "b"]}"#).unwrap(), ["a", "b"]);
        assert_eq!(parse(r#"{"values": []}"#).unwrap(), Vec::<String>::new());
        assert_eq!(parse("{}").unwrap(), Vec::<String>::new());
    }

    #[test]
    fn rejects_other_types() {
        assert!(parse(r#"{"values": 1}"#).is_err());
        assert!(parse(r#"{"values": [1]}"#).is_err());
        assert!(parse(r#"{"values": null}"#).is_err());
    }
}
