//! MQTT light, default schema: <https://www.home-assistant.io/integrations/light.mqtt/>

mod json;

pub use json::{JsonLight, JsonLightSchema};

use crate::string_enum::string_enum;

// @generated begin component light
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

/// An MQTT light with the default (`basic`) schema.
///
/// <https://www.home-assistant.io/integrations/light.mqtt/#default-schema>
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "builder", derive(derive_builder::Builder))]
#[cfg_attr(feature = "builder", builder(setter(into, strip_option)))]
pub struct Light {
    /// Topics that report the availability of the entity.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub availability: Vec<Availability>,
    /// How the `availability` topics are combined. Default: `latest`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub availability_mode: Option<AvailabilityMode>,
    /// Template to generate the payload sent to `brightness_command_topic`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub brightness_command_template: Option<String>,
    /// Topic to publish brightness commands to.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub brightness_command_topic: Option<String>,
    /// Brightness value that means 100 %; at least `1`. Default: `255`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub brightness_scale: Option<u32>,
    /// Topic to receive brightness updates from.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub brightness_state_topic: Option<String>,
    /// Template to extract the brightness value from `brightness_state_topic`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub brightness_value_template: Option<String>,
    /// Topic to receive color mode updates from.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub color_mode_state_topic: Option<String>,
    /// Template to extract the color mode from `color_mode_state_topic`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub color_mode_value_template: Option<String>,
    /// Template to generate the payload sent to `color_temp_command_topic`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub color_temp_command_template: Option<String>,
    /// Topic to publish color temperature commands to.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub color_temp_command_topic: Option<String>,
    /// Use Kelvin instead of mireds for color temperatures. Default: `false`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub color_temp_kelvin: Option<bool>,
    /// Topic to receive color temperature updates from.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub color_temp_state_topic: Option<String>,
    /// Template to extract the color temperature value from `color_temp_state_topic`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub color_temp_value_template: Option<String>,
    /// Topic to publish on/off commands to. Required.
    pub command_topic: String,
    /// Entity ID to use instead of one generated from the name, e.g.
    /// `light.office`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub default_entity_id: Option<String>,
    /// The device this light belongs to. Only works when `unique_id` is set.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub device: Option<Device>,
    /// Template to generate the payload sent to `effect_command_topic`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub effect_command_template: Option<String>,
    /// Topic to publish effect commands to.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub effect_command_topic: Option<String>,
    /// Supported effects. Home Assistant also accepts a single string.
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::common::one_or_many::deserialize"
    )]
    #[cfg_attr(feature = "builder", builder(default))]
    pub effect_list: Vec<String>,
    /// Topic to receive effect updates from.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub effect_state_topic: Option<String>,
    /// Template to extract the effect value from `effect_state_topic`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub effect_value_template: Option<String>,
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
    /// Unique IDs of member entities, making this a group light. Home
    /// Assistant also accepts a single string.
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::common::one_or_many::deserialize"
    )]
    #[cfg_attr(feature = "builder", builder(default))]
    pub group: Vec<String>,
    /// Template to generate the payload sent to `hs_command_topic`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub hs_command_template: Option<String>,
    /// Topic to publish hue and saturation commands to.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub hs_command_topic: Option<String>,
    /// Topic to receive hue and saturation updates from.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub hs_state_topic: Option<String>,
    /// Template to extract the hue and saturation value from `hs_state_topic`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub hs_value_template: Option<String>,
    /// Icon for the entity, e.g. `mdi:ceiling-light`.
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
    /// Name of the light; [`Nullable::Null`] if only the device name is
    /// relevant. Default: `MQTT Light`.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::common::nullable::deserialize"
    )]
    #[cfg_attr(feature = "builder", builder(default))]
    pub name: Option<Nullable<String>>,
    /// When to send `payload_on` along with other commands. Default: `last`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub on_command_type: Option<OnCommandType>,
    /// Assume the state changes as commanded. Default: `true` without
    /// `state_topic`, otherwise `false`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub optimistic: Option<bool>,
    /// The application that sent the discovery message.
    /// Not in the light docs; accepted by Home Assistant.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub origin: Option<Origin>,
    /// Payload sent to turn the light off. Default: `OFF`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub payload_off: Option<String>,
    /// Payload sent to turn the light on. Default: `ON`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub payload_on: Option<String>,
    /// QoS level for subscribing and publishing. Default: `0`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub qos: Option<Qos>,
    /// Publish commands with the retain flag. Default: `false`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub retain: Option<bool>,
    /// Template to generate the payload sent to `rgb_command_topic`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub rgb_command_template: Option<String>,
    /// Topic to publish RGB color commands to.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub rgb_command_topic: Option<String>,
    /// Topic to receive RGB color updates from.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub rgb_state_topic: Option<String>,
    /// Template to extract the RGB color value from `rgb_state_topic`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub rgb_value_template: Option<String>,
    /// Template to generate the payload sent to `rgbw_command_topic`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub rgbw_command_template: Option<String>,
    /// Topic to publish RGBW color commands to.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub rgbw_command_topic: Option<String>,
    /// Topic to receive RGBW color updates from.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub rgbw_state_topic: Option<String>,
    /// Template to extract the RGBW color value from `rgbw_state_topic`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub rgbw_value_template: Option<String>,
    /// Template to generate the payload sent to `rgbww_command_topic`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub rgbww_command_template: Option<String>,
    /// Topic to publish RGBWW color commands to.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub rgbww_command_topic: Option<String>,
    /// Topic to receive RGBWW color updates from.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub rgbww_state_topic: Option<String>,
    /// Template to extract the RGBWW color value from `rgbww_state_topic`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub rgbww_value_template: Option<String>,
    /// Schema of the light config. Only `basic` is modeled. Default: `basic`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub schema: Option<LightSchema>,
    /// Topic to receive on/off state updates from.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub state_topic: Option<String>,
    /// Template to extract the on/off state from `state_topic`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub state_value_template: Option<String>,
    /// ID that uniquely identifies this light. Required for device discovery
    /// and for `device`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub unique_id: Option<String>,
    /// Whether the entity is visible on dashboards when first added. Default:
    /// `true`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub visible_by_default: Option<bool>,
    /// Topic to publish commands to switch to white mode with a brightness.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub white_command_topic: Option<String>,
    /// White value that means 100 %; at least `1`. Default: `255`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub white_scale: Option<u32>,
    /// Template to generate the payload sent to `xy_command_topic`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub xy_command_template: Option<String>,
    /// Topic to publish XY color commands to.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub xy_command_topic: Option<String>,
    /// Topic to receive XY color updates from.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub xy_state_topic: Option<String>,
    /// Template to extract the XY color value from `xy_state_topic`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub xy_value_template: Option<String>,
}

