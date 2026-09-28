mod support;

use hassliebe::{
    Availability, AvailabilityMode, Component, Device, EntityCategory, Nullable, Origin, Qos,
    humidifier::{Humidifier, HumidifierDeviceClass},
};
use pretty_assertions::assert_eq;

fn full() -> Humidifier {
    Humidifier {
        action_template: Some("{{ value_json.action }}".into()),
        action_topic: Some("bedroom_humidifier/action".into()),
        availability: vec![Availability {
            topic: "bedroom_humidifier/status".into(),
            ..Default::default()
        }],
        availability_mode: Some(AvailabilityMode::Latest),
        command_template: Some("{\"state\": \"{{ value }}\"}".into()),
        command_topic: "bedroom_humidifier/on/set".into(),
        current_humidity_template: Some("{{ value_json.humidity }}".into()),
        current_humidity_topic: Some("bedroom_humidifier/humidity/current".into()),
        default_entity_id: Some("humidifier.bedroom_humidifier".into()),
        device: Some(Device {
            identifiers: vec!["bedroom_humidifier_device".into()],
            name: Some("Humidifier".into()),
            ..Default::default()
        }),
        device_class: Some(Nullable::Value(HumidifierDeviceClass::Humidifier)),
        enabled_by_default: Some(true),
        encoding: Some("utf-8".into()),
        entity_category: Some(EntityCategory::Config),
        entity_picture: Some("https://example.com/bedroom_humidifier.png".into()),
        group: vec!["bedroom_humidifier_2".into()],
        icon: Some("mdi:air-humidifier".into()),
        json_attributes_template: Some("{{ value_json | tojson }}".into()),
        json_attributes_topic: Some("bedroom_humidifier/attributes".into()),
        max_humidity: Some(80.0),
        message_expiry_interval: Some(60.into()),
        min_humidity: Some(30.0),
        mode_command_template: Some("{{ value }}".into()),
        mode_command_topic: Some("bedroom_humidifier/preset/preset_mode".into()),
        mode_state_template: Some("{{ value_json.mode }}".into()),
        mode_state_topic: Some("bedroom_humidifier/mode/state".into()),
        modes: vec![
            "normal".into(),
            "eco".into(),
            "away".into(),
            "boost".into(),
            "comfort".into(),
            "home".into(),
            "sleep".into(),
            "auto".into(),
            "baby".into(),
        ],
        name: Some("Bedroom humidifier".into()),
        optimistic: Some(false),
        origin: Some(Origin {
            name: "bedroom_humidifier2mqtt".into(),
            ..Default::default()
        }),
        payload_off: Some("false".into()),
        payload_on: Some("true".into()),
        payload_reset_humidity: Some("None".into()),
        payload_reset_mode: Some("None".into()),
        qos: Some(Qos::AtLeastOnce),
        retain: Some(true),
        state_topic: Some("bedroom_humidifier/on/state".into()),
        state_value_template: Some("{{ value_json.state }}".into()),
        target_humidity_command_template: Some("{{ value | int }}".into()),
        target_humidity_command_topic: "bedroom_humidifier/humidity/set".into(),
        target_humidity_state_template: Some("{{ value_json.target }}".into()),
        target_humidity_state_topic: Some("bedroom_humidifier/humidity/state".into()),
        unique_id: Some("bedroom_humidifier".into()),
        visible_by_default: Some(true),
    }
}

#[test]
fn full_snapshot() {
    insta::assert_json_snapshot!(full());
}

/// The configuration example from
/// <https://www.home-assistant.io/integrations/humidifier.mqtt/>.
#[test]
fn minimal_snapshot() {
    insta::assert_json_snapshot!(Humidifier {
        command_topic: "bedroom_humidifier/on/set".into(),
        target_humidity_command_topic: "bedroom_humidifier/humidity/set".into(),
        ..Default::default()
    });
}

#[test]
fn round_trip() {
    support::assert_round_trip(&full());
}

/// "Full configuration" example from <https://www.home-assistant.io/integrations/humidifier.mqtt/>, as JSON.
#[test]
fn docs_full_example() {
    let value: Humidifier = support::from_fixture("humidifier_bedroom.json");
    assert_eq!(
        value,
        Humidifier {
            action_topic: Some("bedroom_humidifier/action".into()),
            command_topic: "bedroom_humidifier/on/set".into(),
            current_humidity_topic: Some("bedroom_humidifier/humidity/current".into()),
            device_class: Some(Nullable::Value(HumidifierDeviceClass::Humidifier)),
            max_humidity: Some(80.0),
            min_humidity: Some(30.0),
            mode_command_topic: Some("bedroom_humidifier/preset/preset_mode".into()),
            mode_state_topic: Some("bedroom_humidifier/mode/state".into()),
            modes: vec![
                "normal".into(),
                "eco".into(),
                "away".into(),
                "boost".into(),
                "comfort".into(),
                "home".into(),
                "sleep".into(),
                "auto".into(),
                "baby".into()
            ],
            name: Some("Bedroom humidifier".into()),
            payload_off: Some("false".into()),
            payload_on: Some("true".into()),
            qos: Some(Qos::AtMostOnce),
            state_topic: Some("bedroom_humidifier/on/state".into()),
            target_humidity_command_topic: "bedroom_humidifier/humidity/set".into(),
            target_humidity_state_topic: Some("bedroom_humidifier/humidity/state".into()),
            ..Default::default()
        }
    );
}

#[test]
fn component() {
    let component = Component::from(full());
    assert_eq!(component.platform(), "humidifier");
    #[cfg(feature = "json")]
    {
        let json = serde_json::to_value(&component).unwrap();
        assert_eq!(json["platform"], "humidifier");
        let back: Component = serde_json::from_value(json).unwrap();
        assert_eq!(back, component);
    }
}

#[cfg(feature = "builder")]
#[test]
fn builder_requires_fields() {
    assert!(Humidifier::builder().build().is_err());
    assert!(
        Humidifier::builder()
            .command_topic("t")
            .target_humidity_command_topic("h")
            .build()
            .is_ok()
    );
}

#[test]
fn null_device_class() {
    let humidifier: Humidifier = serde_json::from_str(
        r#"{"command_topic": "t", "target_humidity_command_topic": "h", "device_class": null}"#,
    )
    .unwrap();
    assert_eq!(humidifier.device_class, Some(Nullable::Null));
}

#[cfg(feature = "builder")]
#[test]
fn builder_requires_modes_with_mode_command_topic() {
    let builder = || {
        Humidifier::builder()
            .command_topic("t")
            .target_humidity_command_topic("h")
            .clone()
    };
    assert!(builder().mode_command_topic("m").build().is_err());
    assert!(
        builder()
            .mode_command_topic("m")
            .modes(vec!["eco".to_string()])
            .build()
            .is_ok()
    );
}
