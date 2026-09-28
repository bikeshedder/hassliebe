mod support;

use hassliebe::{
    Availability, AvailabilityMode, Component, Device, EntityCategory, Origin, Qos,
    siren::{Siren, SirenTone},
};
use pretty_assertions::assert_eq;

fn full() -> Siren {
    Siren {
        availability: vec![Availability {
            topic: "home/alarm/siren1/status".into(),
            ..Default::default()
        }],
        availability_mode: Some(AvailabilityMode::Latest),
        available_tones: vec!["ping".into(), "siren".into(), SirenTone::Number(3)],
        command_off_template: Some("{\"state\": \"OFF\"}".into()),
        command_template: Some("{\"state\": \"{{ value }}\", \"tone\": \"{{ tone }}\"}".into()),
        command_topic: Some("home/alarm/siren1/set".into()),
        default_entity_id: Some("siren.home_alarm_siren1".into()),
        device: Some(Device {
            identifiers: vec!["home_alarm_siren1_device".into()],
            name: Some("Alarm".into()),
            ..Default::default()
        }),
        enabled_by_default: Some(true),
        encoding: Some("utf-8".into()),
        entity_category: Some(EntityCategory::Config),
        entity_picture: Some("https://example.com/home_alarm_siren1.png".into()),
        group: vec!["home_alarm_siren1_2".into()],
        icon: Some("mdi:alarm-light".into()),
        json_attributes_template: Some("{{ value_json | tojson }}".into()),
        json_attributes_topic: Some("home/alarm/siren1/attributes".into()),
        message_expiry_interval: Some(60.into()),
        name: Some("Intrusion siren".into()),
        optimistic: Some(false),
        origin: Some(Origin {
            name: "home_alarm_siren12mqtt".into(),
            ..Default::default()
        }),
        payload_off: Some("OFF".into()),
        payload_on: Some("ON".into()),
        qos: Some(Qos::AtLeastOnce),
        retain: Some(true),
        state_off: Some("OFF".into()),
        state_on: Some("ON".into()),
        state_topic: Some("home/alarm/siren1".into()),
        state_value_template: Some("{{ value_json.state }}".into()),
        support_duration: Some(true),
        support_volume_set: Some(false),
        unique_id: Some("home_alarm_siren1".into()),
        visible_by_default: Some(true),
    }
}

#[test]
fn full_snapshot() {
    insta::assert_json_snapshot!(full());
}

/// The configuration example from
/// <https://www.home-assistant.io/integrations/siren.mqtt/>.
#[test]
fn minimal_snapshot() {
    insta::assert_json_snapshot!(Siren {
        command_topic: Some("home/bedroom/siren/set".into()),
        ..Default::default()
    });
}

#[test]
fn round_trip() {
    support::assert_round_trip(&full());
}

/// "Full configuration" example from <https://www.home-assistant.io/integrations/siren.mqtt/>, as JSON.
#[test]
fn docs_full_example() {
    let value: Siren = support::from_fixture("siren_intrusion.json");
    assert_eq!(
        value,
        Siren {
            availability: vec![Availability {
                topic: "home/alarm/siren1/available".into(),
                ..Default::default()
            }],
            available_tones: vec!["ping".into(), "siren".into()],
            command_topic: Some("home/alarm/siren1/set".into()),
            name: Some("Intrusion siren".into()),
            optimistic: Some(false),
            payload_off: Some("OFF".into()),
            payload_on: Some("ON".into()),
            qos: Some(Qos::AtMostOnce),
            retain: Some(true),
            state_off: Some("OFF".into()),
            state_on: Some("ON".into()),
            state_topic: Some("home/alarm/siren1".into()),
            unique_id: Some("custom_siren".into()),
            ..Default::default()
        }
    );
}

#[test]
fn component() {
    let component = Component::from(full());
    assert_eq!(component.platform(), "siren");
    #[cfg(feature = "json")]
    {
        let json = serde_json::to_value(&component).unwrap();
        assert_eq!(json["platform"], "siren");
        let back: Component = serde_json::from_value(json).unwrap();
        assert_eq!(back, component);
    }
}

#[cfg(feature = "builder")]
#[test]
fn builder_requires_fields() {
    assert!(Siren::builder().build().is_err());
    assert!(Siren::builder().command_topic("t").build().is_ok());
}

#[test]
fn tones_are_names_or_numbers() {
    let siren: Siren = serde_json::from_str(r#"{"available_tones": ["ping", 2]}"#).unwrap();
    assert_eq!(
        siren.available_tones,
        [SirenTone::from("ping"), SirenTone::Number(2)]
    );
    let siren: Siren = serde_json::from_str(r#"{"available_tones": "ping"}"#).unwrap();
    assert_eq!(siren.available_tones, [SirenTone::from("ping")]);
}