#[cfg(feature = "builder")]
impl Light {
    /// Returns a builder for [`Light`].
    pub fn builder() -> LightBuilder {
        LightBuilder::default()
    }
}

impl Platform for Light {
    const PLATFORM: &'static str = "light";
    const SCHEMA: Option<&'static str> = Some("basic");
}
// @generated end

string_enum! {
    @closed
    /// Schema of a light config. Only the default schema is modeled, so
    /// deserializing a [`Light`] with `schema` `json` or `template` fails.
    ///
    /// <https://www.home-assistant.io/integrations/light.mqtt/>
    ///
    /// Source: `homeassistant/components/mqtt/light/schema.py`
    pub enum LightSchema {
        /// The default schema.
        Basic = "basic",
    }
}

string_enum! {
    /// When a light sends `payload_on` together with other commands.
    ///
    /// <https://www.home-assistant.io/integrations/light.mqtt/#on_command_type>
    ///
    /// Source: `homeassistant/components/mqtt/const.py`
    pub enum OnCommandType {
        /// Send `payload_on` first, then the brightness, color, … commands.
        First = "first",
        /// Send the brightness, color, … commands first, then `payload_on`.
        Last = "last",
        /// Send only the brightness command instead of `payload_on`.
        Brightness = "brightness",
    }
}

string_enum! {
    /// A color mode of a light, for `supported_color_modes`.
    ///
    /// <https://www.home-assistant.io/integrations/light.mqtt/#supported_color_modes>
    ///
    /// Source: `VALID_COLOR_MODES` in `homeassistant/components/light/const.py`
    pub enum ColorMode {
        /// On and off only; must be the only supported mode.
        Onoff = "onoff",
        /// Dimmable without color; must be the only supported mode.
        Brightness = "brightness",
        /// Color temperature.
        ColorTemp = "color_temp",
        /// Hue and saturation.
        Hs = "hs",
        /// CIE xy color.
        Xy = "xy",
        /// RGB color.
        Rgb = "rgb",
        /// RGB color with a white channel.
        Rgbw = "rgbw",
        /// RGB color with cold and warm white channels.
        Rgbww = "rgbww",
        /// White mode; not allowed as the only supported mode.
        White = "white",
    }
}
