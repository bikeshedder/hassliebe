mod support;

use hassliebe::{
    Availability, AvailabilityMode, Component, Device, EntityCategory, Origin, Qos,
    TemperatureUnit,
    climate::{Climate, HvacMode},
};
use pretty_assertions::assert_eq;

fn full() -> Climate {
    Climate {
        action_template: Some("{{ value_json.action }}".into()),
        action_topic: Some("study/ac/action".into()),
        availability: vec![Availability {
            topic: "study/ac/status".into(),
            ..Default::default()
        }],
        availability_mode: Some(AvailabilityMode::Latest),
        current_humidity_template: Some("{{ value_json.current_humidity }}".into()),
        current_humidity_topic: Some("study/ac/current/humidity".into()),
        current_temperature_template: Some("{{ value_json.current_temperature }}".into()),
        current_temperature_topic: Some("study/ac/current/temperature".into()),
        default_entity_id: Some("climate.study_ac".into()),
        device: Some(Device {
            identifiers: vec!["study_ac_device".into()],
            name: Some("Study AC".into()),
            ..Default::default()
        }),
        enabled_by_default: Some(true),
        encoding: Some("utf-8".into()),
        entity_category: Some(EntityCategory::Config),
        entity_picture: Some("https://example.com/study_ac.png".into()),
        fan_mode_command_template: Some("{{ value }}".into()),
        fan_mode_command_topic: Some("study/ac/fan/mode/command".into()),
        fan_mode_state_template: Some("{{ value_json.fan_mode_state }}".into()),
        fan_mode_state_topic: Some("study/ac/fan/mode/state".into()),
        fan_modes: Some(vec!["auto".into(), "low".into(), "high".into()]),
        group: vec!["study_ac_2".into()],
        icon: Some("mdi:thermostat".into()),
        initial: Some(21.0),
        json_attributes_template: Some("{{ value_json | tojson }}".into()),
        json_attributes_topic: Some("study/ac/attributes".into()),
        max_humidity: Some(70.0),
        max_temp: Some(30.0),
        message_expiry_interval: Some(60.into()),
        min_humidity: Some(30.0),
        min_temp: Some(7.0),
        mode_command_template: Some("{{ value }}".into()),
        mode_command_topic: Some("study/ac/mode/command".into()),
        mode_state_template: Some("{{ value_json.mode_state }}".into()),
        mode_state_topic: Some("study/ac/mode/state".into()),
        modes: Some(vec![
            HvacMode::Off,
            HvacMode::Heat,
            HvacMode::Cool,
            HvacMode::Auto,
        ]),
        name: Some("Study".into()),
        optimistic: Some(false),
        origin: Some(Origin {
            name: "study_ac2mqtt".into(),
            ..Default::default()
        }),
        payload_off: Some("OFF".into()),
        payload_on: Some("ON".into()),
        power_command_template: Some("{{ value }}".into()),
        power_command_topic: Some("study/ac/power/command".into()),
        precision: Some(0.5),
        preset_mode_command_template: Some("{{ value }}".into()),
        preset_mode_command_topic: Some("study/ac/preset/mode/command".into()),
        preset_mode_state_topic: Some("study/ac/preset/mode/state".into()),
        preset_mode_value_template: Some("{{ value_json.preset_mode_value }}".into()),
        preset_modes: vec!["eco".into(), "away".into()],
        qos: Some(Qos::AtLeastOnce),
        retain: Some(false),
        swing_horizontal_mode_command_template: Some("{{ value }}".into()),
        swing_horizontal_mode_command_topic: Some("study/ac/swing/horizontal/mode/command".into()),
        swing_horizontal_mode_state_template: Some(
            "{{ value_json.swing_horizontal_mode_state }}".into(),
        ),
        swing_horizontal_mode_state_topic: Some("study/ac/swing/horizontal/mode/state".into()),
        swing_horizontal_modes: Some(vec!["on".into(), "off".into()]),
        swing_mode_command_template: Some("{{ value }}".into()),
        swing_mode_command_topic: Some("study/ac/swing/mode/command".into()),
        swing_mode_state_template: Some("{{ value_json.swing_mode_state }}".into()),
        swing_mode_state_topic: Some("study/ac/swing/mode/state".into()),
        swing_modes: Some(vec!["on".into(), "off".into()]),
        target_humidity_command_template: Some("{{ value }}".into()),
        target_humidity_command_topic: Some("study/ac/target/humidity/command".into()),
        target_humidity_state_template: Some("{{ value_json.target_humidity_state }}".into()),
        target_humidity_state_topic: Some("study/ac/target/humidity/state".into()),
        temp_step: Some(0.5),
        temperature_command_template: Some("{{ value }}".into()),
        temperature_command_topic: Some("study/ac/temperature/command".into()),
        temperature_high_command_template: Some("{{ value }}".into()),
        temperature_high_command_topic: Some("study/ac/temperature/high/command".into()),
        temperature_high_state_template: Some("{{ value_json.temperature_high_state }}".into()),
        temperature_high_state_topic: Some("study/ac/temperature/high/state".into()),
        temperature_low_command_template: Some("{{ value }}".into()),
        temperature_low_command_topic: Some("study/ac/temperature/low/command".into()),
        temperature_low_state_template: Some("{{ value_json.temperature_low_state }}".into()),
        temperature_low_state_topic: Some("study/ac/temperature/low/state".into()),
        temperature_state_template: Some("{{ value_json.temperature_state }}".into()),
        temperature_state_topic: Some("study/ac/temperature/state".into()),
        temperature_unit: Some(TemperatureUnit::Celsius),
        unique_id: Some("study_ac".into()),
        value_template: Some("{{ value_json.value }}".into()),
        visible_by_default: Some(true),
    }
}

