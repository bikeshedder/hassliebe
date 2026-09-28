mod support;

use std::collections::BTreeMap;

use hassliebe::{
    Component, Device, DeviceDiscovery, Origin, Qos, SharedOptions,
    sensor::{Sensor, SensorDeviceClass},
    unit::{UnitOfRatio, UnitOfTemperature},
};

/// The device discovery example from
/// <https://www.home-assistant.io/integrations/mqtt/>, written with this crate.
/// Unlike the docs, each sensor repeats the shared `state_topic`, because
/// `Sensor` requires it.
fn kitchen() -> DeviceDiscovery {
    let sensor = |device_class, unit, value, unique_id: &str| {
        Component::from(Sensor {
            device_class: Some(device_class),
            state_topic: "sensorBedroom/state".into(),
            unique_id: Some(unique_id.into()),
            unit_of_measurement: Some(unit),
            value_template: Some(format!("{{{{ value_json.{value}}}}}")),
            ..Default::default()
        })
    };
    DeviceDiscovery {
        components: BTreeMap::from([
            (
                "some_unique_component_id1".into(),
                sensor(
                    SensorDeviceClass::Temperature,
                    UnitOfTemperature::Celsius.into(),
                    "temperature",
                    "temp01ae_t",
                ),
            ),
            (
                "some_unique_id2".into(),
                sensor(
                    SensorDeviceClass::Humidity,
                    UnitOfRatio::Percentage.into(),
                    "humidity",
                    "temp01ae_h",
                ),
            ),
        ]),
        device: Device {
            hw_version: Some("1.0rev2".into()),
            identifiers: vec!["ea334450945afc".into()],
            manufacturer: Some("Bla electronics".into()),
            model: Some("xya".into()),
            name: Some("Kitchen".into()),
            serial_number: Some("ea334450945afc".into()),
            sw_version: Some("1.0".into()),
            ..Default::default()
        },
        origin: Origin {
            name: "bla2mqtt".into(),
            support_url: Some("https://bla2mqtt.example.com/support".into()),
            sw_version: Some("2.1".into()),
        },
        shared: SharedOptions {
            qos: Some(Qos::ExactlyOnce),
            state_topic: Some("sensorBedroom/state".into()),
            ..Default::default()
        },
    }
}

#[test]
fn kitchen_snapshot() {
    insta::assert_json_snapshot!(kitchen());
}

#[cfg(feature = "json")]
mod json {
    use super::*;
    use pretty_assertions::assert_eq;
    use serde_json::json;

    /// [`kitchen`] as it reads back: the sensors inherit the shared `qos`.
    fn kitchen_inherited() -> DeviceDiscovery {
        let mut discovery = kitchen();
        for component in discovery.components.values_mut() {
            if let Component::Sensor(sensor) = component {
                sensor.qos = Some(Qos::ExactlyOnce);
            }
        }
        discovery
    }

    /// The docs example (keys expanded). The sensors have no `state_topic` of
    /// their own; they inherit it, and `qos`, from the shared options.
    #[test]
    fn docs_example_inherits_shared_options() {
        let discovery: DeviceDiscovery = support::from_fixture("device_discovery_kitchen.json");
        assert_eq!(discovery, kitchen_inherited());
    }

    /// Reading copies shared options into the components, so only a value
    /// that already has them survives a round trip unchanged.
    #[test]
    fn round_trip() {
        let json = serde_json::to_string(&kitchen()).unwrap();
        let back: DeviceDiscovery = serde_json::from_str(&json).unwrap();
        assert_eq!(back, kitchen_inherited());
        support::assert_round_trip(&kitchen_inherited());
    }

    #[test]
    fn component_options_override_shared_ones() {
        let discovery: DeviceDiscovery = serde_json::from_value(json!({
            "device": {"identifiers": "d1", "name": "D"},
            "origin": {"name": "app"},
            "state_topic": "shared/state",
            "components": {
                "a": {"platform": "sensor"},
                "b": {"platform": "sensor", "state_topic": "own/state"},
                "c": {"platform": "sensor", "unique_id": "c"}
            }
        }))
        .unwrap();
        let state_topic = |id: &str| match &discovery.components[id] {
            Component::Sensor(sensor) => sensor.state_topic.clone(),
            other => panic!("unexpected {other:?}"),
        };
        assert_eq!(
            discovery.components["a"],
            Component::Removed {
                platform: "sensor".into()
            }
        );
        assert_eq!(state_topic("b"), "own/state");
        assert_eq!(state_topic("c"), "shared/state");
    }

    #[test]
    fn required_keys() {
        let full = json!({
            "device": {"identifiers": "d1"},
            "origin": {"name": "app"},
            "components": {}
        });
        assert!(serde_json::from_value::<DeviceDiscovery>(full.clone()).is_ok());
        for key in ["device", "origin", "components"] {
            let mut value = full.clone();
            value.as_object_mut().unwrap().remove(key);
            let error = serde_json::from_value::<DeviceDiscovery>(value).unwrap_err();
            assert!(error.to_string().contains(key), "{error}");
        }
    }
}

#[cfg(feature = "builder")]
#[test]
fn builder_requires_device_origin_and_components() {
    assert!(
        DeviceDiscovery::builder()
            .device(Device::default())
            .build()
            .is_err()
    );
    let discovery = DeviceDiscovery::builder()
        .device(Device::default())
        .origin(Origin::default())
        .components(BTreeMap::new())
        .build()
        .unwrap();
    assert_eq!(discovery.shared, SharedOptions::default());
}
