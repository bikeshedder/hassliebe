mod support;

use hassliebe::{
    Availability, AvailabilityMode, Component, Device, EntityCategory, Origin, Qos,
    TemperatureUnit,
    water_heater::{WaterHeater, WaterHeaterMode},
};
use pretty_assertions::assert_eq;

fn full() -> WaterHeater {
    WaterHeater {
        availability: vec![Availability {
            topic: "basement/boiler/status".into(),
            ..Default::default()
        }],
        availability_mode: Some(AvailabilityMode::Latest),
        current_temperature_template: Some("{{ value_json.current }}".into()),
        current_temperature_topic: Some("basement/boiler/current_temperature".into()),
        default_entity_id: Some("water_heater.basement_boiler".into()),
        device: Some(Device {
            identifiers: vec!["basement_boiler_device".into()],
            name: Some("Boiler".into()),
            ..Default::default()
        }),
        enabled_by_default: Some(true),
        encoding: Some("utf-8".into()),
        entity_category: Some(EntityCategory::Config),
        entity_picture: Some("https://example.com/basement_boiler.png".into()),
        group: vec!["basement_boiler_2".into()],
        icon: Some("mdi:water-boiler".into()),
        initial: Some(45.0),
        json_attributes_template: Some("{{ value_json | tojson }}".into()),
        json_attributes_topic: Some("basement/boiler/attributes".into()),
        max_temp: Some(60.0),
        message_expiry_interval: Some(60.into()),
        min_temp: Some(40.0),
        mode_command_template: Some("{{ value }}".into()),
        mode_command_topic: Some("basement/boiler/mode/set".into()),
        mode_state_template: Some("{{ value_json.mode }}".into()),
        mode_state_topic: Some("basement/boiler/mode".into()),
        modes: Some(vec![
            WaterHeaterMode::Off,
            WaterHeaterMode::Eco,
            WaterHeaterMode::Performance,
        ]),
        name: Some("Boiler".into()),
        optimistic: Some(false),
        origin: Some(Origin {
            name: "basement_boiler2mqtt".into(),
            ..Default::default()
        }),
        payload_off: Some("OFF".into()),
        payload_on: Some("ON".into()),
        power_command_template: Some("{{ value }}".into()),
        power_command_topic: Some("basement/boiler/power/set".into()),
        precision: Some(0.5),
        qos: Some(Qos::AtLeastOnce),
        retain: Some(false),
        temperature_command_template: Some("{{ value }}".into()),
        temperature_command_topic: Some("basement/boiler/temperature/set".into()),
        temperature_state_template: Some("{{ value_json.target }}".into()),
        temperature_state_topic: Some("basement/boiler/temperature".into()),
        temperature_unit: Some(TemperatureUnit::Celsius),
        unique_id: Some("basement_boiler".into()),
        value_template: Some("{{ value_json.value }}".into()),
        visible_by_default: Some(true),
    }
}

#[test]
fn full_snapshot() {
    insta::assert_json_snapshot!(full());
}

/// The configuration example from
/// <https://www.home-assistant.io/integrations/water_heater.mqtt/>.
#[test]
fn minimal_snapshot() {
    insta::assert_json_snapshot!(WaterHeater {
        mode_command_topic: Some("basement/boiler/mode/set".into()),
        name: Some("Boiler".into()),
        ..Default::default()
    });
}

#[test]
fn round_trip() {
    support::assert_round_trip(&full());
}

/// "Using templates" example from <https://www.home-assistant.io/integrations/water_heater.mqtt/>, as JSON.
#[test]
fn docs_templates_example() {
    let value: WaterHeater = support::from_fixture("water_heater_boiler.json");
    assert_eq!(
        value,
        WaterHeater {
            mode_command_topic: Some("basement/boiler/mode/set".into()),
            mode_state_template: Some("{{ value_json }}".into()),
            mode_state_topic: Some("basement/boiler/mode/state".into()),
            modes: Some(vec![
                WaterHeaterMode::Off,
                WaterHeaterMode::Eco,
                WaterHeaterMode::Performance
            ]),
            name: Some("Boiler".into()),
            ..Default::default()
        }
    );
}

#[test]
fn component() {
    let component = Component::from(full());
    assert_eq!(component.platform(), "water_heater");
    #[cfg(feature = "json")]
    {
        let json = serde_json::to_value(&component).unwrap();
        assert_eq!(json["platform"], "water_heater");
        let back: Component = serde_json::from_value(json).unwrap();
        assert_eq!(back, component);
    }
}

#[cfg(feature = "builder")]
#[test]
fn builder_has_no_required_fields() {
    assert!(WaterHeater::builder().build().is_ok());
}
