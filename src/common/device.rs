//! The `device` block that ties entities to a device.

use serde::{Deserialize, Serialize};

/// Information about the device an entity belongs to, used to tie it into the
/// device registry. Only works when the entity has a `unique_id`.
///
/// Home Assistant requires at least one of `identifiers` or `connections`, and
/// the MQTT discovery docs require `name` (Home Assistant itself only logs a
/// message when it is missing). The builder enforces both; deserialization
/// does not, so payloads Home Assistant would reject can still be inspected.
///
/// <https://www.home-assistant.io/integrations/sensor.mqtt/#device>
#[derive(Clone, Debug, Default, Eq, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "builder", derive(derive_builder::Builder))]
#[cfg_attr(
    feature = "builder",
    builder(setter(into, strip_option), build_fn(validate = "Self::validate"))
)]
pub struct Device {
    /// Link to the web page that manages the configuration of this device:
    /// `http://`, `https://` or an internal `homeassistant://` URL.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub configuration_url: Option<String>,
    /// Connections of the device to the outside world as
    /// `(connection_type, connection_identifier)` pairs, e.g.
    /// `("mac", "02:5b:26:a8:dc:12")`.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(
        feature = "builder",
        builder(default, setter(each(name = "connection")))
    )]
    pub connections: Vec<(String, String)>,
    /// Hardware version of the device.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub hw_version: Option<String>,
    /// IDs that uniquely identify the device, e.g. a serial number. Home
    /// Assistant also accepts a single string, which is read as a list of one.
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::common::string_list::deserialize"
    )]
    #[cfg_attr(
        feature = "builder",
        builder(default, setter(each(name = "identifier", into)))
    )]
    pub identifiers: Vec<String>,
    /// Manufacturer of the device.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub manufacturer: Option<String>,
    /// Model of the device.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub model: Option<String>,
    /// Model identifier of the device.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub model_id: Option<String>,
    /// Name of the device. Optional in the platform docs and in Home
    /// Assistant, but required by the MQTT discovery docs; the builder
    /// requires it.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Serial number of the device.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub serial_number: Option<String>,
    /// Area to suggest if the device is not in one yet.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub suggested_area: Option<String>,
    /// Firmware version of the device.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub sw_version: Option<String>,
    /// Identifier of a device that routes messages between this device and
    /// Home Assistant.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub via_device: Option<String>,
}

#[cfg(feature = "builder")]
impl Device {
    /// Returns a builder for [`Device`].
    pub fn builder() -> DeviceBuilder {
        DeviceBuilder::default()
    }
}

#[cfg(feature = "builder")]
impl DeviceBuilder {
    fn validate(&self) -> Result<(), String> {
        let is_empty = |list: &Option<Vec<_>>| list.as_ref().is_none_or(Vec::is_empty);
        if is_empty(&self.identifiers) && self.connections.as_ref().is_none_or(Vec::is_empty) {
            return Err("at least one of `identifiers` or `connections` must be set".into());
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::assert_eq;

    /// Device block of the device discovery example in the MQTT docs
    /// (<https://www.home-assistant.io/integrations/mqtt/>), expanded.
    #[test]
    fn identifiers_as_single_string() {
        let device: Device = serde_json::from_str(
            r#"{
                "identifiers": "ea334450945afc",
                "name": "Kitchen",
                "manufacturer": "Bla electronics",
                "model": "xya",
                "sw_version": "1.0",
                "serial_number": "ea334450945afc",
                "hw_version": "1.0rev2"
            }"#,
        )
        .unwrap();
        assert_eq!(device.identifiers, ["ea334450945afc"]);
        assert_eq!(
            serde_json::to_value(&device).unwrap()["identifiers"],
            serde_json::json!(["ea334450945afc"])
        );
    }

    #[test]
    fn connections() {
        let json = r#"{"connections":[["mac","02:5b:26:a8:dc:12"]]}"#;
        let device: Device = serde_json::from_str(json).unwrap();
        assert_eq!(
            device.connections,
            [("mac".to_string(), "02:5b:26:a8:dc:12".to_string())]
        );
        assert_eq!(serde_json::to_string(&device).unwrap(), json);
    }

    #[test]
    fn empty_lists_are_skipped() {
        assert_eq!(serde_json::to_string(&Device::default()).unwrap(), "{}");
    }

    #[cfg(feature = "builder")]
    #[test]
    fn builder_requires_name_and_identification() {
        assert!(Device::builder().identifier("abc").build().is_err());
        assert!(Device::builder().name("Kitchen").build().is_err());
        let device = Device::builder()
            .name("Kitchen")
            .connection(("mac".to_string(), "02:5b:26:a8:dc:12".to_string()))
            .build()
            .unwrap();
        assert_eq!(device.name.as_deref(), Some("Kitchen"));
        assert!(
            Device::builder()
                .name("Kitchen")
                .identifier("abc")
                .build()
                .is_ok()
        );
    }
}
