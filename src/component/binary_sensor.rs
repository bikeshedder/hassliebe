//! MQTT binary sensor: <https://www.home-assistant.io/integrations/binary_sensor.mqtt/>

use crate::string_enum::string_enum;

// @generated begin component binary_sensor
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

/// An MQTT binary sensor: a read-only entity that is `on` or `off`,
/// received on `state_topic`.
///
/// <https://www.home-assistant.io/integrations/binary_sensor.mqtt/>
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "builder", derive(derive_builder::Builder))]
#[cfg_attr(feature = "builder", builder(setter(into, strip_option)))]
pub struct BinarySensor {
    /// Topics that report the availability of the entity.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub availability: Vec<Availability>,
    /// How the `availability` topics are combined. Default: `latest`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub availability_mode: Option<AvailabilityMode>,
    /// Entity ID to use instead of one generated from the name, e.g.
    /// `binary_sensor.front_door`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub default_entity_id: Option<String>,
    /// The device this binary sensor belongs to. Only works when `unique_id` is set.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub device: Option<Device>,
    /// Type of the binary sensor, which sets its icon and how its state is
    /// displayed.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub device_class: Option<BinarySensorDeviceClass>,
    /// Whether the entity is enabled when first added. Default: `true`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub enabled_by_default: Option<bool>,
    /// Encoding of received payloads; an empty string disables decoding.
    /// Default: `utf-8`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub encoding: Option<String>,
    /// Category of the entity. The docs only allow `diagnostic` for sensors.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub entity_category: Option<EntityCategory>,
    /// URL of a picture for the entity.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub entity_picture: Option<String>,
    /// Seconds after the last update until the state becomes unavailable.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub expire_after: Option<u32>,
    /// Send update events even if the value has not changed. Default: `false`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub force_update: Option<bool>,
    /// Unique IDs of member entities, making this a group binary sensor. Home
    /// Assistant also accepts a single string.
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::common::one_or_many::deserialize"
    )]
    #[cfg_attr(feature = "builder", builder(default))]
    pub group: Vec<String>,
    /// Icon for the entity, e.g. `mdi:door`.
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
    /// Not in the binary sensor docs; accepted by Home Assistant.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub message_expiry_interval: Option<MessageExpiryInterval>,
    /// Name of the binary sensor; [`Nullable::Null`] if only the device name is
    /// relevant. Default: `MQTT binary sensor`.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::common::nullable::deserialize"
    )]
    #[cfg_attr(feature = "builder", builder(default))]
    pub name: Option<Nullable<String>>,
    /// Seconds after which the sensor resets to `off` on its own.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub off_delay: Option<u32>,
    /// The application that sent the discovery message.
    /// Not in the binary sensor docs; accepted by Home Assistant.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub origin: Option<Origin>,
    /// Payload that means `off`. Default: `OFF`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub payload_off: Option<String>,
    /// Payload that means `on`. Default: `ON`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub payload_on: Option<String>,
    /// Maximum QoS level for subscriptions. Default: `0`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub qos: Option<Qos>,
    /// Topic to receive sensor states from. Required.
    pub state_topic: String,
    /// ID that uniquely identifies this binary sensor. Required for device discovery
    /// and for `device`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub unique_id: Option<String>,
    /// Template to extract a value from the payload, which is compared to
    /// `payload_on` and `payload_off`.
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
impl BinarySensor {
    /// Returns a builder for [`BinarySensor`].
    pub fn builder() -> BinarySensorBuilder {
        BinarySensorBuilder::default()
    }
}

impl Platform for BinarySensor {
    const PLATFORM: &'static str = "binary_sensor";
}
// @generated end

string_enum! {
    /// Device class of a binary sensor, which sets its icon and the text shown
    /// for its states.
    ///
    /// <https://www.home-assistant.io/integrations/binary_sensor/#device-class>
    ///
    /// Source: `homeassistant/components/binary_sensor/const.py`
    pub enum BinarySensorDeviceClass {
        // @generated begin device-class homeassistant/components/binary_sensor/const.py BinarySensorDeviceClass
        /// Battery.
        ///
        /// On means low, Off means normal.
        Battery = "battery",
        /// Battery charging.
        ///
        /// On means charging, Off means not charging.
        BatteryCharging = "battery_charging",
        /// Carbon monoxide.
        ///
        /// On means carbon monoxide detected, Off means no carbon monoxide (clear).
        Co = "carbon_monoxide",
        /// Cold.
        ///
        /// On means cold, Off means normal.
        Cold = "cold",
        /// Connectivity.
        ///
        /// On means connected, Off means disconnected.
        Connectivity = "connectivity",
        /// Door.
        ///
        /// On means open, Off means closed.
        Door = "door",
        /// Garage door.
        ///
        /// On means open, Off means closed.
        GarageDoor = "garage_door",
        /// Gas.
        ///
        /// On means gas detected, Off means no gas (clear).
        Gas = "gas",
        /// Glass break.
        ///
        /// On means glass break detected, Off means no glass break (clear).
        GlassBreak = "glass_break",
        /// Heat.
        ///
        /// On means hot, Off means normal.
        Heat = "heat",
        /// Light.
        ///
        /// On means light detected, Off means no light.
        Light = "light",
        /// Lock.
        ///
        /// On means open (unlocked), Off means closed (locked).
        Lock = "lock",
        /// Moisture.
        ///
        /// On means wet, Off means dry.
        Moisture = "moisture",
        /// Motion.
        ///
        /// On means motion detected, Off means no motion (clear).
        Motion = "motion",
        /// Moving.
        ///
        /// On means moving, Off means not moving (stopped).
        Moving = "moving",
        /// Occupancy.
        ///
        /// On means occupied, Off means not occupied (clear).
        Occupancy = "occupancy",
        /// Opening.
        ///
        /// On means open, Off means closed.
        Opening = "opening",
        /// Plug.
        ///
        /// On means plugged in, Off means unplugged.
        Plug = "plug",
        /// Power.
        ///
        /// On means power detected, Off means no power.
        Power = "power",
        /// Presence.
        ///
        /// On means home, Off means away.
        Presence = "presence",
        /// Problem.
        ///
        /// On means problem detected, Off means no problem (OK).
        Problem = "problem",
        /// Running.
        ///
        /// On means running, Off means not running.
        Running = "running",
        /// Safety.
        ///
        /// On means unsafe, Off means safe.
        Safety = "safety",
        /// Smoke.
        ///
        /// On means smoke detected, Off means no smoke (clear).
        Smoke = "smoke",
        /// Sound.
        ///
        /// On means sound detected, Off means no sound (clear).
        Sound = "sound",
        /// Tamper.
        ///
        /// On means tampering detected, Off means no tampering (clear).
        Tamper = "tamper",
        /// Update.
        ///
        /// On means update available, Off means up-to-date.
        Update = "update",
        /// Vibration.
        ///
        /// On means vibration detected, Off means no vibration.
        Vibration = "vibration",
        /// Window.
        ///
        /// On means open, Off means closed.
        Window = "window",
        // @generated end
    }
}
