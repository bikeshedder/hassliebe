mod support;

use hassliebe::{
    Availability, AvailabilityMode, Device, EntityCategory, Nullable, Origin, Qos,
    binary_sensor::{BinarySensor, BinarySensorDeviceClass},
};
use pretty_assertions::assert_eq;

fn full() -> BinarySensor {
    BinarySensor {
        availability: vec![Availability {
            payload_available: Some("1".into()),
            payload_not_available: Some("0".into()),
            topic: "hallway/door/status".into(),
            ..Default::default()
        }],
        availability_mode: Some(AvailabilityMode::Any),
        default_entity_id: Some("binary_sensor.hallway_door".into()),
        device: Some(Device {
            connections: vec![("mac".into(), "02:5b:26:a8:dc:12".into())],
            name: Some("Hallway door".into()),
            ..Default::default()
        }),
        device_class: Some(BinarySensorDeviceClass::Door),
        enabled_by_default: Some(true),
        encoding: Some("utf-8".into()),
        entity_category: Some(EntityCategory::Diagnostic),
        entity_picture: Some("https://example.com/door.png".into()),
        expire_after: Some(600),
        force_update: Some(true),
        group: vec!["hallway_window".into()],
        icon: Some("mdi:door".into()),
        json_attributes_template: Some("{{ value_json | tojson }}".into()),
        json_attributes_topic: Some("hallway/door/attributes".into()),
        message_expiry_interval: Some(60.into()),
        name: Some("Door".into()),
        off_delay: Some(30),
        origin: Some(Origin {
            name: "door2mqtt".into(),
            ..Default::default()
        }),
        payload_off: Some("closed".into()),
        payload_on: Some("open".into()),
        qos: Some(Qos::AtMostOnce),
        state_topic: "hallway/door/state".into(),
        unique_id: Some("hallway_door".into()),
        value_template: Some("{{ value_json.state }}".into()),
        visible_by_default: Some(false),
    }
}

#[test]
fn full_snapshot() {
    insta::assert_json_snapshot!(full());
}

#[test]
fn minimal_snapshot() {
    insta::assert_json_snapshot!(BinarySensor {
        state_topic: "hallway/door/state".into(),
        ..Default::default()
    });
}

#[test]
fn round_trip() {
    support::assert_round_trip(&full());
}

/// "Motion detection (binary sensor)" example from
/// <https://www.home-assistant.io/integrations/mqtt/>.
#[test]
fn docs_motion_example() {
    let sensor: BinarySensor = support::from_fixture("binary_sensor_motion.json");
    assert_eq!(
        sensor,
        BinarySensor {
            device: Some(Device {
                identifiers: vec!["01ad".into()],
                name: Some("Garden".into()),
                ..Default::default()
            }),
            device_class: Some(BinarySensorDeviceClass::Motion),
            name: Some(Nullable::Null),
            state_topic: "homeassistant/binary_sensor/garden/state".into(),
            unique_id: Some("motion01ad".into()),
            ..Default::default()
        }
    );
}

#[test]
fn group_accepts_single_string() {
    let sensor: BinarySensor =
        serde_json::from_str(r#"{"state_topic": "t", "group": "hallway_window"}"#).unwrap();
    assert_eq!(sensor.group, ["hallway_window"]);
}

#[cfg(feature = "builder")]
#[test]
fn builder_requires_state_topic() {
    assert!(BinarySensor::builder().build().is_err());
    assert!(BinarySensor::builder().state_topic("t").build().is_ok());
}
