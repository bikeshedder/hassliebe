mod support;

use hassliebe::{
    Availability, AvailabilityMode, Component, Device, EntityCategory, Origin, Qos,
    vacuum::{Vacuum, VacuumFeature},
};
use pretty_assertions::assert_eq;

fn full() -> Vacuum {
    Vacuum {
        availability: vec![Availability {
            topic: "vacuum/living_room/status".into(),
            ..Default::default()
        }],
        availability_mode: Some(AvailabilityMode::Latest),
        clean_segments_command_template: Some("{{ value | tojson }}".into()),
        clean_segments_command_topic: Some("vacuum/living_room/clean_segments".into()),
        command_topic: Some("vacuum/living_room/command".into()),
        default_entity_id: Some("vacuum.vacuum_living_room".into()),
        device: Some(Device {
            identifiers: vec!["vacuum_living_room_device".into()],
            name: Some("Robot vacuum".into()),
            ..Default::default()
        }),
        enabled_by_default: Some(true),
        encoding: Some("utf-8".into()),
        entity_category: Some(EntityCategory::Config),
        entity_picture: Some("https://example.com/vacuum_living_room.png".into()),
        fan_speed_list: vec!["min".into(), "medium".into(), "high".into(), "max".into()],
        group: vec!["vacuum_living_room_2".into()],
        icon: Some("mdi:robot-vacuum".into()),
        json_attributes_template: Some("{{ value_json | tojson }}".into()),
        json_attributes_topic: Some("vacuum/living_room/attributes".into()),
        message_expiry_interval: Some(60.into()),
        name: Some("Living room".into()),
        origin: Some(Origin {
            name: "vacuum_living_room2mqtt".into(),
            ..Default::default()
        }),
        payload_clean_spot: Some("clean_spot".into()),
        payload_locate: Some("locate".into()),
        payload_pause: Some("pause".into()),
        payload_return_to_base: Some("return_to_base".into()),
        payload_start: Some("start".into()),
        payload_stop: Some("stop".into()),
        qos: Some(Qos::AtLeastOnce),
        retain: Some(false),
        send_command_topic: Some("vacuum/living_room/send_command".into()),
        set_fan_speed_topic: Some("vacuum/living_room/set_fan_speed".into()),
        state_topic: Some("vacuum/living_room/state".into()),
        supported_features: Some(vec![
            VacuumFeature::Start,
            VacuumFeature::Stop,
            VacuumFeature::ReturnHome,
            VacuumFeature::FanSpeed,
        ]),
        unique_id: Some("vacuum_living_room".into()),
        visible_by_default: Some(true),
    }
}

#[test]
fn full_snapshot() {
    insta::assert_json_snapshot!(full());
}

/// The configuration example from
/// <https://www.home-assistant.io/integrations/vacuum.mqtt/>.
#[test]
fn minimal_snapshot() {
    insta::assert_json_snapshot!(Vacuum {
        command_topic: Some("command-topic".into()),
        state_topic: Some("state-topic".into()),
        ..Default::default()
    });
}

#[test]
fn round_trip() {
    support::assert_round_trip(&full());
}

/// "Configuration example" from <https://www.home-assistant.io/integrations/vacuum.mqtt/>, as JSON.
#[test]
fn docs_example() {
    let value: Vacuum = support::from_fixture("vacuum_full.json");
    assert_eq!(
        value,
        Vacuum {
            clean_segments_command_topic: Some("vacuum/clean_segments".into()),
            command_topic: Some("vacuum/command".into()),
            fan_speed_list: vec!["min".into(), "medium".into(), "high".into(), "max".into()],
            name: Some("MQTT Vacuum".into()),
            send_command_topic: Some("vacuum/send_command".into()),
            set_fan_speed_topic: Some("vacuum/set_fan_speed".into()),
            supported_features: Some(vec![
                VacuumFeature::Start,
                VacuumFeature::Pause,
                VacuumFeature::Stop,
                VacuumFeature::ReturnHome,
                VacuumFeature::Status,
                VacuumFeature::Locate,
                VacuumFeature::CleanSpot,
                VacuumFeature::FanSpeed,
                VacuumFeature::SendCommand
            ]),
            ..Default::default()
        }
    );
}

#[test]
fn component() {
    let component = Component::from(full());
    assert_eq!(component.platform(), "vacuum");
    #[cfg(feature = "json")]
    {
        let json = serde_json::to_value(&component).unwrap();
        assert_eq!(json["platform"], "vacuum");
        let back: Component = serde_json::from_value(json).unwrap();
        assert_eq!(back, component);
    }
}

#[cfg(feature = "builder")]
#[test]
fn builder_has_no_required_fields() {
    assert!(Vacuum::builder().build().is_ok());
}

#[test]
fn supported_features_accepts_single_string() {
    let vacuum: Vacuum = serde_json::from_str(r#"{"supported_features": "start"}"#).unwrap();
    assert_eq!(vacuum.supported_features, Some(vec![VacuumFeature::Start]));
}

#[cfg(feature = "builder")]
#[test]
fn builder_requires_unique_id_for_clean_segments() {
    assert!(
        Vacuum::builder()
            .clean_segments_command_topic("t")
            .build()
            .is_err()
    );
    assert!(
        Vacuum::builder()
            .clean_segments_command_topic("t")
            .unique_id("vacuum")
            .build()
            .is_ok()
    );
}
