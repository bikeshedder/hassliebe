use serde::Serialize;

use super::Platform;

use crate::common::{
    availability::{Availability, AvailabilityMode},
    device::Device,
    qos::Qos,
};

/// <https://www.home-assistant.io/integrations/select.mqtt/>
#[derive(Clone, Debug, Default, Serialize)]
#[cfg_attr(feature = "builder", derive(derive_builder::Builder))]
#[cfg_attr(feature = "builder", builder(default, setter(into, strip_option)))]
pub struct Select {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availabilty: Option<Vec<Availability>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_topic: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_mode: Option<AvailabilityMode>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub availability_template: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub command_template: Option<String>,
    pub command_topic: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub default_entity_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub device: Option<Device>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled_by_default: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encoding: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_category: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub entity_picture: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub json_attributes_template: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub json_attributes_topic: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub icon: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub optimistic: Option<bool>,
    pub options: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub qos: Option<Qos>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub retain: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub state_topic: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub unique_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value_template: Option<String>,
}

#[cfg(feature = "builder")]
impl Select {
    pub fn builder() -> SelectBuilder {
        SelectBuilder::default()
    }
}

impl Platform for Select {
    const PLATFORM: &'static str = "select";
}
