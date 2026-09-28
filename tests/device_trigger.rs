mod support;

use hassliebe::{
    Component, Device, Qos,
    device_trigger::{DeviceAutomationType, DeviceTrigger},
};
use pretty_assertions::assert_eq;

fn full() -> DeviceTrigger {
    DeviceTrigger {
        automation_type: DeviceAutomationType::Trigger,
        device: Some(Device {
            identifiers: vec!["zigbee2mqtt_0x90fd9ffffedf1266".into()],
            manufacturer: Some("IKEA".into()),
            model: Some("TRADFRI remote control (E1524/E1810)".into()),
            name: Some("0x90fd9ffffedf1266".into()),
            sw_version: Some("Zigbee2MQTT 1.14.0".into()),
            ..Default::default()
        }),
        encoding: Some("utf-8".into()),
        group: vec![],
        payload: Some("arrow_left_click".into()),
        qos: Some(Qos::AtLeastOnce),
        r#type: "action".into(),
        subtype: "arrow_left_click".into(),
        topic: "zigbee2mqtt/0x90fd9ffffedf1266/action".into(),
        value_template: Some("{{ value_json.action }}".into()),
    }
}

#[test]
fn full_snapshot() {
    insta::assert_json_snapshot!(full());
}

#[test]
fn minimal_snapshot() {
    insta::assert_json_snapshot!(DeviceTrigger {
        device: Some(Device {
            identifiers: vec!["zigbee2mqtt_0x90fd9ffffedf1266".into()],
            ..Default::default()
        }),
        r#type: "action".into(),
        subtype: "arrow_right_click".into(),
        topic: "zigbee2mqtt/0x90fd9ffffedf1266/action".into(),
        ..Default::default()
    });
}

#[test]
fn round_trip() {
    support::assert_round_trip(&full());
}

/// "Left arrow click configuration" (Zigbee2MQTT) from
/// <https://www.home-assistant.io/integrations/device_trigger.mqtt/>.
#[test]
fn docs_zigbee2mqtt_example() {
    let value: DeviceTrigger = support::from_fixture("device_trigger_zigbee2mqtt.json");
    assert_eq!(
        value,
        DeviceTrigger {
            device: Some(Device {
                identifiers: vec!["zigbee2mqtt_0x90fd9ffffedf1266".into()],
                manufacturer: Some("IKEA".into()),
                model: Some("TRADFRI remote control (E1524/E1810)".into()),
                name: Some("0x90fd9ffffedf1266".into()),
                sw_version: Some("Zigbee2MQTT 1.14.0".into()),
                ..Default::default()
            }),
            payload: Some("arrow_left_click".into()),
            r#type: "action".into(),
            subtype: "arrow_left_click".into(),
            topic: "zigbee2mqtt/0x90fd9ffffedf1266/action".into(),
            ..Default::default()
        }
    );
}

#[test]
fn component() {
    let component = Component::from(full());
    assert_eq!(component.platform(), "device_automation");
    #[cfg(feature = "json")]
    {
        let json = serde_json::to_value(&component).unwrap();
        assert_eq!(json["platform"], "device_automation");
        let back: Component = serde_json::from_value(json).unwrap();
        assert_eq!(back, component);
    }
}

#[cfg(feature = "builder")]
#[test]
fn builder_requires_fields() {
    assert!(DeviceTrigger::builder().build().is_err());
    assert!(
        DeviceTrigger::builder()
            .r#type("action")
            .subtype("button_1")
            .topic("t")
            .device(Device {
                identifiers: vec!["d".into()],
                ..Default::default()
            })
            .build()
            .is_ok()
    );
}

#[test]
fn automation_type_must_be_trigger() {
    let json = r#"{"automation_type": "action", "type": "a", "subtype": "b", "topic": "t"}"#;
    assert!(serde_json::from_str::<DeviceTrigger>(json).is_err());
}

#[cfg(feature = "builder")]
#[test]
fn builder_requires_device() {
    assert!(
        DeviceTrigger::builder()
            .r#type("action")
            .subtype("button_1")
            .topic("t")
            .build()
            .is_err()
    );
}
