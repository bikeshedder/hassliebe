mod support;

use hassliebe::{
    Availability, AvailabilityMode, Component, Device, EntityCategory, Origin, Qos,
    infrared::{InfraredReceiver, InfraredReceiverSchema},
};
use pretty_assertions::assert_eq;

fn full() -> InfraredReceiver {
    InfraredReceiver {
        availability: vec![Availability {
            topic: "living_room/ir/status".into(),
            ..Default::default()
        }],
        availability_mode: Some(AvailabilityMode::Latest),
        default_entity_id: Some("infrared.living_room_ir".into()),
        device: Some(Device {
            identifiers: vec!["living_room_ir_device".into()],
            name: Some("IR receiver".into()),
            ..Default::default()
        }),
        enabled_by_default: Some(true),
        encoding: Some("utf-8".into()),
        entity_category: Some(EntityCategory::Diagnostic),
        entity_picture: Some("https://example.com/living_room_ir.png".into()),
        group: vec!["living_room_ir_2".into()],
        icon: Some("mdi:remote".into()),
        json_attributes_template: Some("{{ value_json | tojson }}".into()),
        json_attributes_topic: Some("living_room/ir/attributes".into()),
        message_expiry_interval: Some(60.into()),
        name: Some("IR receiver".into()),
        origin: Some(Origin {
            name: "living_room_ir2mqtt".into(),
            ..Default::default()
        }),
        qos: Some(Qos::AtLeastOnce),
        retain: None,
        schema: InfraredReceiverSchema::Receiver,
        state_topic: "living_room/ir/received".into(),
        unique_id: Some("living_room_ir".into()),
        value_template: Some("{{ value_json.signal | tojson }}".into()),
        visible_by_default: Some(true),
    }
}

#[test]
fn full_snapshot() {
    insta::assert_json_snapshot!(full());
}

/// The receiver configuration example from
/// <https://www.home-assistant.io/integrations/infrared.mqtt/>.
#[test]
fn minimal_snapshot() {
    insta::assert_json_snapshot!(InfraredReceiver {
        state_topic: "state-topic".into(),
        ..Default::default()
    });
}

#[test]
fn round_trip() {
    support::assert_round_trip(&full());
}

/// "Manual configuration example" of a receiver from
/// <https://www.home-assistant.io/integrations/infrared.mqtt/>, as JSON.
#[test]
fn docs_manual_example() {
    let value: InfraredReceiver = support::from_fixture("infrared_receiver.json");
    assert_eq!(
        value,
        InfraredReceiver {
            icon: Some("mdi:ab-testing".into()),
            name: Some("My Infrared receiver".into()),
            schema: InfraredReceiverSchema::Receiver,
            state_topic: "ir/receive-signal".into(),
            ..Default::default()
        }
    );
}

#[test]
fn component() {
    let component = Component::from(full());
    assert_eq!(component.platform(), "infrared");
    #[cfg(feature = "json")]
    {
        let json = serde_json::to_value(&component).unwrap();
        assert_eq!(json["platform"], "infrared");
        let back: Component = serde_json::from_value(json).unwrap();
        assert_eq!(back, component);
    }
}

#[cfg(feature = "builder")]
#[test]
fn builder_requires_fields() {
    assert!(InfraredReceiver::builder().build().is_err());
    assert!(InfraredReceiver::builder().state_topic("t").build().is_ok());
}
