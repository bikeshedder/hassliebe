mod support;

use hassliebe::{
    Availability, AvailabilityMode, Component, Device, EntityCategory, Origin, Qos,
    update::{Update, UpdateDeviceClass},
};
use pretty_assertions::assert_eq;

fn full() -> Update {
    Update {
        availability: vec![Availability {
            topic: "shellies/shellyplug-s-112233/status".into(),
            ..Default::default()
        }],
        availability_mode: Some(AvailabilityMode::Latest),
        command_topic: Some("shellies/shellyplug-s-112233/command".into()),
        default_entity_id: Some("update.shellies_shellyplug_s_112233".into()),
        device: Some(Device {
            identifiers: vec!["shellies_shellyplug_s_112233_device".into()],
            name: Some("Shelly Plug S".into()),
            ..Default::default()
        }),
        device_class: Some(UpdateDeviceClass::Firmware),
        display_precision: Some(1),
        enabled_by_default: Some(true),
        encoding: Some("utf-8".into()),
        entity_category: Some(EntityCategory::Config),
        entity_picture: Some("https://example.com/shellies_shellyplug_s_112233.png".into()),
        group: vec!["shellies_shellyplug_s_112233_2".into()],
        icon: Some("mdi:package-up".into()),
        json_attributes_template: Some("{{ value_json | tojson }}".into()),
        json_attributes_topic: Some("shellies/shellyplug-s-112233/attributes".into()),
        latest_version_template: Some("{{ value_json.new_version }}".into()),
        latest_version_topic: Some("shellies/shellyplug-s-112233/latest".into()),
        message_expiry_interval: Some(60.into()),
        name: Some("Firmware".into()),
        origin: Some(Origin {
            name: "shellies_shellyplug_s_1122332mqtt".into(),
            ..Default::default()
        }),
        payload_install: Some("update_fw".into()),
        qos: Some(Qos::AtLeastOnce),
        release_summary: Some("Bug fixes".into()),
        release_url: Some("https://shelly-api-docs.shelly.cloud/gen1/#changelog".into()),
        retain: Some(false),
        state_topic: Some("shellies/shellyplug-s-112233/info".into()),
        title: Some("Shelly Plug S Firmware".into()),
        unique_id: Some("shellies_shellyplug_s_112233".into()),
        value_template: Some("{{ value_json.old_version }}".into()),
        visible_by_default: Some(true),
    }
}

#[test]
fn full_snapshot() {
    insta::assert_json_snapshot!(full());
}

/// The configuration example from
/// <https://www.home-assistant.io/integrations/update.mqtt/>.
#[test]
fn minimal_snapshot() {
    insta::assert_json_snapshot!(Update {
        latest_version_topic: Some("topic-latest".into()),
        state_topic: Some("topic-installed".into()),
        ..Default::default()
    });
}

#[test]
fn round_trip() {
    support::assert_round_trip(&full());
}

/// Shelly Gen1 example from <https://www.home-assistant.io/integrations/update.mqtt/>, as JSON.
#[test]
fn docs_example() {
    let value: Update = support::from_fixture("update_shelly.json");
    assert_eq!(
        value,
        Update {
            command_topic: Some("shellies/shellyplug-s-112233/command".into()),
            device_class: Some(UpdateDeviceClass::Firmware),
            entity_picture: Some("https://brands.home-assistant.io/_/shelly/icon.png".into()),
            latest_version_template: Some("{% if value_json['update'].new_version %}{{ value_json['update'].new_version }}{% else %}{{ value_json['update'].old_version }}{% endif %}".into()),
            latest_version_topic: Some("shellies/shellyplug-s-112233/info".into()),
            name: Some("Shelly Plug S Firmware Update".into()),
            payload_install: Some("update_fw".into()),
            release_url: Some("https://shelly-api-docs.shelly.cloud/gen1/#changelog".into()),
            state_topic: Some("shellies/shellyplug-s-112233/info".into()),
            title: Some("Shelly Plug S Firmware".into()),
            value_template: Some("{{ value_json['update'].old_version }}".into()),
            ..Default::default()
        }
    );
}

#[test]
fn component() {
    let component = Component::from(full());
    assert_eq!(component.platform(), "update");
    #[cfg(feature = "json")]
    {
        let json = serde_json::to_value(&component).unwrap();
        assert_eq!(json["platform"], "update");
        let back: Component = serde_json::from_value(json).unwrap();
        assert_eq!(back, component);
    }
}

#[cfg(feature = "builder")]
#[test]
fn builder_requires_fields() {
    assert!(Update::builder().build().is_err());
    assert!(Update::builder().state_topic("t").build().is_ok());
}
