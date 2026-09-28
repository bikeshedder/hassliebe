mod support;

use hassliebe::{
    Availability, AvailabilityMode, Device, EntityCategory, Nullable, Origin, Qos,
    cover::{Cover, CoverDeviceClass},
};
use pretty_assertions::assert_eq;

fn full() -> Cover {
    Cover {
        availability: vec![Availability {
            topic: "living-room-cover/availability".into(),
            ..Default::default()
        }],
        availability_mode: Some(AvailabilityMode::Latest),
        command_topic: Some("living-room-cover/set".into()),
        default_entity_id: Some("cover.living_room".into()),
        device: Some(Device {
            identifiers: vec!["living_room_cover".into()],
            name: Some("Living room cover".into()),
            ..Default::default()
        }),
        device_class: Some(CoverDeviceClass::Blind),
        enabled_by_default: Some(true),
        encoding: Some("utf-8".into()),
        entity_category: Some(EntityCategory::Config),
        entity_picture: Some("https://example.com/blind.png".into()),
        group: vec!["kitchen_cover".into()],
        icon: Some("mdi:blinds".into()),
        json_attributes_template: Some("{{ value_json | tojson }}".into()),
        json_attributes_topic: Some("living-room-cover/attributes".into()),
        message_expiry_interval: Some(60.into()),
        name: Some(Nullable::Null),
        optimistic: Some(false),
        origin: Some(Origin {
            name: "cover2mqtt".into(),
            ..Default::default()
        }),
        payload_close: Some("CLOSE".into()),
        payload_open: Some("OPEN".into()),
        payload_stop: Some(Nullable::Null),
        payload_stop_tilt: Some("stop".into()),
        position_closed: Some(0),
        position_open: Some(100),
        position_template: Some("{{ value_json.position }}".into()),
        position_topic: Some("living-room-cover/position".into()),
        qos: Some(Qos::AtLeastOnce),
        retain: Some(true),
        set_position_template: Some("{\"position\": {{ position }}}".into()),
        set_position_topic: Some("living-room-cover/position/set".into()),
        state_closed: Some("closed".into()),
        state_closing: Some("closing".into()),
        state_open: Some("open".into()),
        state_opening: Some("opening".into()),
        state_stopped: Some("stopped".into()),
        state_topic: Some("living-room-cover/state".into()),
        tilt_closed_value: Some(0),
        tilt_command_template: Some("{{ tilt_position }}".into()),
        tilt_command_topic: Some("living-room-cover/tilt/set".into()),
        tilt_max: Some(180),
        tilt_min: Some(0),
        tilt_opened_value: Some(180),
        tilt_optimistic: Some(false),
        tilt_status_template: Some("{{ value_json.tilt }}".into()),
        tilt_status_topic: Some("living-room-cover/tilt".into()),
        unique_id: Some("living_room_cover".into()),
        value_template: Some("{{ value_json.state }}".into()),
        visible_by_default: Some(true),
    }
}

#[test]
fn full_snapshot() {
    insta::assert_json_snapshot!(full());
}

#[test]
fn minimal_snapshot() {
    insta::assert_json_snapshot!(Cover {
        command_topic: Some("living-room-cover/set".into()),
        ..Default::default()
    });
}

#[test]
fn round_trip() {
    support::assert_round_trip(&full());
}

/// "Full configuration state topic without tilt" example from
/// <https://www.home-assistant.io/integrations/cover.mqtt/>, as JSON. The
/// legacy top-level `payload_available`/`payload_not_available` are not
/// modeled and are ignored.
#[test]
fn docs_state_topic_example() {
    let cover: Cover = support::from_fixture("cover_living_room.json");
    assert_eq!(
        cover,
        Cover {
            availability: vec![Availability {
                topic: "living-room-cover/availability".into(),
                ..Default::default()
            }],
            command_topic: Some("living-room-cover/set".into()),
            name: Some("MQTT Cover".into()),
            optimistic: Some(false),
            payload_close: Some("CLOSE".into()),
            payload_open: Some("OPEN".into()),
            payload_stop: Some("STOP".into()),
            qos: Some(Qos::AtMostOnce),
            retain: Some(true),
            state_closed: Some("closed".into()),
            state_closing: Some("closing".into()),
            state_open: Some("open".into()),
            state_opening: Some("opening".into()),
            state_topic: Some("living-room-cover/state".into()),
            value_template: Some("{{ value.x }}".into()),
            ..Default::default()
        }
    );
}

/// The docs: "For auto discovery message the payload needs to be set to
/// `null`" to disable a command.
#[test]
fn null_payload_disables_command() {
    let cover: Cover = serde_json::from_str(r#"{"payload_stop": null}"#).unwrap();
    assert_eq!(cover.payload_stop, Some(Nullable::Null));
    assert_eq!(cover.payload_open, None);
    assert_eq!(
        serde_json::to_string(&cover).unwrap(),
        r#"{"payload_stop":null}"#
    );
}

#[cfg(feature = "builder")]
#[test]
fn builder_has_no_required_fields() {
    assert!(Cover::builder().build().is_ok());
}
