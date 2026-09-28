use std::fmt;

use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// The `name` of an entity, which Home Assistant also accepts as `null`.
///
/// A field of type `Option<EntityName>` has three states:
///
/// - `None`: the key is absent and Home Assistant uses the platform's default
///   name (or the name of the device class).
/// - `Some(EntityName::Null)`: serialized as `null`; the entity has no name of
///   its own and only the device name is shown.
/// - `Some(EntityName::Name(..))`: the entity's name.
///
/// <https://www.home-assistant.io/integrations/sensor.mqtt/#name>
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum EntityName {
    /// `null`: only the device name is relevant.
    Null,
    /// The entity's name.
    Name(String),
}

impl EntityName {
    /// Returns the name, or `None` for [`EntityName::Null`].
    pub fn as_str(&self) -> Option<&str> {
        match self {
            Self::Null => None,
            Self::Name(name) => Some(name),
        }
    }
}

impl fmt::Display for EntityName {
    /// Writes the name; writes nothing for [`EntityName::Null`].
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str().unwrap_or_default())
    }
}

impl From<&str> for EntityName {
    fn from(name: &str) -> Self {
        Self::Name(name.into())
    }
}

impl From<String> for EntityName {
    fn from(name: String) -> Self {
        Self::Name(name)
    }
}

impl From<Option<String>> for EntityName {
    fn from(name: Option<String>) -> Self {
        name.map_or(Self::Null, Self::Name)
    }
}

impl Serialize for EntityName {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        self.as_str().serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for EntityName {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        Option::<String>::deserialize(deserializer).map(Into::into)
    }
}

/// Deserializes an `Option<EntityName>` field so that `null` becomes
/// `Some(EntityName::Null)`. Use with `#[serde(default)]` so that an absent key
/// becomes `None`.
pub(crate) fn deserialize<'de, D: Deserializer<'de>>(
    deserializer: D,
) -> Result<Option<EntityName>, D::Error> {
    EntityName::deserialize(deserializer).map(Some)
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
        name: Option<EntityName>,
    }

    #[test]
    fn three_states() {
        let cases = [
            ("{}", None),
            (r#"{"name":null}"#, Some(EntityName::Null)),
            (r#"{"name":"Power"}"#, Some(EntityName::from("Power"))),
        ];
        for (json, name) in cases {
            let test: Test = serde_json::from_str(json).unwrap();
            assert_eq!(test.name, name, "{json}");
            assert_eq!(serde_json::to_string(&test).unwrap(), json);
        }
    }
}
