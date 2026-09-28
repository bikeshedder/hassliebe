//! MQTT tag scanner: <https://www.home-assistant.io/integrations/tag.mqtt/>

// @generated begin component tag
use serde::{Deserialize, Serialize};

use crate::common::{device::Device, qos::Qos};
use crate::component::Platform;

/// An MQTT tag scanner: each message on `topic` is a scanned tag, whose ID
/// fires a tag scanned event. Not an entity.
///
/// <https://www.home-assistant.io/integrations/tag.mqtt/>
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "builder", derive(derive_builder::Builder))]
#[cfg_attr(feature = "builder", builder(setter(into, strip_option)))]
pub struct Tag {
    /// The device the tag scanner belongs to; needs `identifiers` or
    /// `connections`. Required per the docs, but optional in Home Assistant's
    /// source and missing from the docs' example; the builder requires it.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub device: Option<Device>,
    /// Encoding of received payloads. Not in the tag docs; accepted by Home
    /// Assistant, which ignores it and decodes `utf-8`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub encoding: Option<String>,
    /// Not in the tag docs; accepted by Home Assistant (shared base schema)
    /// but unused.
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::common::one_or_many::deserialize"
    )]
    #[cfg_attr(feature = "builder", builder(default))]
    pub group: Vec<String>,
    /// Maximum QoS level for the subscription. Not in the tag docs; accepted
    /// by Home Assistant. Default: `0`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub qos: Option<Qos>,
    /// Topic to receive tag scanned events from. Required.
    pub topic: String,
    /// Template that returns the tag ID. Default: the whole payload.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub value_template: Option<String>,
}

#[cfg(feature = "builder")]
impl Tag {
    /// Returns a builder for [`Tag`].
    pub fn builder() -> TagBuilder {
        TagBuilder::default()
    }
}

impl Platform for Tag {
    const PLATFORM: &'static str = "tag";
}
// @generated end
