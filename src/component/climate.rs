//! MQTT climate (HVAC): <https://www.home-assistant.io/integrations/climate.mqtt/>

use crate::string_enum::string_enum;

// @generated begin component climate
use serde::{Deserialize, Serialize};

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
use crate::component::Platform;

/// An MQTT climate (HVAC) device, such as a thermostat or an air
/// conditioner.
///
/// <https://www.home-assistant.io/integrations/climate.mqtt/>
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "builder", derive(derive_builder::Builder))]
#[cfg_attr(
    feature = "builder",
    builder(setter(into, strip_option), build_fn(validate = "Self::validate"))
)]
pub struct Climate {
    /// Template to extract the current action from `action_topic`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub action_template: Option<String>,
    /// Topic to receive the current action (`HvacAction` values such as
    /// `heating`) from. A `None` payload resets it.
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
    /// Template to extract the current humidity from
    /// `current_humidity_topic`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub current_humidity_template: Option<String>,
    /// Topic to receive the current humidity from. A `None` payload resets it.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub current_humidity_topic: Option<String>,
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
    /// `climate.study`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub default_entity_id: Option<String>,
    /// The device this HVAC device belongs to. Only works when `unique_id` is set.
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
    /// Template to generate the payload sent to `fan_mode_command_topic`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub fan_mode_command_template: Option<String>,
    /// Topic to publish fan mode commands to.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub fan_mode_command_topic: Option<String>,
    /// Template to extract the fan mode from `fan_mode_state_topic`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub fan_mode_state_template: Option<String>,
    /// Topic to receive the fan mode from. A `None` payload resets it.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub fan_mode_state_topic: Option<String>,
    /// Supported fan modes. Default: `auto`, `low`, `medium`, `high`.
    /// Home Assistant also accepts a single string.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::common::one_or_many::deserialize_option"
    )]
    #[cfg_attr(feature = "builder", builder(default))]
    pub fan_modes: Option<Vec<String>>,
    /// Unique IDs of member entities, making this a group HVAC device. Home
    /// Assistant also accepts a single string.
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::common::one_or_many::deserialize"
    )]
    #[cfg_attr(feature = "builder", builder(default))]
    pub group: Vec<String>,
    /// Icon for the entity, e.g. `mdi:thermostat`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub icon: Option<String>,
    /// Initial target temperature. Default: 21 °C or 69.8 °F, depending on
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
    /// Maximum target humidity in percent. Default: `99`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub max_humidity: Option<f64>,
    /// Maximum target temperature. Default: 35 °C or 95 °F, depending on the
    /// temperature unit.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub max_temp: Option<f64>,
    /// Message expiry interval for command payloads.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub message_expiry_interval: Option<MessageExpiryInterval>,
    /// Minimum target humidity in percent. Default: `30`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub min_humidity: Option<f64>,
    /// Minimum target temperature. Default: 7 °C or 44.6 °F, depending on the
    /// temperature unit.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub min_temp: Option<f64>,
    /// Template to generate the payload sent to `mode_command_topic`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub mode_command_template: Option<String>,
    /// Topic to publish HVAC mode commands to.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub mode_command_topic: Option<String>,
    /// Template to extract the HVAC mode from `mode_state_topic`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub mode_state_template: Option<String>,
    /// Topic to receive the HVAC mode from. A `None` payload resets it.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub mode_state_topic: Option<String>,
    /// Supported HVAC modes, a subset of the default. Default: `auto`, `off`,
    /// `cool`, `heat`, `dry`, `fan_only`.
    /// Home Assistant also accepts a single string.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::common::one_or_many::deserialize_option"
    )]
    #[cfg_attr(feature = "builder", builder(default))]
    pub modes: Option<Vec<HvacMode>>,
    /// Name of the HVAC device; [`Nullable::Null`] if only the device name is
    /// relevant. Default: `MQTT HVAC`.
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
    /// Not in the HVAC device docs; accepted by Home Assistant.
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
    /// Template to generate the payload sent to `preset_mode_command_topic`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub preset_mode_command_template: Option<String>,
    /// Topic to publish preset mode commands to.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub preset_mode_command_topic: Option<String>,
    /// Topic to receive the preset mode from. `none` or `None` resets it.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub preset_mode_state_topic: Option<String>,
    /// Template to extract the preset mode from `preset_mode_state_topic`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub preset_mode_value_template: Option<String>,
    /// Supported preset modes, e.g. `eco`, `away` or `boost`; must not
    /// contain `none`. Home Assistant also accepts a single string.
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
    /// Publish commands with the retain flag. Default: `false`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub retain: Option<bool>,
    /// Template to generate the payload sent to
    /// `swing_horizontal_mode_command_topic`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub swing_horizontal_mode_command_template: Option<String>,
    /// Topic to publish horizontal swing mode commands to.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub swing_horizontal_mode_command_topic: Option<String>,
    /// Template to extract the horizontal swing mode from
    /// `swing_horizontal_mode_state_topic`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub swing_horizontal_mode_state_template: Option<String>,
    /// Topic to receive the horizontal swing mode from.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub swing_horizontal_mode_state_topic: Option<String>,
    /// Supported horizontal swing modes. Default: `on`, `off`.
    /// Home Assistant also accepts a single string.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::common::one_or_many::deserialize_option"
    )]
    #[cfg_attr(feature = "builder", builder(default))]
    pub swing_horizontal_modes: Option<Vec<String>>,
    /// Template to generate the payload sent to `swing_mode_command_topic`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub swing_mode_command_template: Option<String>,
    /// Topic to publish swing mode commands to.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub swing_mode_command_topic: Option<String>,
    /// Template to extract the swing mode from `swing_mode_state_topic`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub swing_mode_state_template: Option<String>,
    /// Topic to receive the swing mode from.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub swing_mode_state_topic: Option<String>,
    /// Supported swing modes. Default: `on`, `off`. Home Assistant also
    /// accepts a single string.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::common::one_or_many::deserialize_option"
    )]
    #[cfg_attr(feature = "builder", builder(default))]
    pub swing_modes: Option<Vec<String>>,
    /// Template to generate the payload sent to
    /// `target_humidity_command_topic`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub target_humidity_command_template: Option<String>,
    /// Topic to publish target humidity commands to.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub target_humidity_command_topic: Option<String>,
    /// Template to extract the target humidity from
    /// `target_humidity_state_topic`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub target_humidity_state_template: Option<String>,
    /// Topic to receive the target humidity from. A `None` payload resets it.
    /// Requires `target_humidity_command_topic`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub target_humidity_state_topic: Option<String>,
    /// Step size of the target temperature. Default: `1`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub temp_step: Option<f64>,
    /// Template to generate the payload sent to `temperature_command_topic`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub temperature_command_template: Option<String>,
    /// Topic to publish target temperature commands to.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub temperature_command_topic: Option<String>,
    /// Template to generate the payload sent to
    /// `temperature_high_command_topic`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub temperature_high_command_template: Option<String>,
    /// Topic to publish upper target temperature commands to.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub temperature_high_command_topic: Option<String>,
    /// Template to extract the upper target temperature from
    /// `temperature_high_state_topic`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub temperature_high_state_template: Option<String>,
    /// Topic to receive the upper target temperature from. A `None` payload
    /// resets it.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub temperature_high_state_topic: Option<String>,
    /// Template to generate the payload sent to
    /// `temperature_low_command_topic`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub temperature_low_command_template: Option<String>,
    /// Topic to publish lower target temperature commands to.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub temperature_low_command_topic: Option<String>,
    /// Template to extract the lower target temperature from
    /// `temperature_low_state_topic`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub temperature_low_state_template: Option<String>,
    /// Topic to receive the lower target temperature from. A `None` payload
    /// resets it.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub temperature_low_state_topic: Option<String>,
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
    /// ID that uniquely identifies this HVAC device. Required for device discovery
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
impl Climate {
    /// Returns a builder for [`Climate`].
    pub fn builder() -> ClimateBuilder {
        ClimateBuilder::default()
    }
}

