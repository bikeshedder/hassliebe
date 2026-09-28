//! MQTT quality of service levels.

use std::fmt;

use serde::{Deserialize, Deserializer, Serialize, Serializer, de};

/// MQTT quality of service level, serialized as the integers `0`, `1` and `2`.
///
/// Home Assistant's default is `0` for all platforms.
///
/// <https://www.home-assistant.io/integrations/mqtt/>
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
#[repr(u8)]
pub enum Qos {
    /// QoS 0: at most once.
    AtMostOnce = 0,
    /// QoS 1: at least once.
    AtLeastOnce = 1,
    /// QoS 2: exactly once.
    ExactlyOnce = 2,
}

impl From<Qos> for u8 {
    fn from(qos: Qos) -> Self {
        qos as u8
    }
}

impl TryFrom<u8> for Qos {
    type Error = InvalidQos;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::AtMostOnce),
            1 => Ok(Self::AtLeastOnce),
            2 => Ok(Self::ExactlyOnce),
            _ => Err(InvalidQos(value)),
        }
    }
}

/// Error returned when converting an integer other than `0`, `1` or `2` into
/// [`Qos`].
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct InvalidQos(pub u8);

impl fmt::Display for InvalidQos {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "invalid QoS {}, expected 0, 1 or 2", self.0)
    }
}

impl std::error::Error for InvalidQos {}

impl Serialize for Qos {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_u8(u8::from(*self))
    }
}

impl<'de> Deserialize<'de> for Qos {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = u8::deserialize(deserializer)?;
        Qos::try_from(value).map_err(|_| {
            de::Error::invalid_value(de::Unexpected::Unsigned(value.into()), &"0, 1 or 2")
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn serializes_as_integers() {
        assert_eq!(serde_json::to_string(&Qos::AtMostOnce).unwrap(), "0");
        assert_eq!(serde_json::to_string(&Qos::AtLeastOnce).unwrap(), "1");
        assert_eq!(serde_json::to_string(&Qos::ExactlyOnce).unwrap(), "2");
    }

    #[test]
    fn deserializes_integers() {
        let parse = |s| serde_json::from_str::<Qos>(s).unwrap();
        assert_eq!(parse("0"), Qos::AtMostOnce);
        assert_eq!(parse("1"), Qos::AtLeastOnce);
        assert_eq!(parse("2"), Qos::ExactlyOnce);
    }

    #[test]
    fn rejects_other_values() {
        for s in ["3", "-1", "256", "1.5", r#""1""#, r#""AtMostOnce""#] {
            assert!(serde_json::from_str::<Qos>(s).is_err(), "{s}");
        }
    }

    #[test]
    fn converts_from_and_to_u8() {
        assert_eq!(Qos::try_from(1), Ok(Qos::AtLeastOnce));
        assert_eq!(Qos::try_from(3), Err(InvalidQos(3)));
        assert_eq!(u8::from(Qos::ExactlyOnce), 2);
    }
}
