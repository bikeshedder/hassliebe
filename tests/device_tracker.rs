mod support;

use hassliebe::{
    Availability, AvailabilityMode, Component, Device, EntityCategory, Origin, Qos,
    device_tracker::{DeviceTracker, SourceType},
};
use pretty_assertions::assert_eq;

fn full() -> DeviceTracker {
    DeviceTracker {
        availability: vec![Availability {
            topic: "location/paulus/status".into(),
            ..Default::default()
        }],
        availability_mode: Some(AvailabilityMode::Latest),
        default_entity_id: Some("device_tracker.location_paulus".into()),
        device: Some(Device {
            identifiers: vec!["location_paulus_device".into()],
            name: Some("Paulus' phone".into()),
            ..Default::default()
        }),
        enabled_by_default: Some(true),
        encoding: Some("utf-8".into()),
        entity_category: Some(EntityCategory::Diagnostic),
        entity_picture: Some("https://example.com/location_paulus.png".into()),
        group: vec!["location_paulus_2".into()],
        icon: Some("mdi:cellphone".into()),
        json_attributes_template: Some("{{ value_json | tojson }}".into()),
        json_attributes_topic: Some("location/paulus/attributes".into()),
        message_expiry_interval: Some(60.into()),
        name: Some("Paulus".into()),
        origin: Some(Origin {
            name: "location_paulus2mqtt".into(),
            ..Default::default()
        }),
        payload_home: Some("home".into()),
        payload_not_home: Some("not_home".into()),
        payload_reset: Some("None".into()),
        qos: Some(Qos::AtLeastOnce),
        source_type: Some(SourceType::Router),
        state_topic: Some("location/paulus".into()),
        unique_id: Some("location_paulus".into()),
        value_template: Some("{{ value_json.location }}".into()),
        visible_by_default: Some(true),
    }
}

#[test]
fn full_snapshot() {
    insta::assert_json_snapshot!(full());
}

#[test]
fn minimal_snapshot() {
    insta::assert_json_snapshot!(DeviceTracker {
        state_topic: Some("location/paulus".into()),
        ..Default::default()
    });
}

#[test]
fn round_trip() {
    support::assert_round_trip(&full());
}

/// "Using the discovery protocol" example from
/// <https://www.home-assistant.io/integrations/device_tracker.mqtt/>.
#[test]
fn docs_discovery_example() {
    let value: DeviceTracker = support::from_fixture("device_tracker_discovery.json");
    assert_eq!(
        value,
        DeviceTracker {
            name: Some("My Tracker".into()),
            payload_home: Some("home".into()),
            payload_not_home: Some("not_home".into()),
            state_topic: Some("homeassistant/device_tracker/a4567d663eaf/state".into()),
            ..Default::default()
        }
    );
}

#[test]
fn component() {
    let component = Component::from(full());
    assert_eq!(component.platform(), "device_tracker");
    #[cfg(feature = "json")]
    {
        let json = serde_json::to_value(&component).unwrap();
        assert_eq!(json["platform"], "device_tracker");
        let back: Component = serde_json::from_value(json).unwrap();
        assert_eq!(back, component);
    }
}

#[cfg(feature = "builder")]
#[test]
fn builder_requires_fields() {
    assert!(DeviceTracker::builder().build().is_err());
    assert!(DeviceTracker::builder().state_topic("t").build().is_ok());
}

/// The GPS variant of "Using the discovery protocol" from
/// <https://www.home-assistant.io/integrations/device_tracker.mqtt/>.
#[test]
fn docs_gps_discovery_example() {
    let tracker: DeviceTracker = serde_json::from_str(
        r#"{"json_attributes_topic": "homeassistant/device_tracker/a4567d663eaf/attributes", "name": "My Tracker"}"#,
    )
    .unwrap();
    assert_eq!(tracker.state_topic, None);
    assert_eq!(
        tracker.json_attributes_topic.as_deref(),
        Some("homeassistant/device_tracker/a4567d663eaf/attributes")
    );
}

#[cfg(feature = "builder")]
#[test]
fn builder_accepts_json_attributes_topic_instead() {
    assert!(
        DeviceTracker::builder()
            .json_attributes_topic("t")
            .build()
            .is_ok()
    );
}
