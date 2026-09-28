mod support;

use hassliebe::{
    Availability, AvailabilityMode, Component, Device, EntityCategory, Origin, Qos,
    image::{Image, ImageEncoding},
};
use pretty_assertions::assert_eq;

fn full() -> Image {
    Image {
        availability: vec![Availability {
            topic: "mynas/status/status".into(),
            ..Default::default()
        }],
        availability_mode: Some(AvailabilityMode::Latest),
        content_type: Some("image/png".into()),
        default_entity_id: Some("image.mynas_status".into()),
        device: Some(Device {
            identifiers: vec!["mynas_status_device".into()],
            name: Some("NAS".into()),
            ..Default::default()
        }),
        enabled_by_default: Some(true),
        encoding: Some("utf-8".into()),
        entity_category: Some(EntityCategory::Diagnostic),
        entity_picture: Some("https://example.com/mynas_status.png".into()),
        group: vec!["mynas_status_2".into()],
        icon: Some("mdi:image".into()),
        image_encoding: Some(ImageEncoding::B64),
        image_topic: Some("mynas/status/file".into()),
        json_attributes_template: Some("{{ value_json | tojson }}".into()),
        json_attributes_topic: Some("mynas/status/attributes".into()),
        message_expiry_interval: Some(60.into()),
        name: Some("Logo".into()),
        origin: Some(Origin {
            name: "mynas_status2mqtt".into(),
            ..Default::default()
        }),
        qos: Some(Qos::AtLeastOnce),
        unique_id: Some("mynas_status".into()),
        url_template: None,
        url_topic: None,
        visible_by_default: Some(true),
    }
}

#[test]
fn full_snapshot() {
    insta::assert_json_snapshot!(full());
}

/// The configuration example from
/// <https://www.home-assistant.io/integrations/image.mqtt/>.
#[test]
fn minimal_snapshot() {
    insta::assert_json_snapshot!(Image {
        url_topic: Some("mynas/status/url".into()),
        ..Default::default()
    });
}

#[test]
fn round_trip() {
    support::assert_round_trip(&full());
}

/// "Example receiving images from a file" from
/// <https://www.home-assistant.io/integrations/image.mqtt/>, as JSON.
#[test]
fn docs_file_example() {
    let value: Image = support::from_fixture("image_file.json");
    assert_eq!(
        value,
        Image {
            content_type: Some("image/png".into()),
            image_topic: Some("mynas/status/file".into()),
            ..Default::default()
        }
    );
}

#[test]
fn component() {
    let component = Component::from(full());
    assert_eq!(component.platform(), "image");
    #[cfg(feature = "json")]
    {
        let json = serde_json::to_value(&component).unwrap();
        assert_eq!(json["platform"], "image");
        let back: Component = serde_json::from_value(json).unwrap();
        assert_eq!(back, component);
    }
}

#[cfg(feature = "builder")]
#[test]
fn builder_requires_fields() {
    assert!(Image::builder().build().is_err());
    assert!(
        Image::builder()
            .url_topic("mynas/status/url")
            .build()
            .is_ok()
    );
}

/// A URL-based image, the other of the two exclusive forms.
#[test]
fn url_round_trip() {
    support::assert_round_trip(&Image {
        url_template: Some("{{ value_json.url }}".into()),
        url_topic: Some("mynas/status/url".into()),
        ..Default::default()
    });
}

#[cfg(feature = "builder")]
#[test]
fn builder_checks_topics() {
    assert!(
        Image::builder()
            .image_topic("a")
            .url_topic("b")
            .build()
            .is_err()
    );
    assert!(
        Image::builder()
            .url_topic("b")
            .content_type("image/png")
            .build()
            .is_err()
    );
    assert!(
        Image::builder()
            .image_topic("a")
            .content_type("image/png")
            .build()
            .is_ok()
    );
}
