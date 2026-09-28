mod support;

use hassliebe::{
    Availability, AvailabilityMode, Component, Device, EntityCategory, Origin, Qos, time::Time,
};
use pretty_assertions::assert_eq;

fn full() -> Time {
    Time {
        availability: vec![Availability {
            topic: "alarm/clock/status".into(),
            ..Default::default()
        }],
        availability_mode: Some(AvailabilityMode::Latest),
        command_template: Some("{\"time\": \"{{ value }}\"}".into()),
        command_topic: "alarm/clock/time/set".into(),
        default_entity_id: Some("time.alarm_clock".into()),
        device: Some(Device {
            identifiers: vec!["alarm_clock_device".into()],
            name: Some("Alarm clock".into()),
            ..Default::default()
        }),
        enabled_by_default: Some(true),
        encoding: Some("utf-8".into()),
        entity_category: Some(EntityCategory::Config),
        entity_picture: Some("https://example.com/alarm_clock.png".into()),
        group: vec!["alarm_clock_2".into()],
        icon: Some("mdi:clock-outline".into()),
        json_attributes_template: Some("{{ value_json | tojson }}".into()),
        json_attributes_topic: Some("alarm/clock/attributes".into()),
        message_expiry_interval: Some(60.into()),
        name: Some("Wake-up time".into()),
        optimistic: Some(false),
        origin: Some(Origin {
            name: "alarm_clock2mqtt".into(),
            ..Default::default()
        }),
        qos: Some(Qos::AtLeastOnce),
        retain: Some(true),
        state_topic: Some("alarm/clock/time".into()),
        unique_id: Some("alarm_clock".into()),
        value_template: Some("{{ value_json.time }}".into()),
        visible_by_default: Some(true),
    }
}

#[test]
fn full_snapshot() {
    insta::assert_json_snapshot!(full());
}

/// The configuration example from
/// <https://www.home-assistant.io/integrations/time.mqtt/>.
#[test]
fn minimal_snapshot() {
    insta::assert_json_snapshot!(Time {
        command_topic: "command-topic".into(),
        ..Default::default()
    });
}

#[test]
fn round_trip() {
    support::assert_round_trip(&full());
}

/// "Examples" section of <https://www.home-assistant.io/integrations/time.mqtt/>, as JSON.
#[test]
fn docs_example() {
    let value: Time = support::from_fixture("time_timer.json");
    assert_eq!(
        value,
        Time {
            command_topic: "timer/start".into(),
            icon: Some("mdi:ab-testing".into()),
            name: Some("Scheduled task".into()),
            state_topic: Some("timer/start_state".into()),
            ..Default::default()
        }
    );
}

#[test]
fn component() {
    let component = Component::from(full());
    assert_eq!(component.platform(), "time");
    #[cfg(feature = "json")]
    {
        let json = serde_json::to_value(&component).unwrap();
        assert_eq!(json["platform"], "time");
        let back: Component = serde_json::from_value(json).unwrap();
        assert_eq!(back, component);
    }
}

#[cfg(feature = "builder")]
#[test]
fn builder_requires_fields() {
    assert!(Time::builder().build().is_err());
    assert!(Time::builder().command_topic("t").build().is_ok());
}
