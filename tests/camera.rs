mod support;

use hassliebe::{
    Availability, AvailabilityMode, Component, Device, EntityCategory, Origin, Qos,
    camera::{Camera, CameraImageEncoding},
};
use pretty_assertions::assert_eq;

fn full() -> Camera {
    Camera {
        availability: vec![Availability {
            topic: "home/front_door/camera/status".into(),
            ..Default::default()
        }],
        availability_mode: Some(AvailabilityMode::Latest),
        default_entity_id: Some("camera.home_front_door_camera".into()),
        device: Some(Device {
            identifiers: vec!["home_front_door_camera_device".into()],
            name: Some("Front door camera".into()),
            ..Default::default()
        }),
        enabled_by_default: Some(true),
        encoding: Some("utf-8".into()),
        entity_category: Some(EntityCategory::Diagnostic),
        entity_picture: Some("https://example.com/home_front_door_camera.png".into()),
        group: vec!["home_front_door_camera_2".into()],
        icon: Some("mdi:cctv".into()),
        image_encoding: Some(CameraImageEncoding::B64),
        json_attributes_template: Some("{{ value_json | tojson }}".into()),
        json_attributes_topic: Some("home/front_door/camera/attributes".into()),
        message_expiry_interval: Some(60.into()),
        name: Some("Front door".into()),
        origin: Some(Origin {
            name: "home_front_door_camera2mqtt".into(),
            ..Default::default()
        }),
        qos: Some(Qos::AtLeastOnce),
        topic: "home/front_door/camera/image".into(),
        unique_id: Some("home_front_door_camera".into()),
        visible_by_default: Some(true),
    }
}

#[test]
fn full_snapshot() {
    insta::assert_json_snapshot!(full());
}

#[test]
fn minimal_snapshot() {
    insta::assert_json_snapshot!(Camera {
        topic: "home/front_door/camera/image".into(),
        ..Default::default()
    });
}

#[test]
fn round_trip() {
    support::assert_round_trip(&full());
}

/// The configuration example from
/// <https://www.home-assistant.io/integrations/camera.mqtt/>, as JSON.
#[test]
fn docs_example() {
    let value: Camera = support::from_fixture("camera_zanzito.json");
    assert_eq!(
        value,
        Camera {
            topic: "zanzito/shared_locations/my-device".into(),
            ..Default::default()
        }
    );
}

#[test]
fn component() {
    let component = Component::from(full());
    assert_eq!(component.platform(), "camera");
    #[cfg(feature = "json")]
    {
        let json = serde_json::to_value(&component).unwrap();
        assert_eq!(json["platform"], "camera");
        let back: Component = serde_json::from_value(json).unwrap();
        assert_eq!(back, component);
    }
}

#[cfg(feature = "builder")]
#[test]
fn builder_requires_fields() {
    assert!(Camera::builder().build().is_err());
    assert!(Camera::builder().topic("t").build().is_ok());
}
