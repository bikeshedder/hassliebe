mod support;

use hassliebe::{
    Availability, AvailabilityMode, Component, Device, EntityCategory, Nullable, Origin, Qos,
    valve::{Valve, ValveDeviceClass},
};
use pretty_assertions::assert_eq;

fn full() -> Valve {
    Valve {
        availability: vec![Availability {
            topic: "garden/water/status".into(),
            ..Default::default()
        }],
        availability_mode: Some(AvailabilityMode::Latest),
        command_template: Some("{\"x\": {{ value }} }".into()),
        command_topic: Some("garden/water/set".into()),
        default_entity_id: Some("valve.garden_water".into()),
        device: Some(Device {
            identifiers: vec!["garden_water_device".into()],
            name: Some("Garden water valve".into()),
            ..Default::default()
        }),
        device_class: Some(ValveDeviceClass::Water),
        enabled_by_default: Some(true),
        encoding: Some("utf-8".into()),
        entity_category: Some(EntityCategory::Config),
        entity_picture: Some("https://example.com/garden_water.png".into()),
        group: vec!["garden_water_2".into()],
        icon: Some("mdi:valve".into()),
        json_attributes_template: Some("{{ value_json | tojson }}".into()),
        json_attributes_topic: Some("garden/water/attributes".into()),
        message_expiry_interval: Some(60.into()),
        name: Some("Water".into()),
        optimistic: Some(false),
        origin: Some(Origin {
            name: "garden_water2mqtt".into(),
            ..Default::default()
        }),
        payload_close: Some("CLOSE".into()),
        payload_open: Some("OPEN".into()),
        payload_stop: Some("STOP".into()),
        position_closed: Some(0),
        position_open: Some(100),
        qos: Some(Qos::AtLeastOnce),
        reports_position: Some(false),
        retain: Some(true),
        state_closed: Some("closed".into()),
        state_closing: Some("closing".into()),
        state_open: Some("open".into()),
        state_opening: Some("opening".into()),
        state_topic: Some("garden/water/state".into()),
        unique_id: Some("garden_water".into()),
        value_template: Some("{{ value_json.x }}".into()),
        visible_by_default: Some(true),
    }
}

#[test]
fn full_snapshot() {
    insta::assert_json_snapshot!(full());
}

/// "Valve controlled by states" from
/// <https://www.home-assistant.io/integrations/valve.mqtt/>.
#[test]
fn minimal_snapshot() {
    insta::assert_json_snapshot!(Valve {
        command_topic: Some("heater/valve/set".into()),
        state_topic: Some("heater/valve/state".into()),
        ..Default::default()
    });
}

#[test]
fn round_trip() {
    support::assert_round_trip(&full());
}

/// "Full configuration for a valve that does not report position" from
/// <https://www.home-assistant.io/integrations/valve.mqtt/>, as JSON. The legacy
/// `payload_available`/`payload_not_available` are not modeled and ignored.
#[test]
fn docs_full_example() {
    let value: Valve = support::from_fixture("valve_heater.json");
    assert_eq!(
        value,
        Valve {
            availability: vec![Availability {
                topic: "heater/valve/availability".into(),
                ..Default::default()
            }],
            command_template: Some("{\"x\": {{ value }} }".into()),
            command_topic: Some("heater/valve/set".into()),
            name: Some("MQTT valve".into()),
            optimistic: Some(false),
            payload_close: Some("CLOSE".into()),
            payload_open: Some("OPEN".into()),
            payload_stop: Some("STOP".into()),
            qos: Some(Qos::AtMostOnce),
            reports_position: Some(false),
            retain: Some(true),
            state_closed: Some("closed".into()),
            state_closing: Some("closing".into()),
            state_open: Some("open".into()),
            state_opening: Some("opening".into()),
            state_topic: Some("heater/valve/state".into()),
            value_template: Some("{{ value_json.x }}".into()),
            ..Default::default()
        }
    );
}

#[test]
fn component() {
    let component = Component::from(full());
    assert_eq!(component.platform(), "valve");
    #[cfg(feature = "json")]
    {
        let json = serde_json::to_value(&component).unwrap();
        assert_eq!(json["platform"], "valve");
        let back: Component = serde_json::from_value(json).unwrap();
        assert_eq!(back, component);
    }
}

#[cfg(feature = "builder")]
#[test]
fn builder_has_no_required_fields() {
    assert!(Valve::builder().build().is_ok());
}

/// "Configuration for disabling valve commands" from
/// <https://www.home-assistant.io/integrations/valve.mqtt/>, as JSON: `null`
/// disables a command.
#[test]
fn null_payload_disables_command() {
    let json = r#"{"payload_close":null,"payload_open":"on","payload_stop":"on"}"#;
    let valve: Valve = serde_json::from_str(json).unwrap();
    assert_eq!(valve.payload_close, Some(Nullable::Null));
    assert_eq!(valve.payload_open, Some("on".into()));
    assert_eq!(serde_json::to_string(&valve).unwrap(), json);
}
