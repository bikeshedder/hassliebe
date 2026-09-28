mod support;

use hassliebe::{
    Availability, AvailabilityMode, Component, Device, EntityCategory, Origin, Qos,
    datetime::DateTime,
};
use pretty_assertions::assert_eq;

fn full() -> DateTime {
    DateTime {
        availability: vec![Availability {
            topic: "scheduler/job/status".into(),
            ..Default::default()
        }],
        availability_mode: Some(AvailabilityMode::Latest),
        command_template: Some("{\"at\": \"{{ value }}\"}".into()),
        command_topic: "scheduler/job/at/set".into(),
        default_entity_id: Some("datetime.scheduler_job".into()),
        device: Some(Device {
            identifiers: vec!["scheduler_job_device".into()],
            name: Some("Scheduler".into()),
            ..Default::default()
        }),
        enabled_by_default: Some(true),
        encoding: Some("utf-8".into()),
        entity_category: Some(EntityCategory::Config),
        entity_picture: Some("https://example.com/scheduler_job.png".into()),
        group: vec!["scheduler_job_2".into()],
        icon: Some("mdi:calendar-clock".into()),
        json_attributes_template: Some("{{ value_json | tojson }}".into()),
        json_attributes_topic: Some("scheduler/job/attributes".into()),
        message_expiry_interval: Some(60.into()),
        name: Some("Next run".into()),
        optimistic: Some(false),
        origin: Some(Origin {
            name: "scheduler_job2mqtt".into(),
            ..Default::default()
        }),
        qos: Some(Qos::AtLeastOnce),
        retain: Some(true),
        state_topic: Some("scheduler/job/at".into()),
        timezone: Some("Europe/Berlin".into()),
        unique_id: Some("scheduler_job".into()),
        value_template: Some("{{ value_json.at }}".into()),
        visible_by_default: Some(true),
    }
}

#[test]
fn full_snapshot() {
    insta::assert_json_snapshot!(full());
}

/// The configuration example from
/// <https://www.home-assistant.io/integrations/datetime.mqtt/>.
#[test]
fn minimal_snapshot() {
    insta::assert_json_snapshot!(DateTime {
        command_topic: "command-topic".into(),
        ..Default::default()
    });
}

#[test]
fn round_trip() {
    support::assert_round_trip(&full());
}

/// "Examples" section of <https://www.home-assistant.io/integrations/datetime.mqtt/>, as JSON.
#[test]
fn docs_example() {
    let value: DateTime = support::from_fixture("datetime_scheduled_task.json");
    assert_eq!(
        value,
        DateTime {
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
    assert_eq!(component.platform(), "datetime");
    #[cfg(feature = "json")]
    {
        let json = serde_json::to_value(&component).unwrap();
        assert_eq!(json["platform"], "datetime");
        let back: Component = serde_json::from_value(json).unwrap();
        assert_eq!(back, component);
    }
}

#[cfg(feature = "builder")]
#[test]
fn builder_requires_fields() {
    assert!(DateTime::builder().build().is_err());
    assert!(DateTime::builder().command_topic("t").build().is_ok());
}
