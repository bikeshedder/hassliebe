//! MQTT water heater: <https://www.home-assistant.io/integrations/water_heater.mqtt/>

use crate::string_enum::string_enum;

// @generated begin component water_heater
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
    temperature_unit::TemperatureUnit,
};

/// An MQTT water heater, such as a boiler or a heat pump water heater.
///
/// <https://www.home-assistant.io/integrations/water_heater.mqtt/>
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "builder", derive(derive_builder::Builder))]
#[cfg_attr(feature = "builder", builder(setter(into, strip_option)))]
pub struct WaterHeater {
    /// Topics that report the availability of the entity.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub availability: Vec<Availability>,
    /// How the `availability` topics are combined. Default: `latest`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub availability_mode: Option<AvailabilityMode>,
    /// Template to extract the current temperature from
    /// `current_temperature_topic`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub current_temperature_template: Option<String>,
    /// Topic to receive the current temperature from. A `None` payload
    /// resets it.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub current_temperature_topic: Option<String>,
    /// Entity ID to use instead of one generated from the name, e.g.
    /// `water_heater.boiler`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub default_entity_id: Option<String>,
    /// The device this water heater belongs to. Only works when `unique_id` is set.
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
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub entity_category: Option<EntityCategory>,
    /// URL of a picture for the entity.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub entity_picture: Option<String>,
    /// Unique IDs of member entities, making this a group water heater. Home
    /// Assistant also accepts a single string.
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::common::one_or_many::deserialize"
    )]
    #[cfg_attr(feature = "builder", builder(default))]
    pub group: Vec<String>,
    /// Icon for the entity, e.g. `mdi:water-boiler`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub icon: Option<String>,
    /// Initial target temperature. Default: 43.3 °C or 110 °F, depending on
    /// the temperature unit.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub initial: Option<f64>,
    /// Template to extract a JSON dictionary of attributes from messages on
    /// `json_attributes_topic`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub json_attributes_template: Option<String>,
    /// Topic whose JSON dictionary payload is added as entity attributes.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub json_attributes_topic: Option<String>,
    /// Maximum target temperature. Default: 60 °C or 140 °F, depending on the
    /// temperature unit.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub max_temp: Option<f64>,
    /// Message expiry interval for command payloads.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub message_expiry_interval: Option<MessageExpiryInterval>,
    /// Minimum target temperature. Default: 43.3 °C or 110 °F, depending on
    /// the temperature unit.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub min_temp: Option<f64>,
    /// Template to generate the payload sent to `mode_command_topic`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub mode_command_template: Option<String>,
    /// Topic to publish operation mode commands to.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub mode_command_topic: Option<String>,
    /// Template to extract the operation mode from `mode_state_topic`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub mode_state_template: Option<String>,
    /// Topic to receive the operation mode from. A `None` payload resets it.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub mode_state_topic: Option<String>,
    /// Supported operation modes, a subset of the default. Default: `off`,
    /// `eco`, `electric`, `gas`, `heat_pump`, `high_demand`, `performance`.
    /// Home Assistant also accepts a single string.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::common::one_or_many::deserialize_option"
    )]
    #[cfg_attr(feature = "builder", builder(default))]
    pub modes: Option<Vec<WaterHeaterMode>>,
    /// Name of the water heater; [`Nullable::Null`] if only the device name is
    /// relevant.
    /// Default: `MQTT water heater` per the docs (`MQTT Water Heater` in Home Assistant's source).
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::common::nullable::deserialize"
    )]
    #[cfg_attr(feature = "builder", builder(default))]
    pub name: Option<Nullable<String>>,
    /// Assume every property changes as commanded, even with a state topic.
    /// Default: `true` without a state topic, otherwise `false`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub optimistic: Option<bool>,
    /// The application that sent the discovery message.
    /// Not in the water heater docs; accepted by Home Assistant.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub origin: Option<Origin>,
    /// Payload sent to `power_command_topic` to turn off. Default: `OFF`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub payload_off: Option<String>,
    /// Payload sent to `power_command_topic` to turn on. Default: `ON`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub payload_on: Option<String>,
    /// Template to generate the payload sent to `power_command_topic`; `value`
    /// is `payload_on` or `payload_off`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub power_command_template: Option<String>,
    /// Topic to publish power commands (`payload_on`, `payload_off`) to.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub power_command_topic: Option<String>,
    /// Precision of temperatures: `0.1`, `0.5` or `1.0`. Default: `0.1` for
    /// Celsius, `1.0` for Fahrenheit.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub precision: Option<f64>,
    /// QoS level for subscribing and publishing. Default: `0`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub qos: Option<Qos>,
    /// Publish commands with the retain flag. Default: `false`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub retain: Option<bool>,
    /// Template to generate the payload sent to `temperature_command_topic`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub temperature_command_template: Option<String>,
    /// Topic to publish target temperature commands to.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub temperature_command_topic: Option<String>,
    /// Template to extract the target temperature from
    /// `temperature_state_topic`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub temperature_state_template: Option<String>,
    /// Topic to receive the target temperature from. A `None` payload resets
    /// it.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub temperature_state_topic: Option<String>,
    /// Temperature unit of the device. Default: the system's temperature unit.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub temperature_unit: Option<TemperatureUnit>,
    /// ID that uniquely identifies this water heater. Required for device discovery
    /// and for `device`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub unique_id: Option<String>,
    /// Default template to extract values from all `*_state_topic`s.
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
impl WaterHeater {
    /// Returns a builder for [`WaterHeater`].
    pub fn builder() -> WaterHeaterBuilder {
        WaterHeaterBuilder::default()
    }
}

impl Platform for WaterHeater {
    const PLATFORM: &'static str = "water_heater";
}
// @generated end

string_enum! {
    /// An operation mode of a water heater.
    ///
    /// <https://www.home-assistant.io/integrations/water_heater.mqtt/#modes>
    ///
    /// Source: the `STATE_*` constants in
    /// `homeassistant/components/water_heater/const.py` and `STATE_OFF` in
    /// `homeassistant/const.py`
    pub enum WaterHeaterMode {
        /// The water heater is off.
        Off = "off",
        /// Energy-efficient mode with energy savings and fast heating.
        Eco = "eco",
        /// Electric-only mode, which uses the most energy.
        Electric = "electric",
        /// Gas-only mode, which uses the most energy.
        Gas = "gas",
        /// Heat pump mode: slowest to heat, but uses less energy.
        HeatPump = "heat_pump",
        /// Meets high demand when the water heater is undersized.
        HighDemand = "high_demand",
        /// High performance mode.
        Performance = "performance",
    }
}
