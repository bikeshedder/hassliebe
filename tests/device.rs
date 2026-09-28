mod support;

use hassliebe::Device;

/// Device block of the "Sensors" example in the MQTT docs
/// (<https://www.home-assistant.io/integrations/mqtt/>).
#[test]
fn device_snapshot() {
    let device = Device {
        configuration_url: Some("https://example.com/sensor_portal/config".into()),
        hw_version: Some("1.01a".into()),
        identifiers: vec!["bedroom01ae".into()],
        manufacturer: Some("Example sensors Ltd.".into()),
        model: Some("Example Sensor".into()),
        model_id: Some("K9".into()),
        name: Some("Bedroom".into()),
        serial_number: Some("12AE3010545".into()),
        sw_version: Some("2024.1.0".into()),
        ..Default::default()
    };
    insta::assert_json_snapshot!(device);
}

#[test]
fn device_round_trip() {
    support::assert_round_trip(&Device {
        connections: vec![("mac".into(), "02:5b:26:a8:dc:12".into())],
        identifiers: vec!["bedroom01ae".into()],
        name: Some("Bedroom".into()),
        suggested_area: Some("Bedroom".into()),
        via_device: Some("bridge01".into()),
        ..Default::default()
    });
}
