mod support;

use hassliebe::{
    Component, Nullable,
    light::Light,
    sensor::{Sensor, SensorDeviceClass},
};
use pretty_assertions::assert_eq;

fn temperature() -> Component {
    Sensor {
        device_class: Some(SensorDeviceClass::Temperature),
        name: Some(Nullable::Null),
        state_topic: "sensorBedroom/state".into(),
        unique_id: Some("bla_sensor001".into()),
        value_template: Some("{{ value_json.temperature}}".into()),
        ..Default::default()
    }
    .into()
}

#[test]
fn sensor_snapshot() {
    insta::assert_json_snapshot!(temperature());
}

#[test]
fn removed_serializes_platform_only() {
    let removed = Component::Removed {
        platform: "sensor".into(),
    };
    assert_eq!(
        serde_json::to_string(&removed).unwrap(),
        r#"{"platform":"sensor"}"#
    );
}

#[test]
fn platform() {
    assert_eq!(temperature().platform(), "sensor");
    let light: Component = Light::default().into();
    assert_eq!(light.platform(), "light");
}

#[cfg(feature = "json")]
mod json {
    use super::*;
    use pretty_assertions::assert_eq;
    use serde_json::json;

    fn parse(value: serde_json::Value) -> Result<Component, serde_json::Error> {
        serde_json::from_value(value)
    }

    #[test]
    fn round_trip() {
        support::assert_round_trip(&temperature());
    }

    #[test]
    fn removal_entry() {
        assert_eq!(
            parse(json!({"platform": "sensor"})).unwrap(),
            Component::Removed {
                platform: "sensor".into()
            }
        );
    }

    #[test]
    fn unknown_platform_is_other() {
        let value = json!({
            "platform": "future_platform",
            "mode_command_topic": "hvac/mode/set",
            "modes": ["off", "heat"]
        });
        let component = parse(value.clone()).unwrap();
        let Component::Other { platform, config } = &component else {
            panic!("expected Other, got {component:?}");
        };
        assert_eq!(platform, "future_platform");
        assert!(!config.contains_key("platform"));
        assert_eq!(config["modes"], json!(["off", "heat"]));
        assert_eq!(serde_json::to_value(&component).unwrap(), value);
    }

    #[test]
    fn unknown_light_schema_is_other() {
        let component = parse(json!({
            "platform": "light",
            "schema": "future_schema",
            "command_topic": "kitchen/light/set"
        }))
        .unwrap();
        assert!(matches!(component, Component::Other { .. }));
        let basic = parse(json!({
            "platform": "light",
            "schema": "basic",
            "command_topic": "kitchen/light/set"
        }))
        .unwrap();
        assert!(matches!(basic, Component::Light(_)));
        let default =
            parse(json!({"platform": "light", "command_topic": "kitchen/light/set"})).unwrap();
        assert!(matches!(default, Component::Light(_)));
        let unknown =
            parse(json!({"platform": "light", "schema": 1, "command_topic": "t"})).unwrap();
        assert!(matches!(unknown, Component::Other { .. }));
    }

    #[test]
    fn invalid_entries() {
        assert!(parse(json!({"state_topic": "t"})).is_err());
        assert!(parse(json!({"platform": 1, "state_topic": "t"})).is_err());
        // Known platform, but the required `state_topic` is missing.
        assert!(parse(json!({"platform": "sensor", "name": "x"})).is_err());
    }
}
