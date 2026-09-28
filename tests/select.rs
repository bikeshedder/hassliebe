mod support;

use hassliebe::{
    Availability, AvailabilityMode, Device, EntityCategory, Origin, Qos, select::Select,
};
use pretty_assertions::assert_eq;

fn full() -> Select {
    Select {
        availability: vec![Availability {
            topic: "fan/status".into(),
            ..Default::default()
        }],
        availability_mode: Some(AvailabilityMode::Latest),
        command_template: Some("{\"mode\": \"{{ value }}\"}".into()),
        command_topic: "fan/mode/set".into(),
        default_entity_id: Some("select.fan_mode".into()),
        device: Some(Device {
            identifiers: vec!["fan01".into()],
            name: Some("Fan".into()),
            ..Default::default()
        }),
        enabled_by_default: Some(true),
        encoding: Some("utf-8".into()),
        entity_category: Some(EntityCategory::Config),
        entity_picture: Some("https://example.com/fan.png".into()),
        group: vec!["fan02_mode".into()],
        icon: Some("mdi:fan".into()),
        json_attributes_template: Some("{{ value_json | tojson }}".into()),
        json_attributes_topic: Some("fan/attributes".into()),
        message_expiry_interval: Some(60.into()),
        name: Some("Mode".into()),
        optimistic: Some(false),
        options: vec!["low".into(), "medium".into(), "high".into()],
        origin: Some(Origin {
            name: "fan2mqtt".into(),
            ..Default::default()
        }),
        qos: Some(Qos::AtLeastOnce),
        retain: Some(false),
        state_topic: Some("fan/mode".into()),
        unique_id: Some("fan01_mode".into()),
        value_template: Some("{{ value_json.mode }}".into()),
        visible_by_default: Some(true),
    }
}

#[test]
fn full_snapshot() {
    insta::assert_json_snapshot!(full());
}

#[test]
fn minimal_snapshot() {
    insta::assert_json_snapshot!(Select {
        command_topic: "fan/mode/set".into(),
        options: vec!["low".into(), "high".into()],
        ..Default::default()
    });
}

#[test]
fn round_trip() {
    support::assert_round_trip(&full());
}

/// The configuration example from
/// <https://www.home-assistant.io/integrations/select.mqtt/>, as JSON.
#[test]
fn docs_example() {
    let select: Select = support::from_fixture("select_docs.json");
    assert_eq!(
        select,
        Select {
            command_topic: "topic".into(),
            name: Some("Test Select".into()),
            options: vec!["Option 1".into(), "Option 2".into()],
            ..Default::default()
        }
    );
}

#[test]
fn options_are_required() {
    assert!(serde_json::from_str::<Select>(r#"{"command_topic": "t"}"#).is_err());
}

#[cfg(feature = "builder")]
#[test]
fn builder_requires_command_topic_and_options() {
    assert!(Select::builder().command_topic("t").build().is_err());
    assert!(
        Select::builder()
            .command_topic("t")
            .options(vec!["a".to_string()])
            .build()
            .is_ok()
    );
}

#[test]
fn options_accept_single_string() {
    let select: Select =
        serde_json::from_str(r#"{"command_topic": "t", "options": "only"}"#).unwrap();
    assert_eq!(select.options, ["only"]);
}
