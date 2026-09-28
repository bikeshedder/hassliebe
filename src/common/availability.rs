//! Availability topics of an entity.

use serde::{Deserialize, Serialize};

use crate::string_enum::string_enum;

/// One entry of an entity's `availability` list: a topic that reports whether
/// the entity is available.
///
/// <https://www.home-assistant.io/integrations/sensor.mqtt/#availability>
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "builder", derive(derive_builder::Builder))]
#[cfg_attr(feature = "builder", builder(setter(into, strip_option)))]
pub struct Availability {
    /// Payload that means "available". Home Assistant's default is `online`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub payload_available: Option<String>,
    /// Payload that means "not available". Home Assistant's default is
    /// `offline`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub payload_not_available: Option<String>,
    /// MQTT topic to subscribe to for availability updates. Required.
    pub topic: String,
    /// Template to extract the device's availability from `topic`. It must
    /// render to `payload_available` or `payload_not_available`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub value_template: Option<String>,
}

#[cfg(feature = "builder")]
impl Availability {
    /// Returns a builder for [`Availability`].
    pub fn builder() -> AvailabilityBuilder {
        AvailabilityBuilder::default()
    }
}

string_enum! {
    /// How the `availability` topics are combined into one availability state.
    /// Home Assistant's default is `latest`.
    ///
    /// <https://www.home-assistant.io/integrations/sensor.mqtt/#availability_mode>
    pub enum AvailabilityMode {
        /// Available only if all configured topics report available.
        All = "all",
        /// Available if at least one configured topic reports available.
        Any = "any",
        /// The last availability message received decides.
        Latest = "latest",
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    #[test]
    fn minimal() {
        let availability: Availability =
            serde_json::from_str(r#"{"topic": "home/kitchen/status"}"#).unwrap();
        assert_eq!(
            availability,
            Availability {
                topic: "home/kitchen/status".into(),
                ..Default::default()
            }
        );
        assert_eq!(
            serde_json::to_string(&availability).unwrap(),
            r#"{"topic":"home/kitchen/status"}"#
        );
    }

    #[test]
    fn round_trip_and_unknown_keys() {
        let json = r#"{
            "topic": "home/kitchen/status",
            "payload_available": "1",
            "payload_not_available": "0",
            "value_template": "{{ value_json.state }}",
            "some_future_key": true
        }"#;
        let availability: Availability = serde_json::from_str(json).unwrap();
        assert_eq!(availability.payload_available.as_deref(), Some("1"));
        let back: Availability =
            serde_json::from_str(&serde_json::to_string(&availability).unwrap()).unwrap();
        assert_eq!(back, availability);
    }

    #[test]
    fn topic_is_required() {
        assert!(serde_json::from_str::<Availability>("{}").is_err());
    }

    #[cfg(feature = "builder")]
    #[test]
    fn builder_requires_topic() {
        assert!(Availability::builder().build().is_err());
        let availability = Availability::builder()
            .topic("home/kitchen/status")
            .payload_available("1")
            .build()
            .unwrap();
        assert_eq!(availability.payload_available.as_deref(), Some("1"));
    }
}