impl Platform for Climate {
    const PLATFORM: &'static str = "climate";
}
// @generated end

#[cfg(feature = "builder")]
impl ClimateBuilder {
    /// Home Assistant requires `preset_mode_command_topic` and `preset_modes`
    /// together, and `target_humidity_command_topic` with
    /// `target_humidity_state_topic`.
    fn validate(&self) -> Result<(), String> {
        let set = |field: &Option<Option<String>>| field.as_ref().is_some_and(Option::is_some);
        let modes = self
            .preset_modes
            .as_ref()
            .is_some_and(|modes| !modes.is_empty());
        if set(&self.preset_mode_command_topic) != modes {
            return Err(
                "`preset_mode_command_topic` and `preset_modes` must be set together".into(),
            );
        }
        if set(&self.target_humidity_state_topic) && !set(&self.target_humidity_command_topic) {
            return Err(
                "`target_humidity_state_topic` requires `target_humidity_command_topic`".into(),
            );
        }
        Ok(())
    }
}

string_enum! {
    /// An HVAC mode of a climate device.
    ///
    /// <https://www.home-assistant.io/integrations/climate.mqtt/#modes>
    ///
    /// Source: `homeassistant/components/climate/const.py`
    pub enum HvacMode {
        // @generated begin device-class homeassistant/components/climate/const.py HVACMode
        /// Off.
        ///
        /// All activity disabled / Device is off/standby.
        Off = "off",
        /// Heat.
        ///
        /// Heating.
        Heat = "heat",
        /// Cool.
        ///
        /// Cooling.
        Cool = "cool",
        /// Heat cool.
        ///
        /// The device supports heating/cooling to a range.
        HeatCool = "heat_cool",
        /// Auto.
        ///
        /// The temperature is set based on a schedule, learned behavior, AI or some other related mechanism. User is not able to adjust the temperature.
        Auto = "auto",
        /// Dry.
        ///
        /// Device is in Dry/Humidity mode.
        Dry = "dry",
        /// Fan only.
        ///
        /// Only the fan is on, not fan and another mode like cool.
        FanOnly = "fan_only",
        // @generated end
    }
}
