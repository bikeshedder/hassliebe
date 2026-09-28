mod support;

use hassliebe::{
    Availability, AvailabilityMode, Component, Device, EntityCategory, Origin, Qos, scene::Scene,
};
use pretty_assertions::assert_eq;

fn full() -> Scene {
    Scene {
        availability: vec![Availability {
            topic: "home/living_room/party_scene/status".into(),
            ..Default::default()
        }],
        availability_mode: Some(AvailabilityMode::Latest),
        command_topic: Some("home/living_room/party_scene/set".into()),
        default_entity_id: Some("scene.home_living_room_party_scene".into()),
        device: Some(Device {
            identifiers: vec!["home_living_room_party_scene_device".into()],
            name: Some("Living Room".into()),
            ..Default::default()
        }),
        enabled_by_default: Some(true),
        encoding: Some("utf-8".into()),
        entity_category: Some(EntityCategory::Config),
        entity_picture: Some("https://example.com/home_living_room_party_scene.png".into()),
        group: vec!["home_living_room_party_scene_2".into()],
        icon: Some("mdi:party-popper".into()),
        json_attributes_template: Some("{{ value_json | tojson }}".into()),
        json_attributes_topic: Some("home/living_room/party_scene/attributes".into()),
        message_expiry_interval: Some(60.into()),
        name: Some("Party Scene".into()),
        origin: Some(Origin {
            name: "home_living_room_party_scene2mqtt".into(),
            ..Default::default()
        }),
        payload_on: Some("ON".into()),
        qos: Some(Qos::AtLeastOnce),
        retain: Some(true),
        unique_id: Some("home_living_room_party_scene".into()),
        visible_by_default: Some(true),
    }
}

#[test]
fn full_snapshot() {
    insta::assert_json_snapshot!(full());
}

#[test]
fn minimal_snapshot() {
    insta::assert_json_snapshot!(Scene {
        command_topic: Some("home/living_room/party_scene/set".into()),
        payload_on: Some("ON".into()),
        ..Default::default()
    });
}

#[test]
fn round_trip() {
    support::assert_round_trip(&full());
}

/// "Full configuration" example from <https://www.home-assistant.io/integrations/scene.mqtt/>, as JSON.
#[test]
fn docs_full_example() {
    let value: Scene = support::from_fixture("scene_party.json");
    assert_eq!(
        value,
        Scene {
            availability: vec![Availability {
                topic: "home/living_room/party_scene/available".into(),
                ..Default::default()
            }],
            command_topic: Some("home/living_room/party_scene/set".into()),
            device: Some(Device {
                identifiers: vec!["livingroom_lights".into()],
                name: Some("Living Room".into()),
                ..Default::default()
            }),
            name: Some("Party Scene".into()),
            payload_on: Some("ON".into()),
            qos: Some(Qos::AtMostOnce),
            retain: Some(true),
            unique_id: Some("living_room_party_scene".into()),
            ..Default::default()
        }
    );
}

#[test]
fn component() {
    let component = Component::from(full());
    assert_eq!(component.platform(), "scene");
    #[cfg(feature = "json")]
    {
        let json = serde_json::to_value(&component).unwrap();
        assert_eq!(json["platform"], "scene");
        let back: Component = serde_json::from_value(json).unwrap();
        assert_eq!(back, component);
    }
}

#[cfg(feature = "builder")]
#[test]
fn builder_requires_fields() {
    assert!(Scene::builder().build().is_err());
    assert!(
        Scene::builder()
            .command_topic("t")
            .payload_on("ON")
            .build()
            .is_ok()
    );
}

#[cfg(feature = "builder")]
#[test]
fn builder_requires_payload_on() {
    assert!(Scene::builder().command_topic("t").build().is_err());
}
