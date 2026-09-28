mod support;

use hassliebe::{
    Availability, AvailabilityMode, Device, EntityCategory, Origin, Qos,
    light::{Light, LightSchema, OnCommandType},
};
use pretty_assertions::assert_eq;

fn full() -> Light {
    Light {
        availability: vec![Availability {
            topic: "office/light/status".into(),
            ..Default::default()
        }],
        availability_mode: Some(AvailabilityMode::Latest),
        brightness_command_template: Some("{{ value }}".into()),
        brightness_command_topic: Some("office/light/brightness/command".into()),
        brightness_scale: Some(100),
        brightness_state_topic: Some("office/light/brightness/state".into()),
        brightness_value_template: Some("{{ value_json.brightness_value }}".into()),
        color_mode_state_topic: Some("office/light/color/mode/state".into()),
        color_mode_value_template: Some("{{ value_json.color_mode_value }}".into()),
        color_temp_command_template: Some("{{ value }}".into()),
        color_temp_command_topic: Some("office/light/color/temp/command".into()),
        color_temp_kelvin: Some(true),
        color_temp_state_topic: Some("office/light/color/temp/state".into()),
        color_temp_value_template: Some("{{ value_json.color_temp_value }}".into()),
        command_topic: "office/light/switch".into(),
        default_entity_id: Some("light.office".into()),
        device: Some(Device {
            identifiers: vec!["office_light".into()],
            name: Some("Office light".into()),
            ..Default::default()
        }),
        effect_command_template: Some("{{ value }}".into()),
        effect_command_topic: Some("office/light/effect/command".into()),
        effect_list: vec!["rainbow".into(), "colorloop".into()],
        effect_state_topic: Some("office/light/effect/state".into()),
        effect_value_template: Some("{{ value_json.effect_value }}".into()),
        enabled_by_default: Some(true),
        encoding: Some("utf-8".into()),
        entity_category: Some(EntityCategory::Config),
        entity_picture: Some("https://example.com/light.png".into()),
        group: vec!["office_lamp".into()],
        hs_command_template: Some("{{ value }}".into()),
        hs_command_topic: Some("office/light/hs/command".into()),
        hs_state_topic: Some("office/light/hs/state".into()),
        hs_value_template: Some("{{ value_json.hs_value }}".into()),
        icon: Some("mdi:ceiling-light".into()),
        json_attributes_template: Some("{{ value_json.json_attributes }}".into()),
        json_attributes_topic: Some("office/light/json/attributes".into()),
        max_kelvin: Some(6500),
        max_mireds: Some(370),
        message_expiry_interval: Some(60.into()),
        min_kelvin: Some(2700),
        min_mireds: Some(153),
        name: Some("Office".into()),
        on_command_type: Some(OnCommandType::Brightness),
        optimistic: Some(false),
        origin: Some(Origin {
            name: "light2mqtt".into(),
            ..Default::default()
        }),
        payload_off: Some("off".into()),
        payload_on: Some("on".into()),
        qos: Some(Qos::AtMostOnce),
        retain: Some(false),
        rgb_command_template: Some("{{ value }}".into()),
        rgb_command_topic: Some("office/light/rgb/command".into()),
        rgb_state_topic: Some("office/light/rgb/state".into()),
        rgb_value_template: Some("{{ value_json.rgb_value }}".into()),
        rgbw_command_template: Some("{{ value }}".into()),
        rgbw_command_topic: Some("office/light/rgbw/command".into()),
        rgbw_state_topic: Some("office/light/rgbw/state".into()),
        rgbw_value_template: Some("{{ value_json.rgbw_value }}".into()),
        rgbww_command_template: Some("{{ value }}".into()),
        rgbww_command_topic: Some("office/light/rgbww/command".into()),
        rgbww_state_topic: Some("office/light/rgbww/state".into()),
        rgbww_value_template: Some("{{ value_json.rgbww_value }}".into()),
        schema: Some(LightSchema::Basic),
        state_topic: Some("office/light/state".into()),
        state_value_template: Some("{{ value_json.state_value }}".into()),
        unique_id: Some("office_light".into()),
        visible_by_default: Some(true),
        white_command_topic: Some("office/light/white/command".into()),
        white_scale: Some(100),
        xy_command_template: Some("{{ value }}".into()),
        xy_command_topic: Some("office/light/xy/command".into()),
        xy_state_topic: Some("office/light/xy/state".into()),
        xy_value_template: Some("{{ value_json.xy_value }}".into()),
    }
}

#[test]
fn full_snapshot() {
    insta::assert_json_snapshot!(full());
}

#[test]
fn minimal_snapshot() {
    insta::assert_json_snapshot!(Light {
        command_topic: "office/light/switch".into(),
        ..Default::default()
    });
}

#[test]
fn round_trip() {
    support::assert_round_trip(&full());
}

/// "Brightness and RGB support" example from
/// <https://www.home-assistant.io/integrations/light.mqtt/>, as JSON.
#[test]
fn docs_brightness_rgb_example() {
    let light: Light = support::from_fixture("light_office_rgb.json");
    assert_eq!(
        light,
        Light {
            brightness_command_topic: Some("office/rgb1/brightness/set".into()),
            brightness_state_topic: Some("office/rgb1/brightness/status".into()),
            brightness_value_template: Some("{{ value_json.brightness }}".into()),
            command_topic: "office/rgb1/light/switch".into(),
            name: Some("Office Light RGB".into()),
            optimistic: Some(false),
            payload_off: Some("OFF".into()),
            payload_on: Some("ON".into()),
            qos: Some(Qos::AtMostOnce),
            rgb_command_topic: Some("office/rgb1/rgb/set".into()),
            rgb_state_topic: Some("office/rgb1/rgb/status".into()),
            rgb_value_template: Some("{{ value_json.rgb | join(',') }}".into()),
            state_topic: Some("office/rgb1/light/status".into()),
            state_value_template: Some("{{ value_json.state }}".into()),
            ..Default::default()
        }
    );
}

/// Decision 7 in `docs/plan.md`: only the basic schema is modeled.
#[test]
fn other_schemas_are_rejected() {
    let basic = r#"{"command_topic": "t", "schema": "basic"}"#;
    assert_eq!(
        serde_json::from_str::<Light>(basic).unwrap().schema,
        Some(LightSchema::Basic)
    );
    for schema in ["json", "template"] {
        let json = format!(r#"{{"command_topic": "t", "schema": "{schema}"}}"#);
        assert!(serde_json::from_str::<Light>(&json).is_err(), "{schema}");
    }
}

#[test]
fn effect_list_accepts_single_string() {
    let light: Light =
        serde_json::from_str(r#"{"command_topic": "t", "effect_list": "rainbow"}"#).unwrap();
    assert_eq!(light.effect_list, ["rainbow"]);
}

#[cfg(feature = "builder")]
#[test]
fn builder_requires_command_topic() {
    assert!(Light::builder().build().is_err());
    assert!(Light::builder().command_topic("t").build().is_ok());
}
