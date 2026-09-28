//! Serde helpers for fields Home Assistant accepts as a single value or a list
//! (`cv.ensure_list`).

use serde::{Deserialize, Deserializer};

#[derive(Deserialize)]
#[serde(untagged)]
enum OneOrMany<T> {
    Many(Vec<T>),
    One(T),
}

impl<T> From<OneOrMany<T>> for Vec<T> {
    fn from(value: OneOrMany<T>) -> Self {
        match value {
            OneOrMany::Many(values) => values,
            OneOrMany::One(value) => vec![value],
        }
    }
}

/// Deserializes a value or a list of values into a `Vec<T>`.
pub(crate) fn deserialize<'de, D, T>(deserializer: D) -> Result<Vec<T>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    OneOrMany::deserialize(deserializer).map(Into::into)
}

/// Like [`deserialize`], for `Option<Vec<T>>` fields (use with
/// `#[serde(default)]`).
pub(crate) fn deserialize_option<'de, D, T>(deserializer: D) -> Result<Option<Vec<T>>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    deserialize(deserializer).map(Some)
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use serde::Deserialize;

    #[derive(Debug, Deserialize)]
    struct Test {
        #[serde(default, deserialize_with = "super::deserialize")]
        values: Vec<String>,
        #[serde(default, deserialize_with = "super::deserialize_option")]
        numbers: Option<Vec<u8>>,
    }

    fn parse(json: &str) -> Result<Test, serde_json::Error> {
        serde_json::from_str::<Test>(json)
    }

    #[test]
    fn accepts_one_many_and_absence() {
        assert_eq!(parse(r#"{"values": "a"}"#).unwrap().values, ["a"]);
        assert_eq!(
            parse(r#"{"values": ["a", "b"]}"#).unwrap().values,
            ["a", "b"]
        );
        assert_eq!(
            parse(r#"{"values": []}"#).unwrap().values,
            Vec::<String>::new()
        );
        assert_eq!(parse("{}").unwrap().values, Vec::<String>::new());
        assert_eq!(parse(r#"{"numbers": 1}"#).unwrap().numbers, Some(vec![1]));
        assert_eq!(
            parse(r#"{"numbers": [1, 2]}"#).unwrap().numbers,
            Some(vec![1, 2])
        );
        assert_eq!(parse("{}").unwrap().numbers, None);
    }

    #[test]
    fn rejects_other_types() {
        assert!(parse(r#"{"values": 1}"#).is_err());
        assert!(parse(r#"{"values": [1]}"#).is_err());
        assert!(parse(r#"{"values": null}"#).is_err());
        assert!(parse(r#"{"numbers": "x"}"#).is_err());
    }
}
