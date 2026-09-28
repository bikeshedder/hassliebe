//! MQTT image: <https://www.home-assistant.io/integrations/image.mqtt/>

use crate::string_enum::string_enum;

// @generated begin component image
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

/// An MQTT image: shows an image file received on `image_topic`, or
/// downloaded from a URL received on `url_topic`.
///
/// <https://www.home-assistant.io/integrations/image.mqtt/>
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "builder", derive(derive_builder::Builder))]
#[cfg_attr(
    feature = "builder",
    builder(setter(into, strip_option), build_fn(validate = "Self::validate"))
)]
pub struct Image {
    /// Topics that report the availability of the entity.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub availability: Vec<Availability>,
    /// How the `availability` topics are combined. Default: `latest`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub availability_mode: Option<AvailabilityMode>,
    /// Content type of the images received on `image_topic`. Not allowed with
    /// `url_topic`. Default: `image/jpeg`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub content_type: Option<String>,
    /// Entity ID to use instead of one generated from the name, e.g.
    /// `image.doorbell_snapshot`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub default_entity_id: Option<String>,
    /// The device this image belongs to. Only works when `unique_id` is set.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub device: Option<Device>,
    /// Whether the entity is enabled when first added. Default: `true`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub enabled_by_default: Option<bool>,
    /// Encoding of received payloads; an empty string disables decoding. Not
    /// applied to images on `image_topic`, see `image_encoding`. Default:
    /// `utf-8`.
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
    /// Unique IDs of member entities, making this a group image. Home
    /// Assistant also accepts a single string.
    /// Not in the image docs; accepted by Home Assistant.
    #[serde(
        default,
        skip_serializing_if = "Vec::is_empty",
        deserialize_with = "crate::common::one_or_many::deserialize"
    )]
    #[cfg_attr(feature = "builder", builder(default))]
    pub group: Vec<String>,
    /// Icon for the entity, e.g. `mdi:image`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub icon: Option<String>,
    /// Encoding of the image payloads. Unset: raw binary image data.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub image_encoding: Option<ImageEncoding>,
    /// Topic to receive image files from. Exactly one of `image_topic` and
    /// `url_topic` is required; the builder checks this.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub image_topic: Option<String>,
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
    /// Not in the image docs; accepted by Home Assistant.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub message_expiry_interval: Option<MessageExpiryInterval>,
    /// Name of the image; [`Nullable::Null`] if only the device name is
    /// relevant. Default: `MQTT Image`.
    #[serde(
        default,
        skip_serializing_if = "Option::is_none",
        deserialize_with = "crate::common::nullable::deserialize"
    )]
    #[cfg_attr(feature = "builder", builder(default))]
    pub name: Option<Nullable<String>>,
    /// The application that sent the discovery message.
    /// Not in the image docs; accepted by Home Assistant.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub origin: Option<Origin>,
    /// Maximum QoS level for subscriptions. Default: `0`.
    /// Not in the image docs; accepted by Home Assistant.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub qos: Option<Qos>,
    /// ID that uniquely identifies this image. Required for device discovery
    /// and for `device`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub unique_id: Option<String>,
    /// Template to extract the image URL from messages on `url_topic`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub url_template: Option<String>,
    /// Topic to receive image URLs from. Exactly one of `image_topic` and
    /// `url_topic` is required.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub url_topic: Option<String>,
    /// Whether the entity is visible on dashboards when first added. Default:
    /// `true`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    #[cfg_attr(feature = "builder", builder(default))]
    pub visible_by_default: Option<bool>,
}

#[cfg(feature = "builder")]
impl Image {
    /// Returns a builder for [`Image`].
    pub fn builder() -> ImageBuilder {
        ImageBuilder::default()
    }
}

impl Platform for Image {
    const PLATFORM: &'static str = "image";
}
// @generated end

#[cfg(feature = "builder")]
impl ImageBuilder {
    /// Home Assistant requires exactly one of `image_topic` and `url_topic`,
    /// and rejects `content_type` together with `url_topic`.
    fn validate(&self) -> Result<(), String> {
        let image_topic = self.image_topic.as_ref().is_some_and(Option::is_some);
        let url_topic = self.url_topic.as_ref().is_some_and(Option::is_some);
        if image_topic == url_topic {
            return Err("exactly one of `image_topic` and `url_topic` must be set".into());
        }
        if url_topic && self.content_type.as_ref().is_some_and(Option::is_some) {
            return Err("`content_type` must not be set together with `url_topic`".into());
        }
        Ok(())
    }
}

string_enum! {
    /// Encoding of the image payloads on `image_topic`. Without it, payloads
    /// are raw binary image data.
    ///
    /// <https://www.home-assistant.io/integrations/image.mqtt/#image_encoding>
    ///
    /// Source: `homeassistant/components/mqtt/image.py`
    pub enum ImageEncoding {
        /// Base64 encoded image data.
        B64 = "b64",
        /// Raw binary image data. Not in the image docs; accepted by Home
        /// Assistant.
        Raw = "raw",
    }
}
