//! MQTT light, JSON schema: <https://www.home-assistant.io/integrations/light.mqtt/#json-schema>

use super::ColorMode;
use crate::string_enum::string_enum;

// @generated begin component JsonLight
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

/// An MQTT light with the `json` schema: commands and states are JSON
/// objects.
///
/// <https://www.home-assistant.io/integrations/light.mqtt/#json-schema>
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "builder", derive(derive_builder::Builder))]
#[cfg_attr(feature = "builder", builder(setter(into, strip_option)))]
pub struct JsonLight {
    /// Topics that report the availability of the entity.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub availability: Vec<Availability>,
    /// How the `availability` topics are combined. Default: `latest`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub availability_mode: Option<AvailabilityMode>,
    /// Support brightness when the `rgb`, `rgbw` or `rgbww` color mode is
    /// supported. Default: `false`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub brightness: Option<bool>,
    /// Brightness value that means 100 %; at least `1`. Default: `255`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub brightness_scale: Option<u32>,
    /// Send and receive color temperatures in Kelvin instead of mireds.
    /// Default: `false`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub color_temp_kelvin: Option<bool>,
    /// Topic to publish JSON commands to. Required.
    pub command_topic: String,
    /// Entity ID to use instead of one generated from the name, e.g.
    /// `light.living_room_strip`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub default_entity_id: Option<String>,
    /// The device this JSON light belongs to. Only works when `unique_id` is set.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub device: Option<Device>,
    /// Whether the light supports effects. Default: `false`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub effect: Option<bool>,
    /// Supported effects. Home Assistant also accepts a single string.
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::common::one_or_many::deserialize"
    )]
    #[cfg_attr(feature = "builder", builder(default))]
    pub effect_list: Vec<String>,
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
    /// Whether the light supports flashing. Default: `true`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub flash: Option<bool>,
    /// Duration of a long flash in seconds. Default: `10`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub flash_time_long: Option<u32>,
    /// Duration of a short flash in seconds. Default: `2`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub flash_time_short: Option<u32>,
    /// Unique IDs of member entities, making this a group JSON light. Home
    /// Assistant also accepts a single string.
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::common::one_or_many::deserialize"
    )]
    #[cfg_attr(feature = "builder", builder(default))]
    pub group: Vec<String>,
    /// Icon for the entity, e.g. `mdi:led-strip-variant`.
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
    /// Maximum color temperature in Kelvin. Default: `6535`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub max_kelvin: Option<u32>,
    /// Maximum color temperature in mireds.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub max_mireds: Option<u32>,
    /// Message expiry interval for command payloads.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub message_expiry_interval: Option<MessageExpiryInterval>,
    /// Minimum color temperature in Kelvin. Default: `2000`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub min_kelvin: Option<u32>,
    /// Minimum color temperature in mireds.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub min_mireds: Option<u32>,
    /// Name of the JSON light; [`Nullable::Null`] if only the device name is
    /// relevant. Default: `MQTT JSON Light`.
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
    /// Not in the JSON light docs; accepted by Home Assistant.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub origin: Option<Origin>,
    /// QoS level for subscribing and publishing. Default: `0`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub qos: Option<Qos>,
    /// Publish commands with the retain flag. Default: `false`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub retain: Option<bool>,
    /// Schema of the light config; always `json`. Required.
    #[cfg_attr(feature = "builder", builder(default))]
    pub schema: JsonLightSchema,
    /// Topic to receive JSON state updates from.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub state_topic: Option<String>,
    /// Color modes the light supports. `onoff` or `brightness` must be the
    /// only value if used. Home Assistant also accepts a single string.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::common::one_or_many::deserialize_option"
    )]
    #[cfg_attr(feature = "builder", builder(default))]
    pub supported_color_modes: Option<Vec<ColorMode>>,
    /// Whether the light supports transitions. Default: `true`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub transition: Option<bool>,
    /// ID that uniquely identifies this JSON light. Required for device discovery
    /// and for `device`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub unique_id: Option<String>,
    /// Whether the entity is visible on dashboards when first added. Default:
    /// `true`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub visible_by_default: Option<bool>,
    /// White value that means 100 %; at least `1`. Default: `255`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub white_scale: Option<u32>,
}

#[cfg(feature = "builder")]
impl JsonLight {
    /// Returns a builder for [`JsonLight`].
    pub fn builder() -> JsonLightBuilder {
        JsonLightBuilder::default()
    }
}

impl Platform for JsonLight {
    const PLATFORM: &'static str = "light";
    const SCHEMA: Option<&'static str> = Some("json");
}
// @generated end

string_enum! {
    @closed
    /// The `schema` of a [`JsonLight`]: always `json`.
    ///
    /// <https://www.home-assistant.io/integrations/light.mqtt/#json-schema>
    #[derive(Default)]
    pub enum JsonLightSchema {
        /// The JSON schema.
        #[default]
        Json = "json",
    }
}
