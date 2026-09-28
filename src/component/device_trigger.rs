//! MQTT device trigger: <https://www.home-assistant.io/integrations/device_trigger.mqtt/>

use crate::string_enum::string_enum;

// @generated begin component DeviceTrigger
use serde::{Deserialize, Serialize};

use crate::common::{device::Device, qos::Qos};
use crate::component::Platform;

/// An MQTT device trigger: fires a device automation trigger when a
/// message arrives on `topic`. Not an entity.
///
/// <https://www.home-assistant.io/integrations/device_trigger.mqtt/>
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "builder", derive(derive_builder::Builder))]
#[cfg_attr(feature = "builder", builder(setter(into, strip_option)))]
pub struct DeviceTrigger {
    /// Type of the automation; always `trigger`. Required.
    #[cfg_attr(feature = "builder", builder(default))]
    pub automation_type: DeviceAutomationType,
    /// The device the trigger belongs to; needs `identifiers` or
    /// `connections`. Required, but inherited from the device in device
    /// discovery; the builder requires it.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub device: Option<Device>,
    /// Encoding of received payloads. Not in the device trigger docs;
    /// accepted by Home Assistant, which ignores it and decodes `utf-8`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub encoding: Option<String>,
    /// Not in the device trigger docs; accepted by Home Assistant (shared
    /// base schema) but unused.
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::common::one_or_many::deserialize"
    )]
    #[cfg_attr(feature = "builder", builder(default))]
    pub group: Vec<String>,
    /// Payload that fires the trigger. Without it, every payload does.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub payload: Option<String>,
    /// Maximum QoS level for the subscription. Default: `0`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub qos: Option<Qos>,
    /// Subtype of the trigger, e.g. `button_1`. Required.
    pub subtype: String,
    /// Topic to receive trigger events from. Required.
    pub topic: String,
    /// Type of the trigger, e.g. `button_short_press`. Required.
    pub r#type: String,
    /// Template to extract the value that is compared to `payload`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub value_template: Option<String>,
}

#[cfg(feature = "builder")]
impl DeviceTrigger {
    /// Returns a builder for [`DeviceTrigger`].
    pub fn builder() -> DeviceTriggerBuilder {
        DeviceTriggerBuilder::default()
    }
}

impl Platform for DeviceTrigger {
    const PLATFORM: &'static str = "device_automation";
}
// @generated end

string_enum! {
    @closed
    /// The `automation_type` of a [`DeviceTrigger`]: always `trigger`.
    ///
    /// <https://www.home-assistant.io/integrations/device_trigger.mqtt/>
    ///
    /// Source: `homeassistant/components/mqtt/device_automation.py`
    #[derive(Default)]
    pub enum DeviceAutomationType {
        /// A device trigger.
        #[default]
        Trigger = "trigger",
    }
}
