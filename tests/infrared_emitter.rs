mod support;

use hassliebe::{
    Availability, AvailabilityMode, Component, Device, EntityCategory, Origin, Qos,
    infrared::{InfraredEmitter, InfraredEmitterSchema},
};
use pretty_assertions::assert_eq;

fn full() -> InfraredEmitter {
    InfraredEmitter {
        availability: vec![Availability {
            topic: "living_room/ir/status".into(),
            ..Default::default()
        }],
        availability_mode: Some(AvailabilityMode::Latest),
        command_template: Some(
            "{{ {\"timings\": timings, \"modulation\": modulation} | tojson }}".into(),
        ),
        command_topic: "living_room/ir/send".into(),
        default_entity_id: Some("infrared.living_room_ir".into()),
        device: Some(Device {
            identifiers: vec!["living_room_ir_device".into()],
            name: Some("IR blaster".into()),
            ..Default::default()
        }),
        enabled_by_default: Some(true),
        encoding: Some("utf-8".into()),
        entity_category: Some(EntityCategory::Config),
        entity_picture: Some("https://example.com/living_room_ir.png".into()),
        group: vec!["living_room_ir_2".into()],
        icon: Some("mdi:remote".into()),
        json_attributes_template: Some("{{ value_json | tojson }}".into()),
        json_attributes_topic: Some("living_room/ir/attributes".into()),
        message_expiry_interval: Some(60.into()),
        name: Some("IR blaster".into()),
        origin: Some(Origin {
            name: "living_room_ir2mqtt".into(),
            ..Default::default()
        }),
        qos: Some(Qos::AtLeastOnce),
        retain: Some(false),
        schema: InfraredEmitterSchema::Emitter,
        unique_id: Some("living_room_ir".into()),
        visible_by_default: Some(true),
    }
}

#[test]
fn full_snapshot() {
    insta::assert_json_snapshot!(full());
}

/// The emitter configuration example from
/// <https://www.home-assistant.io/integrations/infrared.mqtt/>.
#[test]
fn minimal_snapshot() {
    insta::assert_json_snapshot!(InfraredEmitter {
        command_topic: "command-topic".into(),
        ..Default::default()
    });
}

#[test]
fn round_trip() {
    support::assert_round_trip(&full());
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
    assert!(InfraredEmitter::builder().build().is_err());
    assert!(
        InfraredEmitter::builder()
            .command_topic("t")
            .build()
            .is_ok()
    );
}

#[cfg(feature = "json")]
#[test]
fn component_requires_schema() {
    let emitter: Component = serde_json::from_str(
        r#"{"platform": "infrared", "schema": "emitter", "command_topic": "t"}"#,
    )
    .unwrap();
    assert!(matches!(emitter, Component::InfraredEmitter(_)));
    let unknown: Component =
        serde_json::from_str(r#"{"platform": "infrared", "command_topic": "t"}"#).unwrap();
    assert!(matches!(unknown, Component::Other { .. }));
}
