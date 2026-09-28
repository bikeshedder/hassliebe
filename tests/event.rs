mod support;

use hassliebe::{
    Availability, AvailabilityMode, Component, Device, EntityCategory, Origin, Qos,
    event::{Event, EventDeviceClass},
};
use pretty_assertions::assert_eq;

fn full() -> Event {
    Event {
        availability: vec![Availability {
            topic: "home/doorbell/status".into(),
            ..Default::default()
        }],
        availability_mode: Some(AvailabilityMode::Latest),
        default_entity_id: Some("event.home_doorbell".into()),
        device: Some(Device {
            identifiers: vec!["home_doorbell_device".into()],
            name: Some("Front door".into()),
            ..Default::default()
        }),
        device_class: Some(EventDeviceClass::Doorbell),
        enabled_by_default: Some(true),
        encoding: Some("utf-8".into()),
        entity_category: Some(EntityCategory::Diagnostic),
        entity_picture: Some("https://example.com/home_doorbell.png".into()),
        event_types: vec!["press".into(), "hold".into()],
        group: vec!["home_doorbell_2".into()],
        icon: Some("mdi:doorbell".into()),
        json_attributes_template: Some("{{ value_json | tojson }}".into()),
        json_attributes_topic: Some("home/doorbell/attributes".into()),
        message_expiry_interval: Some(60.into()),
        name: Some("Doorbell".into()),
        origin: Some(Origin {
            name: "home_doorbell2mqtt".into(),
            ..Default::default()
        }),
        qos: Some(Qos::AtLeastOnce),
        state_topic: "home/doorbell/state".into(),
        unique_id: Some("home_doorbell".into()),
        value_template: Some("{{ value_json | tojson }}".into()),
        visible_by_default: Some(true),
    }
}

#[test]
fn full_snapshot() {
    insta::assert_json_snapshot!(full());
}

/// The configuration example from
/// <https://www.home-assistant.io/integrations/event.mqtt/>.
#[test]
fn minimal_snapshot() {
    insta::assert_json_snapshot!(Event {
        event_types: vec!["press".into()],
        state_topic: "home/doorbell/state".into(),
        ..Default::default()
    });
}

#[test]
fn round_trip() {
    support::assert_round_trip(&full());
}

/// "Full configuration with JSON data" example from
/// <https://www.home-assistant.io/integrations/event.mqtt/>, as JSON.
#[test]
fn docs_full_example() {
    let value: Event = support::from_fixture("event_doorbell.json");
    assert_eq!(
        value,
        Event {
            availability: vec![Availability {
                topic: "home/doorbell/available".into(),
                ..Default::default()
            }],
            device_class: Some(EventDeviceClass::Doorbell),
            event_types: vec!["press".into(), "hold".into()],
            qos: Some(Qos::AtMostOnce),
            state_topic: "home/doorbell/state".into(),
            ..Default::default()
        }
    );
}

#[test]
fn component() {
    let component = Component::from(full());
    assert_eq!(component.platform(), "event");
    #[cfg(feature = "json")]
    {
        let json = serde_json::to_value(&component).unwrap();
        assert_eq!(json["platform"], "event");
        let back: Component = serde_json::from_value(json).unwrap();
        assert_eq!(back, component);
    }
}

#[cfg(feature = "builder")]
#[test]
fn builder_requires_fields() {
    assert!(Event::builder().build().is_err());
    assert!(
        Event::builder()
            .state_topic("t")
            .event_types(vec!["press".to_string()])
            .build()
            .is_ok()
    );
}

#[test]
fn event_types_accepts_single_string() {
    let event: Event =
        serde_json::from_str(r#"{"state_topic": "t", "event_types": "press"}"#).unwrap();
    assert_eq!(event.event_types, ["press"]);
    assert!(serde_json::from_str::<Event>(r#"{"state_topic": "t"}"#).is_err());
}
