mod support;

use hassliebe::{
    Availability, AvailabilityMode, Device, EntityCategory, Origin, Qos, TimePeriod,
    switch::{Switch, SwitchDeviceClass},
};
use pretty_assertions::assert_eq;

fn full() -> Switch {
    Switch {
        availability: vec![Availability {
            topic: "garden/pump/status".into(),
            value_template: Some("{{ value_json.status }}".into()),
            ..Default::default()
        }],
        availability_mode: Some(AvailabilityMode::All),
        command_template: Some("{\"state\": \"{{ value }}\"}".into()),
        command_topic: "garden/pump/set".into(),
        default_entity_id: Some("switch.garden_pump".into()),
        device: Some(Device {
            identifiers: vec!["garden_pump".into()],
            name: Some("Garden pump".into()),
            ..Default::default()
        }),
        device_class: Some(SwitchDeviceClass::Outlet),
        enabled_by_default: Some(true),
        encoding: Some("utf-8".into()),
        entity_category: Some(EntityCategory::Config),
        entity_picture: Some("https://example.com/pump.png".into()),
        group: vec!["garden_valve".into()],
        icon: Some("mdi:water-pump".into()),
        json_attributes_template: Some("{{ value_json | tojson }}".into()),
        json_attributes_topic: Some("garden/pump/attributes".into()),
        message_expiry_interval: Some(
            TimePeriod {
                minutes: Some(5.0),
                ..Default::default()
            }
            .into(),
        ),
        name: Some("Pump".into()),
        optimistic: Some(false),
        origin: Some(Origin {
            name: "garden2mqtt".into(),
            support_url: Some("https://example.com/garden2mqtt".into()),
            ..Default::default()
        }),
        payload_off: Some("0".into()),
        payload_on: Some("1".into()),
        qos: Some(Qos::ExactlyOnce),
        retain: Some(true),
        state_off: Some("off".into()),
        state_on: Some("on".into()),
        state_topic: Some("garden/pump/state".into()),
        unique_id: Some("garden_pump".into()),
        value_template: Some("{{ value_json.state }}".into()),
        visible_by_default: Some(true),
    }
}

#[test]
fn full_snapshot() {
    insta::assert_json_snapshot!(full());
}

#[test]
fn minimal_snapshot() {
    insta::assert_json_snapshot!(Switch {
        command_topic: "garden/pump/set".into(),
        ..Default::default()
    });
}

#[test]
fn round_trip() {
    support::assert_round_trip(&full());
}

/// "Entities with command topics" example from
/// <https://www.home-assistant.io/integrations/mqtt/>.
#[test]
fn docs_irrigation_example() {
    let switch: Switch = support::from_fixture("switch_irrigation.json");
    assert_eq!(
        switch,
        Switch {
            command_topic: "homeassistant/switch/irrigation/set".into(),
            device: Some(Device {
                identifiers: vec!["garden01ad".into()],
                name: Some("Garden".into()),
                ..Default::default()
            }),
            name: Some("Irrigation".into()),
            state_topic: Some("homeassistant/switch/irrigation/state".into()),
            unique_id: Some("irr01ad".into()),
            ..Default::default()
        }
    );
}

#[cfg(feature = "builder")]
#[test]
fn builder_requires_command_topic() {
    assert!(Switch::builder().state_topic("t").build().is_err());
    assert!(Switch::builder().command_topic("t").build().is_ok());
}
