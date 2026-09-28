//! MQTT device tracker: <https://www.home-assistant.io/integrations/device_tracker.mqtt/>

use crate::string_enum::string_enum;

// @generated begin component device_tracker
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

/// An MQTT device tracker: a location received on `state_topic`, or GPS
/// coordinates received as attributes on `json_attributes_topic`.
///
/// <https://www.home-assistant.io/integrations/device_tracker.mqtt/>
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "builder", derive(derive_builder::Builder))]
#[cfg_attr(
    feature = "builder",
    builder(setter(into, strip_option), build_fn(validate = "Self::validate"))
)]
pub struct DeviceTracker {
    /// Topics that report the availability of the entity.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub availability: Vec<Availability>,
    /// How the `availability` topics are combined. Default: `latest`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub availability_mode: Option<AvailabilityMode>,
    /// Entity ID to use instead of one generated from the name, e.g.
    /// `device_tracker.paulus_phone`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub default_entity_id: Option<String>,
    /// The device this device tracker belongs to. Only works when `unique_id` is set.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub device: Option<Device>,
    /// Whether the entity is enabled when first added. Default: `true`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub enabled_by_default: Option<bool>,
    /// Encoding of received payloads; an empty string disables decoding.
    /// Default: `utf-8`.
    /// Not in the device tracker docs; accepted by Home Assistant.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub encoding: Option<String>,
    /// Category of the entity.
    /// Not in the device tracker docs; accepted by Home Assistant.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub entity_category: Option<EntityCategory>,
    /// URL of a picture for the entity.
    /// Not in the device tracker docs; accepted by Home Assistant.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub entity_picture: Option<String>,
    /// Unique IDs of member entities, making this a group device tracker. Home
    /// Assistant also accepts a single string.
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::common::one_or_many::deserialize"
    )]
    #[cfg_attr(feature = "builder", builder(default))]
    pub group: Vec<String>,
    /// Icon for the entity, e.g. `mdi:cellphone`.
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
    /// Not in the device tracker docs; accepted by Home Assistant.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub message_expiry_interval: Option<MessageExpiryInterval>,
    /// Name of the device tracker; [`Nullable::Null`] if only the device name is
    /// relevant.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::common::nullable::deserialize"
    )]
    #[cfg_attr(feature = "builder", builder(default))]
    pub name: Option<Nullable<String>>,
    /// The application that sent the discovery message.
    /// Not in the device tracker docs; accepted by Home Assistant.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub origin: Option<Origin>,
    /// Payload that means the device is `home`. Default: `home`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub payload_home: Option<String>,
    /// Payload that means the device is `not_home`. Default: `not_home`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub payload_not_home: Option<String>,
    /// Payload on `state_topic` after which the location is derived from
    /// Home Assistant's zones again. Default: the string `None`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub payload_reset: Option<String>,
    /// Maximum QoS level for subscriptions. Default: `0`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub qos: Option<Qos>,
    /// Source of the location, which affects `person` entities. Home
    /// Assistant's source defaults to `gps`; the docs give no default.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub source_type: Option<SourceType>,
    /// Topic to receive the location (`home`, `not_home` or a zone name)
    /// from. Required unless `json_attributes_topic` is set; the builder
    /// checks this.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub state_topic: Option<String>,
    /// ID that uniquely identifies this device tracker. Required for device discovery
    /// and for `device`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub unique_id: Option<String>,
    /// Template to extract the location from the payload.
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
impl DeviceTracker {
    /// Returns a builder for [`DeviceTracker`].
    pub fn builder() -> DeviceTrackerBuilder {
        DeviceTrackerBuilder::default()
    }
}

impl Platform for DeviceTracker {
    const PLATFORM: &'static str = "device_tracker";
}
// @generated end

#[cfg(feature = "builder")]
impl DeviceTrackerBuilder {
    /// Home Assistant requires `state_topic` unless `json_attributes_topic` is
    /// set.
    fn validate(&self) -> Result<(), String> {
        let set = |field: &Option<Option<String>>| field.as_ref().is_some_and(Option::is_some);
        if !set(&self.state_topic) && !set(&self.json_attributes_topic) {
            return Err("one of `state_topic` and `json_attributes_topic` must be set".into());
        }
        Ok(())
    }
}

string_enum! {
    /// Source of a device tracker's location, which affects `person` entities.
    ///
    /// <https://www.home-assistant.io/integrations/device_tracker.mqtt/#source_type>
    ///
    /// Source: `homeassistant/components/device_tracker/const.py`
    pub enum SourceType {
        /// Location from GPS.
        Gps = "gps",
        /// Presence detected by a router.
        Router = "router",
        /// Presence detected over Bluetooth.
        Bluetooth = "bluetooth",
        /// Presence detected over Bluetooth Low Energy.
        BluetoothLe = "bluetooth_le",
    }
}
