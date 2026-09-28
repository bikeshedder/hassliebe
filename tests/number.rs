mod support;

use hassliebe::{
    Availability, AvailabilityMode, Device, EntityCategory, Origin, Qos,
    number::{Number, NumberDeviceClass, NumberMode},
    unit::UnitOfTemperature,
};
use pretty_assertions::assert_eq;

fn full() -> Number {
    Number {
        availability: vec![Availability {
            topic: "heating/status".into(),
            ..Default::default()
        }],
        availability_mode: Some(AvailabilityMode::Latest),
        command_template: Some("{{ value | round(1) }}".into()),
        command_topic: "heating/setpoint/set".into(),
        default_entity_id: Some("number.heating_setpoint".into()),
        device: Some(Device {
            identifiers: vec!["heating01".into()],
            name: Some("Heating".into()),
            ..Default::default()
        }),
        device_class: Some(NumberDeviceClass::Temperature),
        enabled_by_default: Some(true),
        encoding: Some("utf-8".into()),
        entity_category: Some(EntityCategory::Config),
        entity_picture: Some("https://example.com/heating.png".into()),
        group: vec!["heating_boost".into()],
        icon: Some("mdi:thermostat".into()),
        json_attributes_template: Some("{{ value_json | tojson }}".into()),
        json_attributes_topic: Some("heating/attributes".into()),
        max: Some(28.0),
        message_expiry_interval: Some(600.into()),
        min: Some(5.0),
        mode: Some(NumberMode::Slider),
        name: Some("Setpoint".into()),
        optimistic: Some(false),
        origin: Some(Origin {
            name: "heating2mqtt".into(),
            ..Default::default()
        }),
        payload_reset: Some("None".into()),
        qos: Some(Qos::AtLeastOnce),
        retain: Some(false),
        state_topic: Some("heating/setpoint".into()),
        step: Some(0.5),
        unique_id: Some("heating_setpoint".into()),
        unit_of_measurement: Some(UnitOfTemperature::Celsius.into()),
        value_template: Some("{{ value_json.setpoint }}".into()),
        visible_by_default: Some(true),
    }
}

#[test]
fn full_snapshot() {
    insta::assert_json_snapshot!(full());
}

/// The configuration example from
/// <https://www.home-assistant.io/integrations/number.mqtt/>.
#[test]
fn minimal_snapshot() {
    insta::assert_json_snapshot!(Number {
        command_topic: "my-device/threshold".into(),
        ..Default::default()
    });
}

#[test]
fn round_trip() {
    support::assert_round_trip(&full());
}

/// `data1` of `test_discovery_update_unchanged_number` in Home Assistant's
/// `tests/components/mqtt/test_number.py`.
#[test]
fn home_assistant_test_payload() {
    let number: Number = support::from_fixture("number_beer.json");
    assert_eq!(
        number,
        Number {
            command_topic: "test-topic".into(),
            name: Some("Beer".into()),
            state_topic: Some("test-topic".into()),
            ..Default::default()
        }
    );
}

#[test]
fn integer_bounds() {
    let number: Number =
        serde_json::from_str(r#"{"command_topic": "t", "min": 0, "max": 10, "step": 1}"#).unwrap();
    assert_eq!(
        (number.min, number.max, number.step),
        (Some(0.0), Some(10.0), Some(1.0))
    );
}

#[cfg(feature = "builder")]
#[test]
fn builder_requires_command_topic() {
    assert!(Number::builder().build().is_err());
    assert!(
        Number::builder()
            .command_topic("t")
            .min(0.0)
            .build()
            .is_ok()
    );
}
