//! MQTT vacuum: <https://www.home-assistant.io/integrations/vacuum.mqtt/>

use crate::string_enum::string_enum;

// @generated begin component vacuum
use serde::{Deserialize, Serialize};

use crate::common::{
    availability::{Availability, AvailabilityMode},
    device::Device,
    entity_category::EntityCategory,
    message_expiry_interval::MessageExpiryInterval,
    nullable::Nullable,
    origin::Origin,
    qos::Qos,
};
use crate::component::Platform;

/// An MQTT vacuum: controlled by publishing to `command_topic`; its state
/// is received as a JSON object on `state_topic`.
///
/// <https://www.home-assistant.io/integrations/vacuum.mqtt/>
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "builder", derive(derive_builder::Builder))]
#[cfg_attr(
    feature = "builder",
    builder(setter(into, strip_option), build_fn(validate = "Self::validate"))
)]
pub struct Vacuum {
    /// Topics that report the availability of the entity.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub availability: Vec<Availability>,
    /// How the `availability` topics are combined. Default: `latest`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub availability_mode: Option<AvailabilityMode>,
    /// Template to generate the payload sent to
    /// `clean_segments_command_topic`; `value` is the list of segment IDs.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub clean_segments_command_template: Option<String>,
    /// Topic to publish a JSON list of segment IDs to clean to. Requires
    /// `unique_id`; the builder checks this.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub clean_segments_command_topic: Option<String>,
    /// Topic to publish commands such as `payload_start` to.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub command_topic: Option<String>,
    /// Entity ID to use instead of one generated from the name, e.g.
    /// `vacuum.living_room`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub default_entity_id: Option<String>,
    /// The device this vacuum belongs to. Only works when `unique_id` is set.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub device: Option<Device>,
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
    /// Not in the vacuum docs; accepted by Home Assistant.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub entity_category: Option<EntityCategory>,
    /// URL of a picture for the entity.
    /// Not in the vacuum docs; accepted by Home Assistant.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub entity_picture: Option<String>,
    /// Possible fan speeds. Home Assistant also accepts a single string.
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::common::one_or_many::deserialize"
    )]
    #[cfg_attr(feature = "builder", builder(default))]
    pub fan_speed_list: Vec<String>,
    /// Unique IDs of member entities, making this a group vacuum. Home
    /// Assistant also accepts a single string.
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::common::one_or_many::deserialize"
    )]
    #[cfg_attr(feature = "builder", builder(default))]
    pub group: Vec<String>,
    /// Icon for the entity, e.g. `mdi:robot-vacuum`.
    /// Not in the vacuum docs; accepted by Home Assistant.
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
    /// Name of the vacuum; [`Nullable::Null`] if only the device name is
    /// relevant.
    /// Default: `MQTT Vacuum` per the docs (`MQTT State Vacuum` in Home Assistant's source).
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::common::nullable::deserialize"
    )]
    #[cfg_attr(feature = "builder", builder(default))]
    pub name: Option<Nullable<String>>,
    /// The application that sent the discovery message.
    /// Not in the vacuum docs; accepted by Home Assistant.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub origin: Option<Origin>,
    /// Payload sent to start a spot cleaning cycle. Default: `clean_spot`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub payload_clean_spot: Option<String>,
    /// Payload sent to locate the vacuum. Default: `locate`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub payload_locate: Option<String>,
    /// Payload sent to pause cleaning. Default: `pause`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub payload_pause: Option<String>,
    /// Payload sent to return the vacuum to its base. Default:
    /// `return_to_base`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub payload_return_to_base: Option<String>,
    /// Payload sent to start cleaning. Default: `start`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub payload_start: Option<String>,
    /// Payload sent to stop cleaning. Default: `stop`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub payload_stop: Option<String>,
    /// QoS level for subscribing and publishing. Default: `0`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub qos: Option<Qos>,
    /// Publish commands with the retain flag. Default: `false`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub retain: Option<bool>,
    /// Topic to publish custom commands to.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub send_command_topic: Option<String>,
    /// Topic to publish fan speed commands to.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub set_fan_speed_topic: Option<String>,
    /// Topic to receive the state from, a JSON object with `state` and
    /// optionally `fan_speed` and `segments`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub state_topic: Option<String>,
    /// Features the vacuum supports. Default: `start`, `stop`, `return_home`,
    /// `status` and `clean_spot` per the docs (without `status` in Home
    /// Assistant's source). Home Assistant also accepts a single string.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::common::one_or_many::deserialize_option"
    )]
    #[cfg_attr(feature = "builder", builder(default))]
    pub supported_features: Option<Vec<VacuumFeature>>,
    /// ID that uniquely identifies this vacuum. Required for device discovery
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
impl Vacuum {
    /// Returns a builder for [`Vacuum`].
    pub fn builder() -> VacuumBuilder {
        VacuumBuilder::default()
    }
}

impl Platform for Vacuum {
    const PLATFORM: &'static str = "vacuum";
}
// @generated end

#[cfg(feature = "builder")]
impl VacuumBuilder {
    /// Home Assistant requires `unique_id` with `clean_segments_command_topic`.
    fn validate(&self) -> Result<(), String> {
        let set = |field: &Option<Option<String>>| field.as_ref().is_some_and(Option::is_some);
        if set(&self.clean_segments_command_topic) && !set(&self.unique_id) {
            return Err("`clean_segments_command_topic` requires `unique_id`".into());
        }
        Ok(())
    }
}

string_enum! {
    /// A feature of a vacuum, for `supported_features`.
    ///
    /// <https://www.home-assistant.io/integrations/vacuum.mqtt/#supported_features>
    ///
    /// Source: `SERVICE_TO_STRING` in `homeassistant/components/mqtt/vacuum.py`
    pub enum VacuumFeature {
        /// The vacuum can start cleaning.
        Start = "start",
        /// The vacuum can pause cleaning.
        Pause = "pause",
        /// The vacuum can stop cleaning.
        Stop = "stop",
        /// The vacuum can return to its base.
        ReturnHome = "return_home",
        /// The fan speed can be set.
        FanSpeed = "fan_speed",
        /// The vacuum reports a status. Deprecated in Home Assistant.
        Status = "status",
        /// Custom commands can be sent.
        SendCommand = "send_command",
        /// The vacuum can be located.
        Locate = "locate",
        /// The vacuum can clean a spot.
        CleanSpot = "clean_spot",
    }
}
