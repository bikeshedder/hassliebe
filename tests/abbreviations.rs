#![cfg(feature = "json")]

mod support;

use hassliebe::{
    Component, DeviceDiscovery,
    abbreviations::{expand_component_payload, expand_device_payload},
    switch::Switch,
};
use pretty_assertions::assert_eq;
use serde_json::{Value, json};

fn expanded_component(name: &str) -> Value {
    let mut payload: Value = support::from_fixture(name);
    expand_component_payload(&mut payload);
    payload
}

/// The device discovery example from
/// <https://www.home-assistant.io/integrations/mqtt/> must read the same
/// abbreviated and expanded.
#[test]
fn device_discovery_example() {
    let mut payload: Value = support::from_fixture("device_discovery_kitchen_abbreviated.json");
    expand_device_payload(&mut payload);
    let abbreviated: DeviceDiscovery = serde_json::from_value(payload).unwrap();
    let expanded: DeviceDiscovery = support::from_fixture("device_discovery_kitchen.json");
    assert_eq!(abbreviated, expanded);
}

/// "Using abbreviations and base topic" example from
/// <https://www.home-assistant.io/integrations/mqtt/>.
#[test]
fn switch_with_base_topic() {
    let switch: Switch =
        serde_json::from_value(expanded_component("switch_irrigation_abbreviated.json")).unwrap();
    assert_eq!(
        switch,
        Switch {
            command_topic: "homeassistant/switch/irrigation/set".into(),
            name: Some("garden".into()),
            state_topic: Some("homeassistant/switch/irrigation/state".into()),
            ..Default::default()
        }
    );
}

/// "Example with using abbreviated Device and Origin info in discovery
/// messages" from <https://www.home-assistant.io/integrations/mqtt/>. It is a
/// JSON schema light, which this crate keeps as `Component::Other`.
#[test]
fn light_with_abbreviated_device_and_origin() {
    let mut payload = expanded_component("light_kitchen_abbreviated.json");
    assert_eq!(
        payload,
        json!({
            "name": null,
            "unique_id": "kitchen_light",
            "command_topic": "homeassistant/light/kitchen/set",
            "state_topic": "homeassistant/light/kitchen/state",
            "schema": "json",
            "device": {
                "identifiers": "ea334450945afc",
                "name": "Kitchen",
                "manufacturer": "Bla electronics",
                "model": "xya",
                "model_id": "ABC123",
                "sw_version": "1.0",
                "serial_number": "ea334450945afc",
                "hw_version": "1.0rev2"
            },
            "origin": {
                "name": "bla2mqtt",
                "sw_version": "2.1",
                "support_url": "https://bla2mqtt.example.com/support"
            }
        })
    );
    payload["platform"] = json!("light");
    let component: Component = serde_json::from_value(payload).unwrap();
    assert!(matches!(component, Component::Other { .. }));
}

#[test]
fn device_payload_components_and_removal() {
    let mut payload = json!({
        "dev": {"ids": "d1", "cns": [["mac", "02:5b:26:a8:dc:12"]]},
        "o": {"name": "app", "url": "https://example.com"},
        "stat_t": "shared/state",
        "cmps": {
            "a": {"p": "sensor"},
            "b": {"p": "binary_sensor", "~": "base/b", "stat_t": "~/own", "avty": [{"t": "~/avail"}]},
            "c": {"p": "sensor", "uniq_id": "c"}
        }
    });
    expand_device_payload(&mut payload);
    assert_eq!(
        payload,
        json!({
            "device": {"identifiers": "d1", "connections": [["mac", "02:5b:26:a8:dc:12"]]},
            "origin": {"name": "app", "support_url": "https://example.com"},
            "state_topic": "shared/state",
            "components": {
                "a": {"platform": "sensor"},
                "b": {
                    "platform": "binary_sensor",
                    "state_topic": "base/b/own",
                    "availability": [{"topic": "base/b/avail"}]
                },
                "c": {"platform": "sensor", "unique_id": "c", "state_topic": "shared/state"}
            }
        })
    );
    let discovery: DeviceDiscovery = serde_json::from_value(payload).unwrap();
    assert_eq!(discovery.components.len(), 3);
}
