//! MQTT humidifier: <https://www.home-assistant.io/integrations/humidifier.mqtt/>

use crate::string_enum::string_enum;

// @generated begin component humidifier
use serde::{Deserialize, Serialize};

use super::Platform;
use crate::common::{
    availability::{Availability, AvailabilityMode},
    device::Device,
    entity_category::EntityCategory,
    message_expiry_interval::MessageExpiryInterval,
    nullable::Nullable,
    origin::Origin,
    qos::Qos,
};

/// An MQTT humidifier or dehumidifier with a target humidity, switched on
/// and off by publishing to `command_topic`.
///
/// <https://www.home-assistant.io/integrations/humidifier.mqtt/>
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "builder", derive(derive_builder::Builder))]
#[cfg_attr(
    feature = "builder",
    builder(setter(into, strip_option), build_fn(validate = "Self::validate"))
)]
pub struct Humidifier {
    /// Template to extract the current action from `action_topic`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub action_template: Option<String>,
    /// Topic to receive the current action (`off`, `humidifying`, `drying`,
    /// `idle`) from.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub action_topic: Option<String>,
    /// Topics that report the availability of the entity.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub availability: Vec<Availability>,
    /// How the `availability` topics are combined. Default: `latest`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub availability_mode: Option<AvailabilityMode>,
    /// Template to generate the payload sent to `command_topic`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub command_template: Option<String>,
    /// Topic to publish on/off commands to. Required.
    pub command_topic: String,
    /// Template to extract the current humidity from
    /// `current_humidity_topic`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub current_humidity_template: Option<String>,
    /// Topic to receive the current humidity from. `payload_reset_humidity`
    /// resets it.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub current_humidity_topic: Option<String>,
    /// Entity ID to use instead of one generated from the name, e.g.
    /// `humidifier.bedroom_humidifier`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub default_entity_id: Option<String>,
    /// The device this humidifier belongs to. Only works when `unique_id` is set.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub device: Option<Device>,
    /// Type of the device; [`Nullable::Null`] for none. Default:
    /// `humidifier`.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::common::nullable::deserialize"
    )]
    #[cfg_attr(feature = "builder", builder(default))]
    pub device_class: Option<Nullable<HumidifierDeviceClass>>,
    /// Whether the entity is enabled when first added. Default: `true`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub enabled_by_default: Option<bool>,
    /// Encoding of received and published payloads; an empty string disables
    /// decoding of received payloads. Default: `utf-8`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub encoding: Option<String>,
    /// Category of the entity.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub entity_category: Option<EntityCategory>,
    /// URL of a picture for the entity.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub entity_picture: Option<String>,
    /// Unique IDs of member entities, making this a group humidifier. Home
    /// Assistant also accepts a single string.
    /// Not in the humidifier docs; accepted by Home Assistant.
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::common::one_or_many::deserialize"
    )]
    #[cfg_attr(feature = "builder", builder(default))]
    pub group: Vec<String>,
    /// Icon for the entity, e.g. `mdi:air-humidifier`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub icon: Option<String>,
    /// Template to extract a JSON dictionary of attributes from messages on
    /// `json_attributes_topic`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub json_attributes_template: Option<String>,
    /// Topic whose JSON dictionary payload is added as entity attributes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub json_attributes_topic: Option<String>,
    /// Maximum target humidity in percent. Default: `100`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub max_humidity: Option<f64>,
    /// Message expiry interval for command payloads.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub message_expiry_interval: Option<MessageExpiryInterval>,
    /// Minimum target humidity in percent. Default: `0`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub min_humidity: Option<f64>,
    /// Template to generate the payload sent to `mode_command_topic`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub mode_command_template: Option<String>,
    /// Topic to publish mode commands to. Must be set together with `modes`;
    /// the builder checks this.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub mode_command_topic: Option<String>,
    /// Template to extract the mode from `mode_state_topic`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub mode_state_template: Option<String>,
    /// Topic to receive the mode from.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub mode_state_topic: Option<String>,
    /// Available modes, e.g. `normal`, `eco` or `baby`; custom modes are
    /// allowed. Must be set together with `mode_command_topic`. Home
    /// Assistant also accepts a single string.
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::common::one_or_many::deserialize"
    )]
    #[cfg_attr(feature = "builder", builder(default))]
    pub modes: Vec<String>,
    /// Name of the humidifier; [`Nullable::Null`] if only the device name is
    /// relevant.
    /// Default: `MQTT humidifier` per the docs (`MQTT Humidifier` in Home Assistant's source).
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::common::nullable::deserialize"
    )]
    #[cfg_attr(feature = "builder", builder(default))]
    pub name: Option<Nullable<String>>,
    /// Assume the state changes as commanded. Default: `true` without
    /// `state_topic`, otherwise `false`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub optimistic: Option<bool>,
    /// The application that sent the discovery message.
    /// Not in the humidifier docs; accepted by Home Assistant.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub origin: Option<Origin>,
    /// Payload that means off. Default: `OFF`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub payload_off: Option<String>,
    /// Payload that means on. Default: `ON`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub payload_on: Option<String>,
    /// Payload that resets the target or current humidity to unknown.
    /// Default: the string `None`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub payload_reset_humidity: Option<String>,
    /// Payload on `mode_state_topic` that resets the mode to unknown.
    /// Default: the string `None`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub payload_reset_mode: Option<String>,
    /// QoS level for subscribing and publishing. Default: `0`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub qos: Option<Qos>,
    /// Publish commands with the retain flag. Default: `true` per the docs
    /// (`false` in Home Assistant's source).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub retain: Option<bool>,
    /// Topic to receive the on/off state from. A `None` payload resets it.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub state_topic: Option<String>,
    /// Template to extract the on/off state from `state_topic`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub state_value_template: Option<String>,
    /// Template to generate the payload sent to
    /// `target_humidity_command_topic`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub target_humidity_command_template: Option<String>,
    /// Topic to publish target humidity commands to. Required.
    pub target_humidity_command_topic: String,
    /// Template to extract the target humidity from
    /// `target_humidity_state_topic`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub target_humidity_state_template: Option<String>,
    /// Topic to receive the target humidity from.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub target_humidity_state_topic: Option<String>,
    /// ID that uniquely identifies this humidifier. Required for device discovery
    /// and for `device`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub unique_id: Option<String>,
    /// Whether the entity is visible on dashboards when first added. Default:
    /// `true`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub visible_by_default: Option<bool>,
}

