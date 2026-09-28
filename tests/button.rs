mod support;

use hassliebe::{
    Availability, AvailabilityMode, Device, EntityCategory, Origin, Qos,
    button::{Button, ButtonDeviceClass},
};
use pretty_assertions::assert_eq;

fn full() -> Button {
    Button {
        availability: vec![Availability {
            topic: "home/bedroom/switch1/available".into(),
            ..Default::default()
        }],
        availability_mode: Some(AvailabilityMode::Latest),
        command_template: Some("{\"action\": \"{{ value }}\"}".into()),
        command_topic: "home/bedroom/switch1/commands".into(),
        default_entity_id: Some("button.restart_bedroom_switch".into()),
        device: Some(Device {
            identifiers: vec!["bedroom_switch1".into()],
            name: Some("Bedroom switch".into()),
            ..Default::default()
        }),
        device_class: Some(ButtonDeviceClass::Restart),
        enabled_by_default: Some(true),
        encoding: Some("utf-8".into()),
        entity_category: Some(EntityCategory::Config),
        entity_picture: Some("https://example.com/switch.png".into()),
        group: vec!["bedroom_switch2_reboot_btn".into()],
        icon: Some("mdi:restart".into()),
        json_attributes_template: Some("{{ value_json | tojson }}".into()),
        json_attributes_topic: Some("home/bedroom/switch1/attributes".into()),
        message_expiry_interval: Some(30.into()),
        name: Some("Restart".into()),
        origin: Some(Origin {
            name: "switch2mqtt".into(),
            ..Default::default()
        }),
        payload_press: Some("restart".into()),
        qos: Some(Qos::AtLeastOnce),
        retain: Some(false),
        unique_id: Some("bedroom_switch_reboot_btn".into()),
        visible_by_default: Some(true),
    }
}

#[test]
fn full_snapshot() {
    insta::assert_json_snapshot!(full());
}

/// The first configuration example from
/// <https://www.home-assistant.io/integrations/button.mqtt/>.
#[test]
fn minimal_snapshot() {
    insta::assert_json_snapshot!(Button {
        command_topic: "home/bedroom/switch1/reboot".into(),
        ..Default::default()
    });
}

#[test]
fn round_trip() {
    support::assert_round_trip(&full());
}

/// "Full configuration" example from
/// <https://www.home-assistant.io/integrations/button.mqtt/>, as JSON.
#[test]
fn docs_full_example() {
    let button: Button = support::from_fixture("button_restart.json");
    assert_eq!(
        button,
        Button {
            availability: vec![Availability {
                topic: "home/bedroom/switch1/available".into(),
                ..Default::default()
            }],
            command_topic: "home/bedroom/switch1/commands".into(),
            device_class: Some(ButtonDeviceClass::Restart),
            entity_category: Some(EntityCategory::Config),
            name: Some("Restart Bedroom Switch".into()),
            payload_press: Some("restart".into()),
            qos: Some(Qos::AtMostOnce),
            retain: Some(false),
            unique_id: Some("bedroom_switch_reboot_btn".into()),
            ..Default::default()
        }
    );
}

#[cfg(feature = "builder")]
#[test]
fn builder_requires_command_topic() {
    assert!(Button::builder().build().is_err());
    assert!(Button::builder().command_topic("t").build().is_ok());
}
