mod support;

use hassliebe::Device;

/// Device block of the "Sensors" example in the MQTT docs
/// (<https://www.home-assistant.io/integrations/mqtt/>).
#[test]
fn device_snapshot() {
    let device = Device {
        configuration_url: Some("https://example.com/sensor_portal/config".into()),
        hw_version: Some("1.01a".into()),
        identifiers: Some(vec!["bedroom01ae".into()]),
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
