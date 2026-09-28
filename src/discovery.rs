//! Discovery topics, Home Assistant's status topic, and device-based discovery
//! (one message that announces a device with all of its components).
//!
//! <https://www.home-assistant.io/integrations/mqtt/#mqtt-discovery>

use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::{
    common::{
        availability::{Availability, AvailabilityMode},
        device::Device,
        message_expiry_interval::MessageExpiryInterval,
        origin::Origin,
        qos::Qos,
    },
    component::{Component, Platform},
};

/// The default discovery prefix.
pub const DEFAULT_PREFIX: &str = "homeassistant";

/// The topic Home Assistant publishes its birth and last will messages to, by
/// default. Devices should subscribe to it and publish their discovery
/// payloads again when [`STATUS_ONLINE`] arrives.
pub const STATUS_TOPIC: &str = "homeassistant/status";

/// Home Assistant's default birth payload on [`STATUS_TOPIC`].
pub const STATUS_ONLINE: &str = "online";

/// Home Assistant's default last will payload on [`STATUS_TOPIC`].
pub const STATUS_OFFLINE: &str = "offline";

/// Returns the discovery topic
/// `<prefix>/<component>/[<node_id>/]<object_id>/config`, where `component` is
/// a platform like `sensor` or `device` for device-based discovery.
///
/// `node_id` and `object_id` must only contain `[a-zA-Z0-9_-]` (see
/// [`is_valid_id`]); Home Assistant ignores messages on other topics. This
/// function does not check them.
pub fn discovery_topic(
    prefix: &str,
    component: &str,
    node_id: Option<&str>,
    object_id: &str,
) -> String {
    match node_id {
        Some(node_id) => format!("{prefix}/{component}/{node_id}/{object_id}/config"),
        None => format!("{prefix}/{component}/{object_id}/config"),
    }
}

/// Returns the single component discovery topic for `config`, e.g.
/// `homeassistant/sensor/garden/temperature/config`. See [`discovery_topic`].
pub fn component_topic<P: Platform>(
    prefix: &str,
    _config: &P,
    node_id: Option<&str>,
    object_id: &str,
) -> String {
    discovery_topic(prefix, P::PLATFORM, node_id, object_id)
}

/// Returns the device discovery topic, e.g.
/// `homeassistant/device/kitchen/config`. See [`discovery_topic`].
pub fn device_topic(prefix: &str, node_id: Option<&str>, object_id: &str) -> String {
    discovery_topic(prefix, "device", node_id, object_id)
}

/// Returns whether `id` is a valid `node_id` or `object_id`: not empty and only
/// `[a-zA-Z0-9_-]`.
pub fn is_valid_id(id: &str) -> bool {
    !id.is_empty()
        && id
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b == b'_' || b == b'-')
}

/// A device discovery payload, published to
/// `<discovery_prefix>/device/[<node_id>/]<object_id>/config`.
///
/// Serializing always works. Deserializing needs the `json` feature (see
/// [`Component`]). When reading, the shared options are copied into every
/// component that does not set them, as Home Assistant does, so a payload that
/// relies on a shared `state_topic` still yields complete components. A value
/// whose components leave shared options unset therefore reads back with them
/// set.
///
/// When writing, each component must be complete on its own: a
/// [`Sensor`](crate::sensor::Sensor) in `components` needs its own
/// `state_topic`, even if `shared.state_topic` is set.
///
/// <https://www.home-assistant.io/integrations/mqtt/#device-discovery-payload>
#[derive(Clone, Debug, Default, PartialEq, Serialize)]
#[cfg_attr(feature = "builder", derive(derive_builder::Builder))]
#[cfg_attr(feature = "builder", builder(setter(into)))]
pub struct DeviceDiscovery {
    /// The components of the device, keyed by an ID that is unique within the
    /// device. Required.
    pub components: BTreeMap<String, Component>,
    /// The device. Required.
    pub device: Device,
    /// The application that sent the message. Required.
    pub origin: Origin,
    /// Options inherited by all components that do not set them.
    #[serde(flatten)]
    #[cfg_attr(feature = "builder", builder(default))]
    pub shared: SharedOptions,
}