#[test]
fn full_snapshot() {
    insta::assert_json_snapshot!(full());
}

/// The configuration example from
/// <https://www.home-assistant.io/integrations/climate.mqtt/>.
#[test]
fn minimal_snapshot() {
    insta::assert_json_snapshot!(Climate {
        mode_command_topic: Some("study/ac/mode/set".into()),
        name: Some("Study".into()),
        ..Default::default()
    });
}

#[test]
fn round_trip() {
    support::assert_round_trip(&full());
}

/// "Using templates" example from <https://www.home-assistant.io/integrations/climate.mqtt/>, as JSON.
#[test]
fn docs_templates_example() {
    let value: Climate = support::from_fixture("climate_study.json");
    assert_eq!(
        value,
        Climate {
            mode_command_topic: Some("study/ac/mode/set".into()),
            mode_state_template: Some("{{ value_json }}".into()),
            mode_state_topic: Some("study/ac/mode/state".into()),
            modes: Some(vec![HvacMode::Off, HvacMode::Heat, HvacMode::Auto]),
            name: Some("Study".into()),
            ..Default::default()
        }
    );
}

#[test]
fn component() {
    let component = Component::from(full());
    assert_eq!(component.platform(), "climate");
    #[cfg(feature = "json")]
    {
        let json = serde_json::to_value(&component).unwrap();
        assert_eq!(json["platform"], "climate");
        let back: Component = serde_json::from_value(json).unwrap();
        assert_eq!(back, component);
    }
}

#[cfg(feature = "builder")]
#[test]
fn builder_has_no_required_fields() {
    assert!(Climate::builder().build().is_ok());
}

#[test]
fn modes_accept_single_string() {
    let climate: Climate = serde_json::from_str(r#"{"modes": "heat"}"#).unwrap();
    assert_eq!(climate.modes, Some(vec![HvacMode::Heat]));
}

#[cfg(feature = "builder")]
#[test]
fn builder_checks_related_topics() {
    assert!(Climate::builder().build().is_ok());
    assert!(
        Climate::builder()
            .preset_mode_command_topic("p")
            .build()
            .is_err()
    );
    assert!(
        Climate::builder()
            .target_humidity_state_topic("s")
            .build()
            .is_err()
    );
    assert!(
        Climate::builder()
            .target_humidity_state_topic("s")
            .target_humidity_command_topic("c")
            .build()
            .is_ok()
    );
}
