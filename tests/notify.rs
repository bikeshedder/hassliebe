mod support;

use hassliebe::{
    Availability, AvailabilityMode, Component, Device, EntityCategory, Origin, Qos, notify::Notify,
};
use pretty_assertions::assert_eq;

fn full() -> Notify {
    Notify {
        availability: vec![Availability {
            topic: "home/living_room/status_screen/status".into(),
            ..Default::default()
        }],
        availability_mode: Some(AvailabilityMode::Latest),
        command_template: Some("{\"text\": \"{{ value }}\"}".into()),
        command_topic: Some("home/living_room/status_screen/notifications".into()),
        default_entity_id: Some("notify.home_living_room_status_screen".into()),
        device: Some(Device {
            identifiers: vec!["home_living_room_status_screen_device".into()],
            name: Some("Living room".into()),
            ..Default::default()
        }),
        enabled_by_default: Some(true),
        encoding: Some("utf-8".into()),
        entity_category: Some(EntityCategory::Config),
        entity_picture: Some("https://example.com/home_living_room_status_screen.png".into()),
        group: vec!["home_living_room_status_screen_2".into()],
        icon: Some("mdi:message-text".into()),
        json_attributes_template: Some("{{ value_json | tojson }}".into()),
        json_attributes_topic: Some("home/living_room/status_screen/attributes".into()),
        message_expiry_interval: Some(60.into()),
        name: Some("Status screen".into()),
        origin: Some(Origin {
            name: "home_living_room_status_screen2mqtt".into(),
            ..Default::default()
        }),
        qos: Some(Qos::AtLeastOnce),
        retain: Some(false),
        unique_id: Some("home_living_room_status_screen".into()),
        visible_by_default: Some(true),
    }
}

#[test]
fn full_snapshot() {
    insta::assert_json_snapshot!(full());
}

/// The configuration example from
/// <https://www.home-assistant.io/integrations/notify.mqtt/>.
#[test]
fn minimal_snapshot() {
    insta::assert_json_snapshot!(Notify {
        command_topic: Some("home/living_room/status_screen/notifications".into()),
        ..Default::default()
    });
}

#[test]
fn round_trip() {
    support::assert_round_trip(&full());
}

/// "Full configuration" example from <https://www.home-assistant.io/integrations/notify.mqtt/>, as JSON.
#[test]
fn docs_full_example() {
    let value: Notify = support::from_fixture("notify_status_screen.json");
    assert_eq!(
        value,
        Notify {
            availability: vec![Availability {
                topic: "home/living_room/status_screen/available".into(),
                ..Default::default()
            }],
            command_topic: Some("home/living_room/status_screen/notifications".into()),
            name: Some("Living room status screen".into()),
            qos: Some(Qos::AtMostOnce),
            retain: Some(false),
            unique_id: Some("living_room_stat_scr01".into()),
            ..Default::default()
        }
    );
}

#[test]
fn component() {
    let component = Component::from(full());
    assert_eq!(component.platform(), "notify");
    #[cfg(feature = "json")]
    {
        let json = serde_json::to_value(&component).unwrap();
        assert_eq!(json["platform"], "notify");
        let back: Component = serde_json::from_value(json).unwrap();
        assert_eq!(back, component);
    }
}

#[cfg(feature = "builder")]
#[test]
fn builder_requires_fields() {
    assert!(Notify::builder().build().is_err());
    assert!(Notify::builder().command_topic("t").build().is_ok());
}