#[cfg(feature = "builder")]
impl DeviceDiscovery {
    /// Returns a builder for [`DeviceDiscovery`].
    pub fn builder() -> DeviceDiscoveryBuilder {
        DeviceDiscoveryBuilder::default()
    }
}

/// Options at the root of a [`DeviceDiscovery`] payload that every component
/// inherits unless it sets them itself.
///
/// The docs list the availability options, `command_topic`, `encoding`, `qos`
/// and `state_topic`; Home Assistant also shares `message_expiry_interval`.
///
/// <https://www.home-assistant.io/integrations/mqtt/#device-discovery-payload>
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[cfg_attr(feature = "builder", derive(derive_builder::Builder))]
#[cfg_attr(feature = "builder", builder(default, setter(into, strip_option)))]
pub struct SharedOptions {
    /// Topics that report the availability of the components.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub availability: Vec<Availability>,
    /// How the `availability` topics are combined. Default: `latest`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub availability_mode: Option<AvailabilityMode>,
    /// Topic to publish commands to.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub command_topic: Option<String>,
    /// Encoding of payloads. Default: `utf-8`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub encoding: Option<String>,
    /// Message expiry interval for command payloads. Not in the docs' list of
    /// shared options; shared by Home Assistant.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message_expiry_interval: Option<MessageExpiryInterval>,
    /// QoS level. Default: `0`.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub qos: Option<Qos>,
    /// Topic to receive state updates from.
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub state_topic: Option<String>,
}

#[cfg(feature = "json")]
impl<'de> Deserialize<'de> for DeviceDiscovery {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        use serde::de::Error;
        use serde_json::{Map, Value};

        fn take<T: serde::de::DeserializeOwned, E: Error>(
            root: &mut Map<String, Value>,
            key: &'static str,
        ) -> Result<T, E> {
            let value = root.remove(key).ok_or_else(|| E::missing_field(key))?;
            serde_json::from_value(value).map_err(E::custom)
        }

        let mut root = Map::deserialize(deserializer)?;
        let device = take(&mut root, "device")?;
        let origin = take(&mut root, "origin")?;
        let mut components: Map<String, Value> = take(&mut root, "components")?;
        let shared: SharedOptions =
            serde_json::from_value(Value::Object(root)).map_err(D::Error::custom)?;

        // Like `_merge_common_device_options` in Home Assistant's
        // `mqtt/discovery.py`; removal entries (only `platform`) are left alone.
        let Value::Object(shared_values) =
            serde_json::to_value(&shared).map_err(D::Error::custom)?
        else {
            return Err(D::Error::custom(
                "shared options must serialize to an object",
            ));
        };
        for config in components.values_mut() {
            if let Value::Object(config) = config
                && config.keys().any(|key| key != "platform")
            {
                for (key, value) in &shared_values {
                    config.entry(key).or_insert_with(|| value.clone());
                }
            }
        }
        let components =
            serde_json::from_value(Value::Object(components)).map_err(D::Error::custom)?;

        Ok(Self {
            components,
            device,
            origin,
            shared,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sensor::Sensor;
    use pretty_assertions::assert_eq;

    #[test]
    fn topics() {
        assert_eq!(
            component_topic(DEFAULT_PREFIX, &Sensor::default(), None, "temp01"),
            "homeassistant/sensor/temp01/config"
        );
        assert_eq!(
            component_topic("ha", &Sensor::default(), Some("garden"), "temp01"),
            "ha/sensor/garden/temp01/config"
        );
        assert_eq!(
            device_topic(DEFAULT_PREFIX, None, "0AFFD2"),
            "homeassistant/device/0AFFD2/config"
        );
        assert_eq!(
            device_topic(DEFAULT_PREFIX, Some("bridge"), "0AFFD2"),
            "homeassistant/device/bridge/0AFFD2/config"
        );
    }

    #[test]
    fn valid_ids() {
        assert!(is_valid_id("garden_01-a"));
        for id in ["", "a b", "a/b", "a.b", "ä", "a+"] {
            assert!(!is_valid_id(id), "{id:?}");
        }
    }
}
