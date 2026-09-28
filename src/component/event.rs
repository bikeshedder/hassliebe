//! MQTT event: <https://www.home-assistant.io/integrations/event.mqtt/>

// @generated begin component event
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

/// An MQTT event: a stateless entity that fires events received as JSON on
/// `state_topic`.
///
/// <https://www.home-assistant.io/integrations/event.mqtt/>
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "builder", derive(derive_builder::Builder))]
#[cfg_attr(feature = "builder", builder(setter(into, strip_option)))]
pub struct Event {
    /// Topics that report the availability of the entity.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub availability: Vec<Availability>,
    /// How the `availability` topics are combined. Default: `latest`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub availability_mode: Option<AvailabilityMode>,
    /// Entity ID to use instead of one generated from the name, e.g.
    /// `event.doorbell`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub default_entity_id: Option<String>,
    /// The device this event belongs to. Only works when `unique_id` is set.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub device: Option<Device>,
    /// Type of the event, which sets its icon.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub device_class: Option<EventDeviceClass>,
    /// Whether the entity is enabled when first added. Default: `true`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub enabled_by_default: Option<bool>,
    /// Encoding of received payloads; an empty string disables decoding.
    /// Default: `utf-8`.
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
    /// The `event_type` values the entity accepts. Required. Home Assistant
    /// also accepts a single string.
    #[serde(deserialize_with = "crate::common::one_or_many::deserialize")]
    pub event_types: Vec<String>,
    /// Unique IDs of member entities, making this a group event. Home
    /// Assistant also accepts a single string.
    /// Not in the event docs; accepted by Home Assistant.
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::common::one_or_many::deserialize"
    )]
    #[cfg_attr(feature = "builder", builder(default))]
    pub group: Vec<String>,
    /// Icon for the entity, e.g. `mdi:doorbell`.
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
    /// Not in the event docs; accepted by Home Assistant.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub message_expiry_interval: Option<MessageExpiryInterval>,
    /// Name of the event; [`Nullable::Null`] if only the device name is
    /// relevant. Default: `MQTT Event`.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::common::nullable::deserialize"
    )]
    #[cfg_attr(feature = "builder", builder(default))]
    pub name: Option<Nullable<String>>,
    /// The application that sent the discovery message.
    /// Not in the event docs; accepted by Home Assistant.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub origin: Option<Origin>,
    /// Maximum QoS level for subscriptions. Default: `0`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub qos: Option<Qos>,
    /// Topic to receive JSON event payloads with an `event_type` from;
    /// retained messages are ignored. Required.
    pub state_topic: String,
    /// ID that uniquely identifies this event. Required for device discovery
    /// and for `device`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub unique_id: Option<String>,
    /// Template that renders the payload into a JSON event payload.
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
impl Event {
    /// Returns a builder for [`Event`].
    pub fn builder() -> EventBuilder {
        EventBuilder::default()
    }
}

impl Platform for Event {
    const PLATFORM: &'static str = "event";
}
// @generated end

string_enum! {
    /// Device class of an event entity, which sets its icon.
    ///
    /// <https://www.home-assistant.io/integrations/event/#device-class>
    ///
    /// Source: `homeassistant/components/event/const.py`
    pub enum EventDeviceClass {
        // @generated begin device-class homeassistant/components/event/const.py EventDeviceClass
        /// Doorbell.
        Doorbell = "doorbell",
        /// Button.
        Button = "button",
        /// Motion.
        Motion = "motion",
        // @generated end
    }
}
