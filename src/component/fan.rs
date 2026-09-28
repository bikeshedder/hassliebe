//! MQTT fan: <https://www.home-assistant.io/integrations/fan.mqtt/>

// @generated begin component fan
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

/// An MQTT fan: switched on and off by publishing to `command_topic`, with
/// optional speed, preset mode, oscillation and direction.
///
/// <https://www.home-assistant.io/integrations/fan.mqtt/>
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "builder", derive(derive_builder::Builder))]
#[cfg_attr(
    feature = "builder",
    builder(setter(into, strip_option), build_fn(validate = "Self::validate"))
)]
pub struct Fan {
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
    /// Entity ID to use instead of one generated from the name, e.g.
    /// `fan.bedroom`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub default_entity_id: Option<String>,
    /// The device this fan belongs to. Only works when `unique_id` is set.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub device: Option<Device>,
    /// Template to generate the payload sent to `direction_command_topic`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub direction_command_template: Option<String>,
    /// Topic to publish direction commands to.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub direction_command_topic: Option<String>,
    /// Topic to receive the direction from.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub direction_state_topic: Option<String>,
    /// Template to extract the direction from messages on
    /// `direction_state_topic`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub direction_value_template: Option<String>,
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
    /// Unique IDs of member entities, making this a group fan. Home
    /// Assistant also accepts a single string.
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::common::one_or_many::deserialize"
    )]
    #[cfg_attr(feature = "builder", builder(default))]
    pub group: Vec<String>,
    /// Icon for the entity, e.g. `mdi:ceiling-fan`.
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
    /// Message expiry interval for command payloads.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub message_expiry_interval: Option<MessageExpiryInterval>,
    /// Name of the fan; [`Nullable::Null`] if only the device name is
    /// relevant. Default: `MQTT Fan`.
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
    /// Not in the fan docs; accepted by Home Assistant.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub origin: Option<Origin>,
    /// Template to generate the payload sent to `oscillation_command_topic`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub oscillation_command_template: Option<String>,
    /// Topic to publish oscillation commands to.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub oscillation_command_topic: Option<String>,
    /// Topic to receive the oscillation state from.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub oscillation_state_topic: Option<String>,
    /// Template to extract the oscillation state from messages on
    /// `oscillation_state_topic`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub oscillation_value_template: Option<String>,
    /// Payload that means off. Default: `OFF`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub payload_off: Option<String>,
    /// Payload that means on. Default: `ON`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub payload_on: Option<String>,
    /// Payload that means oscillation off. Default: `oscillate_off`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub payload_oscillation_off: Option<String>,
    /// Payload that means oscillation on. Default: `oscillate_on`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub payload_oscillation_on: Option<String>,
    /// Payload on `percentage_state_topic` that resets the percentage to
    /// unknown. Default: the string `None`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub payload_reset_percentage: Option<String>,
    /// Payload on `preset_mode_state_topic` that resets the preset mode to
    /// unknown. Default: the string `None`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub payload_reset_preset_mode: Option<String>,
    /// Template to generate the payload sent to `percentage_command_topic`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub percentage_command_template: Option<String>,
    /// Topic to publish speed commands to, in the range `speed_range_min` to
    /// `speed_range_max`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub percentage_command_topic: Option<String>,
    /// Topic to receive the speed from, in the range `speed_range_min` to
    /// `speed_range_max`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub percentage_state_topic: Option<String>,
    /// Template to extract the speed from messages on `percentage_state_topic`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub percentage_value_template: Option<String>,
    /// Template to generate the payload sent to `preset_mode_command_topic`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub preset_mode_command_template: Option<String>,
    /// Topic to publish preset mode commands to. Requires `preset_modes`; the
    /// builder checks this.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub preset_mode_command_topic: Option<String>,
    /// Topic to receive the preset mode from.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub preset_mode_state_topic: Option<String>,
    /// Template to extract the preset mode from messages on
    /// `preset_mode_state_topic`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub preset_mode_value_template: Option<String>,
    /// Supported preset modes, e.g. `auto` or `eco`. Requires
    /// `preset_mode_command_topic`. Home Assistant also accepts a single string.
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::common::one_or_many::deserialize"
    )]
    #[cfg_attr(feature = "builder", builder(default))]
    pub preset_modes: Vec<String>,
    /// QoS level for subscribing and publishing. Default: `0`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub qos: Option<Qos>,
    /// Publish commands with the retain flag. Default: `true` per the docs
    /// (`false` in Home Assistant's source).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub retain: Option<bool>,
    /// Speed value that means 100 %; greater than `speed_range_min`.
    /// Default: `100`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub speed_range_max: Option<u32>,
    /// Lowest speed value above off; at least `1`. Default: `1`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub speed_range_min: Option<u32>,
    /// Topic to receive the on/off state from; the payload `None` resets it
    /// to unknown.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub state_topic: Option<String>,
    /// Template to extract the on/off state from messages on `state_topic`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub state_value_template: Option<String>,
    /// ID that uniquely identifies this fan. Required for device discovery
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
impl Fan {
    /// Returns a builder for [`Fan`].
    pub fn builder() -> FanBuilder {
        FanBuilder::default()
    }
}

impl Platform for Fan {
    const PLATFORM: &'static str = "fan";
}
// @generated end

#[cfg(feature = "builder")]
impl FanBuilder {
    /// Home Assistant requires `preset_mode_command_topic` and `preset_modes`
    /// together.
    fn validate(&self) -> Result<(), String> {
        let topic = self
            .preset_mode_command_topic
            .as_ref()
            .is_some_and(Option::is_some);
        let modes = self
            .preset_modes
            .as_ref()
            .is_some_and(|modes| !modes.is_empty());
        if topic != modes {
            return Err(
                "`preset_mode_command_topic` and `preset_modes` must be set together".into(),
            );
        }
        Ok(())
    }
}
