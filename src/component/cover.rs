//! MQTT cover: <https://www.home-assistant.io/integrations/cover.mqtt/>

// @generated begin component cover
use serde::{Deserialize, Serialize};

use super::Platform;
use crate::{
    common::{
        availability::{Availability, AvailabilityMode},
        device::Device,
        entity_category::EntityCategory,
        message_expiry_interval::MessageExpiryInterval,
        nullable::Nullable,
        origin::Origin,
        qos::Qos,
    },
    string_enum::string_enum,
};

/// An MQTT cover, such as a blind, shutter or garage door.
///
/// <https://www.home-assistant.io/integrations/cover.mqtt/>
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "builder", derive(derive_builder::Builder))]
#[cfg_attr(feature = "builder", builder(setter(into, strip_option)))]
pub struct Cover {
    /// Topics that report the availability of the entity.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub availability: Vec<Availability>,
    /// How the `availability` topics are combined. Default: `latest`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub availability_mode: Option<AvailabilityMode>,
    /// Topic to publish open, close and stop commands to.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub command_topic: Option<String>,
    /// Entity ID to use instead of one generated from the name, e.g.
    /// `cover.living_room`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub default_entity_id: Option<String>,
    /// The device this cover belongs to. Only works when `unique_id` is set.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub device: Option<Device>,
    /// Type of the cover, which sets its icon and the text shown for its
    /// states.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub device_class: Option<CoverDeviceClass>,
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
    /// Unique IDs of member entities, making this a group cover. Home
    /// Assistant also accepts a single string.
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::common::string_list::deserialize"
    )]
    #[cfg_attr(feature = "builder", builder(default))]
    pub group: Vec<String>,
    /// Icon for the entity, e.g. `mdi:blinds`.
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
    /// Name of the cover; [`Nullable::Null`] if only the device name is
    /// relevant. Default: `MQTT Cover`.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::common::nullable::deserialize"
    )]
    #[cfg_attr(feature = "builder", builder(default))]
    pub name: Option<Nullable<String>>,
    /// Assume the state changes as commanded. Default: `false`, or `true`
    /// without `state_topic` and `position_topic`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub optimistic: Option<bool>,
    /// The application that sent the discovery message. Not in the cover
    /// docs; accepted by Home Assistant for all MQTT entities.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub origin: Option<Origin>,
    /// Payload sent to close the cover; [`Nullable::Null`] disables the command.
    /// Default: `CLOSE`.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::common::nullable::deserialize"
    )]
    #[cfg_attr(feature = "builder", builder(default))]
    pub payload_close: Option<Nullable<String>>,
    /// Payload sent to open the cover; [`Nullable::Null`] disables the command.
    /// Default: `OPEN`.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::common::nullable::deserialize"
    )]
    #[cfg_attr(feature = "builder", builder(default))]
    pub payload_open: Option<Nullable<String>>,
    /// Payload sent to stop the cover; [`Nullable::Null`] disables the command.
    /// Default: `STOP`.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::common::nullable::deserialize"
    )]
    #[cfg_attr(feature = "builder", builder(default))]
    pub payload_stop: Option<Nullable<String>>,
    /// Payload sent to stop tilting; [`Nullable::Null`] disables the
    /// command. Default: `stop` per the docs (`STOP` in Home Assistant's
    /// source).
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::common::nullable::deserialize"
    )]
    #[cfg_attr(feature = "builder", builder(default))]
    pub payload_stop_tilt: Option<Nullable<String>>,
    /// Position that means fully closed. Default: `0`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub position_closed: Option<i32>,
    /// Position that means fully open. Default: `100`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub position_open: Option<i32>,
    /// Template to extract the position from messages on `position_topic`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub position_template: Option<String>,
    /// Topic to receive the position from.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub position_topic: Option<String>,
    /// QoS level for subscribing and publishing. Default: `0`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub qos: Option<Qos>,
    /// Publish commands with the retain flag. Default: `false`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub retain: Option<bool>,
    /// Template to generate the payload sent to `set_position_topic`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub set_position_template: Option<String>,
    /// Topic to publish position commands to. Requires `position_topic`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub set_position_topic: Option<String>,
    /// Payload on `state_topic` that means closed. Default: `closed`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub state_closed: Option<String>,
    /// Payload on `state_topic` that means closing. Default: `closing`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub state_closing: Option<String>,
    /// Payload on `state_topic` that means open. Default: `open`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub state_open: Option<String>,
    /// Payload on `state_topic` that means opening. Default: `opening`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub state_opening: Option<String>,
    /// Payload on `state_topic` that means stopped. Default: `stopped`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub state_stopped: Option<String>,
    /// Topic to receive the state from.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub state_topic: Option<String>,
    /// Tilt value sent to close the tilt. Default: `0`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub tilt_closed_value: Option<i32>,
    /// Template to generate the payload sent to `tilt_command_topic`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub tilt_command_template: Option<String>,
    /// Topic to publish tilt commands to.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub tilt_command_topic: Option<String>,
    /// Maximum tilt value. Default: `100`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub tilt_max: Option<i32>,
    /// Minimum tilt value. Default: `0`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub tilt_min: Option<i32>,
    /// Tilt value sent to open the tilt. Default: `100`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub tilt_opened_value: Option<i32>,
    /// Assume the tilt changes as commanded. Default: `true` without
    /// `tilt_status_topic`, otherwise `false`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub tilt_optimistic: Option<bool>,
    /// Template to extract the tilt from messages on `tilt_status_topic`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub tilt_status_template: Option<String>,
    /// Topic to receive the tilt from.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub tilt_status_topic: Option<String>,
    /// ID that uniquely identifies this cover. Required for device discovery
    /// and for `device`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub unique_id: Option<String>,
    /// Template to extract the state from messages on `state_topic`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub value_template: Option<String>,
    /// Whether the entity is visible on dashboards when first added. Default:
    /// `true`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub visible_by_default: Option<bool>,
}

#[cfg(feature = "builder")]
impl Cover {
    /// Returns a builder for [`Cover`].
    pub fn builder() -> CoverBuilder {
        CoverBuilder::default()
    }
}

impl Platform for Cover {
    const PLATFORM: &'static str = "cover";
}
// @generated end

string_enum! {
    /// Device class of a cover, which sets its icon and the text shown for its
    /// states.
    ///
    /// <https://www.home-assistant.io/integrations/cover/#device-class>
    ///
    /// Source: `homeassistant/components/cover/const.py`
    pub enum CoverDeviceClass {
        // @generated begin device-class homeassistant/components/cover/const.py CoverDeviceClass
        /// Awning.
        Awning = "awning",
        /// Blind.
        Blind = "blind",
        /// Curtain.
        Curtain = "curtain",
        /// Damper.
        Damper = "damper",
        /// Door.
        Door = "door",
        /// Garage.
        Garage = "garage",
        /// Gate.
        Gate = "gate",
        /// Shade.
        Shade = "shade",
        /// Shutter.
        Shutter = "shutter",
        /// Window.
        Window = "window",
        // @generated end
    }
}