#[cfg(feature = "builder")]
impl Humidifier {
    /// Returns a builder for [`Humidifier`].
    pub fn builder() -> HumidifierBuilder {
        HumidifierBuilder::default()
    }
}

impl Platform for Humidifier {
    const PLATFORM: &'static str = "humidifier";
}
// @generated end

#[cfg(feature = "builder")]
impl HumidifierBuilder {
    /// Home Assistant requires `mode_command_topic` and `modes` together.
    fn validate(&self) -> Result<(), String> {
        let topic = self
            .mode_command_topic
            .as_ref()
            .is_some_and(Option::is_some);
        let modes = self.modes.as_ref().is_some_and(|modes| !modes.is_empty());
        if topic != modes {
            return Err("`mode_command_topic` and `modes` must be set together".into());
        }
        Ok(())
    }
}

string_enum! {
    /// Device class of a humidifier.
    ///
    /// <https://www.home-assistant.io/integrations/humidifier/#device-class>
    ///
    /// Source: `homeassistant/components/humidifier/const.py`
    pub enum HumidifierDeviceClass {
        // @generated begin device-class homeassistant/components/humidifier/const.py HumidifierDeviceClass
        /// Humidifier.
        Humidifier = "humidifier",
        /// Dehumidifier.
        Dehumidifier = "dehumidifier",
        // @generated end
    }
}
