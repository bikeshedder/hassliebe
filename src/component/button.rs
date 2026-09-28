//! MQTT button: <https://www.home-assistant.io/integrations/button.mqtt/>

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

/// An MQTT button: pressing it publishes `payload_press` to
/// `command_topic`.
///
/// <https://www.home-assistant.io/integrations/button.mqtt/>
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "builder", derive(derive_builder::Builder))]
#[cfg_attr(feature = "builder", builder(setter(into, strip_option)))]
pub struct Button {
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
    /// Topic to publish button presses to. Required.
    pub command_topic: String,
    /// Entity ID to use instead of one generated from the name, e.g.
    /// `button.restart_bedroom_switch`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub default_entity_id: Option<String>,
    /// The device this button belongs to. Only works when `unique_id` is set.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub device: Option<Device>,
    /// Type of the button, which sets its icon.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub device_class: Option<ButtonDeviceClass>,
    /// Whether the entity is enabled when first added. Default: `true`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub enabled_by_default: Option<bool>,
    /// Encoding of published payloads. Default: `utf-8`.
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
    /// Unique IDs of member entities, making this a group button. Home
    /// Assistant also accepts a single string.
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::common::string_list::deserialize"
    )]
    #[cfg_attr(feature = "builder", builder(default))]
    pub group: Vec<String>,
    /// Icon for the entity, e.g. `mdi:restart`.
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
    /// Name of the button; [`Nullable::Null`] if only the device name is
    /// relevant. Default: `MQTT Button`.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::common::nullable::deserialize"
    )]
    #[cfg_attr(feature = "builder", builder(default))]
    pub name: Option<Nullable<String>>,
    /// The application that sent the discovery message. Not in the button
    /// docs; accepted by Home Assistant for all MQTT entities.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub origin: Option<Origin>,
    /// Payload sent when the button is pressed. Default: `PRESS`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub payload_press: Option<String>,
    /// QoS level for publishing. Default: `0`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub qos: Option<Qos>,
    /// Publish presses with the retain flag. Default: `false`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub retain: Option<bool>,
    /// ID that uniquely identifies this button. Required for device discovery
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
impl Button {
    /// Returns a builder for [`Button`].
    pub fn builder() -> ButtonBuilder {
        ButtonBuilder::default()
    }
}

impl Platform for Button {
    const PLATFORM: &'static str = "button";
}

string_enum! {
    /// Device class of a button, which sets its icon.
    ///
    /// <https://www.home-assistant.io/integrations/button/#device-class>
    ///
    /// Source: `homeassistant/components/button/const.py`
    pub enum ButtonDeviceClass {
        /// Identify.
        Identify = "identify",
        /// Restart.
        Restart = "restart",
        /// Update.
        Update = "update",
    }
}
