mod support;

use hassliebe::{
    Availability, AvailabilityMode, Component, Device, EntityCategory, Origin, Qos,
    lawn_mower::LawnMower,
};
use pretty_assertions::assert_eq;

fn full() -> LawnMower {
    LawnMower {
        activity_state_topic: Some("lawn_mower_plus/state".into()),
        activity_value_template: Some("{{ value_json.activity }}".into()),
        availability: vec![Availability {
            topic: "lawn_mower_plus/status".into(),
            ..Default::default()
        }],
        availability_mode: Some(AvailabilityMode::Latest),
        default_entity_id: Some("lawn_mower.lawn_mower_plus".into()),
        device: Some(Device {
            identifiers: vec!["lawn_mower_plus_device".into()],
            name: Some("Robot mower".into()),
            ..Default::default()
        }),
        dock_command_template: Some("{\"activity\": \"{{ value }}\"}".into()),
        dock_command_topic: Some("lawn_mower_plus/set".into()),
        enabled_by_default: Some(true),
        encoding: Some("utf-8".into()),
        entity_category: Some(EntityCategory::Config),
        entity_picture: Some("https://example.com/lawn_mower_plus.png".into()),
        group: vec!["lawn_mower_plus_2".into()],
        icon: Some("mdi:robot-mower".into()),
        json_attributes_template: Some("{{ value_json | tojson }}".into()),
        json_attributes_topic: Some("lawn_mower_plus/attributes".into()),
        message_expiry_interval: Some(60.into()),
        name: Some("Lawn Mower Plus".into()),
        optimistic: Some(false),
        origin: Some(Origin {
            name: "lawn_mower_plus2mqtt".into(),
            ..Default::default()
        }),
        pause_command_template: Some("{\"activity\": \"{{ value }}\"}".into()),
        pause_command_topic: Some("lawn_mower_plus/set".into()),
        qos: Some(Qos::AtLeastOnce),
        retain: Some(false),
        start_mowing_command_template: Some("{\"activity\": \"{{ value }}\"}".into()),
        start_mowing_command_topic: Some("lawn_mower_plus/set".into()),
        stop_command_template: Some("{\"activity\": \"{{ value }}\"}".into()),
        stop_command_topic: Some("lawn_mower_plus/set".into()),
        unique_id: Some("lawn_mower_plus".into()),
        visible_by_default: Some(true),
    }
}

#[test]
fn full_snapshot() {
    insta::assert_json_snapshot!(full());
}

/// The configuration example from
/// <https://www.home-assistant.io/integrations/lawn_mower.mqtt/>.
#[test]
fn minimal_snapshot() {
    insta::assert_json_snapshot!(LawnMower {
        name: Some("Test Lawn Mower".into()),
        start_mowing_command_topic: Some("topic".into()),
        ..Default::default()
    });
}

#[test]
fn round_trip() {
    support::assert_round_trip(&full());
}

/// "Example" from <https://www.home-assistant.io/integrations/lawn_mower.mqtt/>, as JSON.
#[test]
fn docs_example() {
    let value: LawnMower = support::from_fixture("lawn_mower_plus.json");
    assert_eq!(
        value,
        LawnMower {
            activity_state_topic: Some("lawn_mower_plus/state".into()),
            activity_value_template: Some("{{ value_json.activity }}".into()),
            dock_command_template: Some("{\"activity\": \"{{ value }}\"}".into()),
            dock_command_topic: Some("lawn_mower_plus/set".into()),
            name: Some("Lawn Mower Plus".into()),
            pause_command_template: Some("{\"activity\": \"{{ value }}\"}".into()),
            pause_command_topic: Some("lawn_mower_plus/set".into()),
            start_mowing_command_template: Some("{\"activity\": \"{{ value }}\"}".into()),
            start_mowing_command_topic: Some("lawn_mower_plus/set".into()),
            ..Default::default()
        }
    );
}

#[test]
fn component() {
    let component = Component::from(full());
    assert_eq!(component.platform(), "lawn_mower");
    #[cfg(feature = "json")]
    {
        let json = serde_json::to_value(&component).unwrap();
        assert_eq!(json["platform"], "lawn_mower");
        let back: Component = serde_json::from_value(json).unwrap();
        assert_eq!(back, component);
    }
}

#[cfg(feature = "builder")]
#[test]
fn builder_has_no_required_fields() {
    assert!(LawnMower::builder().build().is_ok());
}
