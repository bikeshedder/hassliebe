mod support;

use hassliebe::{
    Availability, AvailabilityMode, Component, Device, EntityCategory, Origin, Qos, fan::Fan,
};
use pretty_assertions::assert_eq;

fn full() -> Fan {
    Fan {
        availability: vec![Availability {
            topic: "bedroom_fan/status".into(),
            ..Default::default()
        }],
        availability_mode: Some(AvailabilityMode::Latest),
        command_template: Some("{ state: '{{ value }}'}".into()),
        command_topic: "bedroom_fan/on/set".into(),
        default_entity_id: Some("fan.bedroom_fan".into()),
        device: Some(Device {
            identifiers: vec!["bedroom_fan_device".into()],
            name: Some("Ceiling fan".into()),
            ..Default::default()
        }),
        direction_command_template: Some("{{ iif(value == 'forward', 'fwd', 'rev') }}".into()),
        direction_command_topic: Some("bedroom_fan/direction/set".into()),
        direction_state_topic: Some("bedroom_fan/direction/state".into()),
        direction_value_template: Some("{{ iif(value == 'fwd', 'forward', 'reverse') }}".into()),
        enabled_by_default: Some(true),
        encoding: Some("utf-8".into()),
        entity_category: Some(EntityCategory::Config),
        entity_picture: Some("https://example.com/bedroom_fan.png".into()),
        group: vec!["bedroom_fan_2".into()],
        icon: Some("mdi:ceiling-fan".into()),
        json_attributes_template: Some("{{ value_json | tojson }}".into()),
        json_attributes_topic: Some("bedroom_fan/attributes".into()),
        message_expiry_interval: Some(60.into()),
        name: Some("Bedroom Fan".into()),
        optimistic: Some(false),
        origin: Some(Origin {
            name: "bedroom_fan2mqtt".into(),
            ..Default::default()
        }),
        oscillation_command_template: Some("{ oscillation: '{{ value }}'}".into()),
        oscillation_command_topic: Some("bedroom_fan/oscillation/set".into()),
        oscillation_state_topic: Some("bedroom_fan/oscillation/state".into()),
        oscillation_value_template: Some("{{ value_json.oscillation }}".into()),
        payload_off: Some("false".into()),
        payload_on: Some("true".into()),
        payload_oscillation_off: Some("false".into()),
        payload_oscillation_on: Some("true".into()),
        payload_reset_percentage: Some("None".into()),
        payload_reset_preset_mode: Some("None".into()),
        percentage_command_template: Some("{ percentage: '{{ value }}'}".into()),
        percentage_command_topic: Some("bedroom_fan/speed/percentage".into()),
        percentage_state_topic: Some("bedroom_fan/speed/percentage_state".into()),
        percentage_value_template: Some("{{ value_json.percentage }}".into()),
        preset_mode_command_template: Some("{ preset_mode: '{{ value }}'}".into()),
        preset_mode_command_topic: Some("bedroom_fan/preset/preset_mode".into()),
        preset_mode_state_topic: Some("bedroom_fan/preset/preset_mode_state".into()),
        preset_mode_value_template: Some("{{ value_json.preset_mode }}".into()),
        preset_modes: vec![
            "auto".into(),
            "smart".into(),
            "whoosh".into(),
            "eco".into(),
            "breeze".into(),
        ],
        qos: Some(Qos::AtLeastOnce),
        retain: Some(true),
        speed_range_max: Some(10),
        speed_range_min: Some(1),
        state_topic: Some("bedroom_fan/on/state".into()),
        state_value_template: Some("{{ value_json.state }}".into()),
        unique_id: Some("bedroom_fan".into()),
        visible_by_default: Some(true),
    }
}

#[test]
fn full_snapshot() {
    insta::assert_json_snapshot!(full());
}

/// The configuration example from
/// <https://www.home-assistant.io/integrations/fan.mqtt/>.
#[test]
fn minimal_snapshot() {
    insta::assert_json_snapshot!(Fan {
        command_topic: "bedroom_fan/on/set".into(),
        ..Default::default()
    });
}

#[test]
fn round_trip() {
    support::assert_round_trip(&full());
}

/// "Full configuration" example from <https://www.home-assistant.io/integrations/fan.mqtt/>, as JSON.
#[test]
fn docs_full_example() {
    let value: Fan = support::from_fixture("fan_bedroom.json");
    assert_eq!(
        value,
        Fan {
            command_topic: "bedroom_fan/on/set".into(),
            direction_command_topic: Some("bedroom_fan/direction/set".into()),
            direction_state_topic: Some("bedroom_fan/direction/state".into()),
            name: Some("Bedroom Fan".into()),
            oscillation_command_topic: Some("bedroom_fan/oscillation/set".into()),
            oscillation_state_topic: Some("bedroom_fan/oscillation/state".into()),
            payload_off: Some("false".into()),
            payload_on: Some("true".into()),
            payload_oscillation_off: Some("false".into()),
            payload_oscillation_on: Some("true".into()),
            percentage_command_topic: Some("bedroom_fan/speed/percentage".into()),
            percentage_state_topic: Some("bedroom_fan/speed/percentage_state".into()),
            preset_mode_command_topic: Some("bedroom_fan/preset/preset_mode".into()),
            preset_mode_state_topic: Some("bedroom_fan/preset/preset_mode_state".into()),
            preset_modes: vec![
                "auto".into(),
                "smart".into(),
                "whoosh".into(),
                "eco".into(),
                "breeze".into()
            ],
            qos: Some(Qos::AtMostOnce),
            speed_range_max: Some(10),
            speed_range_min: Some(1),
            state_topic: Some("bedroom_fan/on/state".into()),
            ..Default::default()
        }
    );
}

#[test]
fn component() {
    let component = Component::from(full());
    assert_eq!(component.platform(), "fan");
    #[cfg(feature = "json")]
    {
        let json = serde_json::to_value(&component).unwrap();
        assert_eq!(json["platform"], "fan");
        let back: Component = serde_json::from_value(json).unwrap();
        assert_eq!(back, component);
    }
}

#[cfg(feature = "builder")]
#[test]
fn builder_requires_fields() {
    assert!(Fan::builder().build().is_err());
    assert!(Fan::builder().command_topic("t").build().is_ok());
}

#[cfg(feature = "builder")]
#[test]
fn builder_requires_preset_modes_with_command_topic() {
    assert!(
        Fan::builder()
            .command_topic("t")
            .preset_mode_command_topic("p")
            .build()
            .is_err()
    );
    assert!(
        Fan::builder()
            .command_topic("t")
            .preset_mode_command_topic("p")
            .preset_modes(vec!["eco".to_string()])
            .build()
            .is_ok()
    );
    assert!(
        Fan::builder()
            .command_topic("t")
            .preset_modes(vec!["eco".to_string()])
            .build()
            .is_err()
    );
}
