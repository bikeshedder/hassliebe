mod support;

use hassliebe::{
    Availability, AvailabilityMode, Component, Device, EntityCategory, Origin, Qos, date::Date,
};
use pretty_assertions::assert_eq;

fn full() -> Date {
    Date {
        availability: vec![Availability {
            topic: "scheduler/task/status".into(),
            ..Default::default()
        }],
        availability_mode: Some(AvailabilityMode::Latest),
        command_template: Some("{\"date\": \"{{ value }}\"}".into()),
        command_topic: "scheduler/task/date/set".into(),
        default_entity_id: Some("date.scheduler_task".into()),
        device: Some(Device {
            identifiers: vec!["scheduler_task_device".into()],
            name: Some("Scheduler".into()),
            ..Default::default()
        }),
        enabled_by_default: Some(true),
        encoding: Some("utf-8".into()),
        entity_category: Some(EntityCategory::Config),
        entity_picture: Some("https://example.com/scheduler_task.png".into()),
        group: vec!["scheduler_task_2".into()],
        icon: Some("mdi:calendar".into()),
        json_attributes_template: Some("{{ value_json | tojson }}".into()),
        json_attributes_topic: Some("scheduler/task/attributes".into()),
        message_expiry_interval: Some(60.into()),
        name: Some("Scheduled task".into()),
        optimistic: Some(false),
        origin: Some(Origin {
            name: "scheduler_task2mqtt".into(),
            ..Default::default()
        }),
        qos: Some(Qos::AtLeastOnce),
        retain: Some(true),
        state_topic: Some("scheduler/task/date".into()),
        unique_id: Some("scheduler_task".into()),
        value_template: Some("{{ value_json.date }}".into()),
        visible_by_default: Some(true),
    }
}

#[test]
fn full_snapshot() {
    insta::assert_json_snapshot!(full());
}

/// The configuration example from
/// <https://www.home-assistant.io/integrations/date.mqtt/>.
#[test]
fn minimal_snapshot() {
    insta::assert_json_snapshot!(Date {
        command_topic: "command-topic".into(),
        ..Default::default()
    });
}

#[test]
fn round_trip() {
    support::assert_round_trip(&full());
}

/// "Examples" section of <https://www.home-assistant.io/integrations/date.mqtt/>, as JSON.
#[test]
fn docs_example() {
    let value: Date = support::from_fixture("date_scheduled_task.json");
    assert_eq!(
        value,
        Date {
            command_topic: "date_selector/set".into(),
            icon: Some("mdi:ab-testing".into()),
            name: Some("Scheduled task".into()),
            state_topic: Some("date_selector/state".into()),
            ..Default::default()
        }
    );
}

#[test]
fn component() {
    let component = Component::from(full());
    assert_eq!(component.platform(), "date");
    #[cfg(feature = "json")]
    {
        let json = serde_json::to_value(&component).unwrap();
        assert_eq!(json["platform"], "date");
        let back: Component = serde_json::from_value(json).unwrap();
        assert_eq!(back, component);
    }
}

#[cfg(feature = "builder")]
#[test]
fn builder_requires_fields() {
    assert!(Date::builder().build().is_err());
    assert!(Date::builder().command_topic("t").build().is_ok());
}
