use std::num::NonZeroU32;

use super::Platform;
use super::device_class::DeviceClass;

use crate::common::{
    availability::{Availability, AvailabilityMode},
    device::Device,
    entity_category::EntityCategory,
    qos::Qos,
};
use serde::Serialize;

/// <https://www.home-assistant.io/integrations/binary_sensor.mqtt/>
#[derive(Clone, Debug, Default, Serialize)]
#[cfg_attr(feature = "builder", derive(derive_builder::Builder))]
#[cfg_attr(feature = "builder", builder(default, setter(into, strip_option)))]
pub struct BinarySensor {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availabilty: Option<Vec<Availability>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_mode: Option<AvailabilityMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_template: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_topic: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_entity_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device: Option<Device>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device_class: Option<DeviceClass>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled_by_default: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_category: Option<EntityCategory>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_picture: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expire_after: Option<NonZeroU32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub force_update: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub json_attributes_template: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub json_attributes_topic: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub off_delay: Option<NonZeroU32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload_available: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload_not_available: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload_off: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub payload_on: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qos: Option<Qos>,
    pub state_topic: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unique_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_template: Option<String>,
}

#[cfg(feature = "builder")]
impl BinarySensor {
    pub fn builder() -> BinarySensorBuilder {
        BinarySensorBuilder::default()
    }
}

impl Platform for BinarySensor {
    const PLATFORM: &'static str = "binary_sensor";
}
