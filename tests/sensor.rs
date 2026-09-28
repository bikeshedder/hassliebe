mod support;

use hassliebe::{
    Availability, AvailabilityMode, Device, EntityCategory, EntityName, Origin, Qos, StateClass,
    sensor::{Sensor, SensorDeviceClass},
    unit::UnitOfPower,
};
use pretty_assertions::assert_eq;

fn full() -> Sensor {
    Sensor {
        availability: vec![Availability {
            topic: "wallbox/garage/status".into(),
            ..Default::default()
        }],
        availability_mode: Some(AvailabilityMode::Latest),
        default_entity_id: Some("sensor.wallbox_garage_power".into()),
        device: Some(Device {
            identifiers: vec!["wallbox_garage".into()],
            manufacturer: Some("Example Chargers".into()),
            model: Some("WB-11".into()),
            name: Some("Wallbox Garage".into()),
            sw_version: Some("1.4.2".into()),
            ..Default::default()
        }),
        device_class: Some(SensorDeviceClass::Power),
        enabled_by_default: Some(true),
        encoding: Some("utf-8".into()),
        entity_category: Some(EntityCategory::Diagnostic),
        entity_picture: Some("https://example.com/wallbox.png".into()),
        expire_after: Some(300),
        force_update: Some(false),
        group: vec![],
        icon: Some("mdi:ev-station".into()),
        json_attributes_template: Some("{{ value_json.attributes | tojson }}".into()),
        json_attributes_topic: Some("wallbox/garage/attributes".into()),
        last_reset_value_template: None,
        message_expiry_interval: Some(3600.into()),
        name: Some("Power".into()),
        options: None,
        origin: Some(Origin {
            name: "wallbox2mqtt".into(),
            sw_version: Some("0.3.0".into()),
            ..Default::default()
        }),
        qos: Some(Qos::AtLeastOnce),
        state_class: Some(StateClass::Measurement),
        state_topic: "wallbox/garage/power".into(),
        suggested_display_precision: Some(1),
        unique_id: Some("wallbox_garage_power".into()),
        unit_of_measurement: Some(UnitOfPower::Watt.into()),
        value_template: Some("{{ value_json.power }}".into()),
        visible_by_default: Some(true),
    }
}

#[test]
fn full_snapshot() {
    insta::assert_json_snapshot!(full());
}

#[test]
fn minimal_snapshot() {
    insta::assert_json_snapshot!(Sensor {
        state_topic: "home/kitchen/temperature".into(),
        ..Default::default()
    });
}

#[test]
fn round_trip() {
    support::assert_round_trip(&full());
    support::assert_round_trip(&Sensor {
        name: Some(EntityName::Null),
        state_topic: "home/kitchen/temperature".into(),
        ..Default::default()
    });
}

/// "Processing Unix EPOCH timestamps" example from
/// <https://www.home-assistant.io/integrations/sensor.mqtt/>.
#[test]
fn docs_timestamp_example() {
    let sensor: Sensor = support::from_fixture("sensor_timestamp.json");
    assert_eq!(
        sensor,
        Sensor {
            device: Some(Device {
                identifiers: vec!["hp_1231232".into()],
                name: Some("Heat pump".into()),
                ..Default::default()
            }),
            device_class: Some(SensorDeviceClass::Timestamp),
            name: Some("turned on".into()),
            state_topic: "pump/timestamp_on".into(),
            unique_id: Some("hp_1231232_ts_on".into()),
            value_template: Some("{{ as_datetime(value) }}".into()),
            ..Default::default()
        }
    );
}

#[test]
fn null_name_and_unknown_values() {
    let sensor: Sensor = serde_json::from_str(
        r#"{
            "name": null,
            "state_topic": "t",
            "device_class": "future_class",
            "unit_of_measurement": "µg/m³",
            "some_future_key": 1
        }"#,
    )
    .unwrap();
    assert_eq!(sensor.name, Some(EntityName::Null));
    assert_eq!(
        sensor.device_class,
        Some(SensorDeviceClass::Other("future_class".into()))
    );
    assert_eq!(sensor.unit_of_measurement.unwrap().as_str(), "μg/m³");
}

#[test]
fn state_topic_is_required() {
    assert!(serde_json::from_str::<Sensor>("{}").is_err());
}

#[cfg(feature = "builder")]
#[test]
fn builder() {
    assert!(Sensor::builder().name("Power").build().is_err());
    let sensor = Sensor::builder()
        .name("Power")
        .state_topic("wallbox/garage/power")
        .unit_of_measurement(UnitOfPower::Watt)
        .build()
        .unwrap();
    assert_eq!(sensor.name, Some("Power".into()));
    assert_eq!(sensor.unit_of_measurement, Some(UnitOfPower::Watt.into()));
}
