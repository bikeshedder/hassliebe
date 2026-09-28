mod support;

use hassliebe::{Component, Device, Qos, tag::Tag};
use pretty_assertions::assert_eq;

fn full() -> Tag {
    Tag {
        device: Some(Device {
            identifiers: vec!["0AFFD2".into()],
            name: Some("Tag reader".into()),
            ..Default::default()
        }),
        encoding: Some("utf-8".into()),
        group: vec![],
        qos: Some(Qos::AtLeastOnce),
        topic: "0AFFD2/tag_scanned".into(),
        value_template: Some("{{ value_json.PN532.UID }}".into()),
    }
}

#[test]
fn full_snapshot() {
    insta::assert_json_snapshot!(full());
}

#[test]
fn minimal_snapshot() {
    insta::assert_json_snapshot!(Tag {
        topic: "0AFFD2/tag_scanned".into(),
        ..Default::default()
    });
}

#[test]
fn round_trip() {
    support::assert_round_trip(&full());
}

/// "Full configuration with tag ID extracted from JSON data" (Tasmota) from
/// <https://www.home-assistant.io/integrations/tag.mqtt/>.
#[test]
fn docs_tasmota_example() {
    let value: Tag = support::from_fixture("tag_tasmota.json");
    assert_eq!(
        value,
        Tag {
            topic: "0AFFD2/tag_scanned".into(),
            value_template: Some("{{ value_json.PN532.UID }}".into()),
            ..Default::default()
        }
    );
}

#[test]
fn component() {
    let component = Component::from(full());
    assert_eq!(component.platform(), "tag");
    #[cfg(feature = "json")]
    {
        let json = serde_json::to_value(&component).unwrap();
        assert_eq!(json["platform"], "tag");
        let back: Component = serde_json::from_value(json).unwrap();
        assert_eq!(back, component);
    }
}

#[cfg(feature = "builder")]
#[test]
fn builder_requires_fields() {
    assert!(Tag::builder().build().is_err());
    assert!(
        Tag::builder()
            .topic("t")
            .device(Device {
                identifiers: vec!["d".into()],
                ..Default::default()
            })
            .build()
            .is_ok()
    );
}

#[cfg(feature = "builder")]
#[test]
fn builder_requires_device() {
    assert!(Tag::builder().topic("t").build().is_err());
}
