//! Values that can also be an explicit `null`.

use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// A value that Home Assistant also accepts as an explicit `null`, with a
/// meaning different from leaving the key out.
///
/// Such fields are `Option<Nullable<T>>`, which has three states:
///
/// - `None`: the key is absent and Home Assistant uses its default.
/// - `Some(Nullable::Null)`: serialized as `null`. For an entity `name` this
///   means only the device name is shown; for a cover command payload it
///   disables that command.
/// - `Some(Nullable::Value(..))`: the value.
///
/// <https://www.home-assistant.io/integrations/sensor.mqtt/#name>
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum Nullable<T> {
    /// An explicit `null`.
    Null,
    /// A value.
    Value(T),
}

impl<T> Nullable<T> {
    /// Returns the value, or `None` for [`Nullable::Null`].
    pub fn as_ref(&self) -> Option<&T> {
        match self {
            Self::Null => None,
            Self::Value(value) => Some(value),
        }
    }
}

impl Nullable<String> {
    /// Returns the string, or `None` for [`Nullable::Null`].
    pub fn as_str(&self) -> Option<&str> {
        self.as_ref().map(String::as_str)
    }
}

impl From<&str> for Nullable<String> {
    fn from(value: &str) -> Self {
        Self::Value(value.into())
    }
}

impl From<String> for Nullable<String> {
    fn from(value: String) -> Self {
        Self::Value(value)
    }
}

impl<T> From<Option<T>> for Nullable<T> {
    fn from(value: Option<T>) -> Self {
        value.map_or(Self::Null, Self::Value)
    }
}

impl<T: Serialize> Serialize for Nullable<T> {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.as_ref().serialize(serializer)
    }
}

impl<'de, T: Deserialize<'de>> Deserialize<'de> for Nullable<T> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Option::<T>::deserialize(deserializer).map(Into::into)
    }
}

/// Deserializes an `Option<Nullable<T>>` field so that `null` becomes
/// `Some(Nullable::Null)`. Use with `#[serde(default)]` so that an absent key
/// becomes `None`.
pub(crate) fn deserialize<'de, D, T>(deserializer: D) -> Result<Option<Nullable<T>>, D::Error>
where
    D: Deserializer<'de>,
    T: Deserialize<'de>,
{
    Nullable::deserialize(deserializer).map(Some)
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Default, PartialEq, Serialize, Deserialize)]
    struct Test {
        #[serde(
            default,
            skip_serializing_if = "Option::is_none",
            deserialize_with = "super::deserialize"
        )]
        name: Option<Nullable<String>>,
    }

    #[test]
    fn three_states() {
        let cases = [
            ("{}", None),
            (r#"{"name":null}"#, Some(Nullable::Null)),
            (r#"{"name":"Power"}"#, Some(Nullable::from("Power"))),
        ];
        for (json, name) in cases {
            let test: Test = serde_json::from_str(json).unwrap();
            assert_eq!(test.name, name, "{json}");
            assert_eq!(serde_json::to_string(&test).unwrap(), json);
        }
    }
}
