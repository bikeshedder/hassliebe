//! MQTT alarm control panel: <https://www.home-assistant.io/integrations/alarm_control_panel.mqtt/>

use crate::string_enum::string_enum;

// @generated begin component alarm_control_panel
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

/// An MQTT alarm control panel: its state is received on `state_topic`,
/// and it is armed, disarmed and triggered by publishing to `command_topic`.
///
/// <https://www.home-assistant.io/integrations/alarm_control_panel.mqtt/>
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "builder", derive(derive_builder::Builder))]
#[cfg_attr(feature = "builder", builder(setter(into, strip_option)))]
pub struct AlarmControlPanel {
    /// Topics that report the availability of the entity.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub availability: Vec<Availability>,
    /// How the `availability` topics are combined. Default: `latest`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub availability_mode: Option<AvailabilityMode>,
    /// Code to arm or disarm the alarm in the frontend, validated locally.
    /// `REMOTE_CODE` or `REMOTE_CODE_TEXT` pass a numeric or text code on to
    /// `command_template` instead.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub code: Option<String>,
    /// Whether the code is required to arm the alarm. Default: `true`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub code_arm_required: Option<bool>,
    /// Whether the code is required to disarm the alarm. Default: `true`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub code_disarm_required: Option<bool>,
    /// Whether the code is required to trigger the alarm. Default: `true`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub code_trigger_required: Option<bool>,
    /// Template to generate the payload sent to `command_topic`, with the
    /// variables `action` and `code`. Default: `action` per the docs
    /// (`{{action}}` in Home Assistant's source).
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub command_template: Option<String>,
    /// Topic to publish commands to. Required.
    pub command_topic: String,
    /// Entity ID to use instead of one generated from the name, e.g.
    /// `alarm_control_panel.home_alarm`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub default_entity_id: Option<String>,
    /// The device this alarm control panel belongs to. Only works when `unique_id` is set.
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
    /// Unique IDs of member entities, making this a group alarm control panel. Home
    /// Assistant also accepts a single string.
    /// Not in the alarm control panel docs; accepted by Home Assistant.
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::common::one_or_many::deserialize"
    )]
    #[cfg_attr(feature = "builder", builder(default))]
    pub group: Vec<String>,
    /// Icon for the entity, e.g. `mdi:shield-home`.
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
    /// Name of the alarm control panel; [`Nullable::Null`] if only the device name is
    /// relevant. Default: `MQTT Alarm`.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::common::nullable::deserialize"
    )]
    #[cfg_attr(feature = "builder", builder(default))]
    pub name: Option<Nullable<String>>,
    /// The application that sent the discovery message.
    /// Not in the alarm control panel docs; accepted by Home Assistant.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub origin: Option<Origin>,
    /// Payload sent to arm in away mode. Default: `ARM_AWAY`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub payload_arm_away: Option<String>,
    /// Payload sent to arm with custom bypass. Default: `ARM_CUSTOM_BYPASS`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub payload_arm_custom_bypass: Option<String>,
    /// Payload sent to arm in home mode. Default: `ARM_HOME`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub payload_arm_home: Option<String>,
    /// Payload sent to arm in night mode. Default: `ARM_NIGHT`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub payload_arm_night: Option<String>,
    /// Payload sent to arm in vacation mode. Default: `ARM_VACATION`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub payload_arm_vacation: Option<String>,
    /// Payload sent to disarm. Default: `DISARM`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub payload_disarm: Option<String>,
    /// Payload sent to trigger the alarm. Default: `TRIGGER`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub payload_trigger: Option<String>,
    /// QoS level for subscribing and publishing. Default: `0`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub qos: Option<Qos>,
    /// Publish commands with the retain flag. Default: `false`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub retain: Option<bool>,
    /// Topic to receive the state from, e.g. `armed_away` or `disarmed`.
    /// Required.
    pub state_topic: String,
    /// Features the panel supports. Default: all of `arm_home`, `arm_away`,
    /// `arm_night`, `arm_vacation`, `arm_custom_bypass` and `trigger`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub supported_features: Option<Vec<AlarmControlPanelFeature>>,
    /// ID that uniquely identifies this alarm control panel. Required for device discovery
    /// and for `device`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub unique_id: Option<String>,
    /// Template to extract the state from the payload.
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
impl AlarmControlPanel {
    /// Returns a builder for [`AlarmControlPanel`].
    pub fn builder() -> AlarmControlPanelBuilder {
        AlarmControlPanelBuilder::default()
    }
}

impl Platform for AlarmControlPanel {
    const PLATFORM: &'static str = "alarm_control_panel";
}
// @generated end

string_enum! {
    /// A feature of an alarm control panel, for `supported_features`.
    ///
    /// <https://www.home-assistant.io/integrations/alarm_control_panel.mqtt/#supported_features>
    ///
    /// Source: `ALARM_CONTROL_PANEL_SUPPORTED_FEATURES` in
    /// `homeassistant/components/mqtt/const.py`
    pub enum AlarmControlPanelFeature {
        /// The panel can be armed in home mode.
        ArmHome = "arm_home",
        /// The panel can be armed in away mode.
        ArmAway = "arm_away",
        /// The panel can be armed in night mode.
        ArmNight = "arm_night",
        /// The panel can be armed in vacation mode.
        ArmVacation = "arm_vacation",
        /// The panel can be armed with a custom bypass.
        ArmCustomBypass = "arm_custom_bypass",
        /// The alarm can be triggered.
        Trigger = "trigger",
    }
}
